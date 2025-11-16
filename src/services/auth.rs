use crate::database::DbPool;
use crate::errors::{AppError, AppResult};
use crate::models::{User, CreateUserRequest, LoginRequest, LoginResponse, UserResponse, UserRole};
use crate::middleware::Claims;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::Row;
use uuid::Uuid;
use validator::Validate;

pub struct AuthService;

impl AuthService {
    pub async fn register_user(
        pool: &DbPool,
        request: CreateUserRequest,
    ) -> AppResult<UserResponse> {
        // Validate input
        request.validate()
            .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;

        // Check if user already exists
        let existing_user = sqlx::query("SELECT id FROM users WHERE email = $1")
            .bind(&request.email)
            .fetch_optional(pool)
            .await?;

        if existing_user.is_some() {
            return Err(AppError::BadRequest("User with this email already exists".to_string()));
        }

        // Hash password
        let password_hash = hash(&request.password, DEFAULT_COST)
            .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))?;

        // Insert user
        let user_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, first_name, last_name, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(user_id)
        .bind(&request.email)
        .bind(&password_hash)
        .bind(&request.first_name)
        .bind(&request.last_name)
        .bind(&request.role)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;

        // Return user response
        Ok(UserResponse {
            id: user_id,
            email: request.email,
            first_name: request.first_name,
            last_name: request.last_name,
            role: request.role,
            is_active: true,
            created_at: now,
        })
    }

    pub async fn login_user(
        pool: &DbPool,
        request: LoginRequest,
    ) -> AppResult<LoginResponse> {
        // Validate input
        request.validate()
            .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;

        // Find user by email
        let user_row = sqlx::query(
            "SELECT id, email, password_hash, first_name, last_name, role, is_active, created_at FROM users WHERE email = $1"
        )
        .bind(&request.email)
        .fetch_optional(pool)
        .await?;

        let user_row = user_row.ok_or_else(|| {
            AppError::Authentication("Invalid email or password".to_string())
        })?;

        // Check if user is active
        let is_active: bool = user_row.get("is_active");
        if !is_active {
            return Err(AppError::Authentication("Account is deactivated".to_string()));
        }

        // Verify password
        let password_hash: String = user_row.get("password_hash");
        let password_valid = verify(&request.password, &password_hash)
            .map_err(|e| AppError::Internal(format!("Password verification failed: {}", e)))?;

        if !password_valid {
            return Err(AppError::Authentication("Invalid email or password".to_string()));
        }

        // Create user response
        let user = UserResponse {
            id: user_row.get("id"),
            email: user_row.get("email"),
            first_name: user_row.get("first_name"),
            last_name: user_row.get("last_name"),
            role: user_row.get("role"),
            is_active,
            created_at: user_row.get("created_at"),
        };

        // Generate JWT token
        let token = Self::generate_jwt_token(&user)?;

        Ok(LoginResponse { token, user })
    }

    pub async fn get_user_by_id(pool: &DbPool, user_id: Uuid) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, first_name, last_name, role, is_active, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user)
    }

    fn generate_jwt_token(user: &UserResponse) -> AppResult<String> {
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());

        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("Valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id,
            email: user.email.clone(),
            role: format!("{:?}", user.role).to_lowercase(),
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .map_err(|e| AppError::Internal(format!("Token generation failed: {}", e)))?;

        Ok(token)
    }

    pub async fn verify_user_role(
        pool: &DbPool,
        user_id: Uuid,
        required_role: UserRole,
    ) -> AppResult<bool> {
        let user = Self::get_user_by_id(pool, user_id).await?;
        
        // Admin can access everything
        if user.role == UserRole::Admin {
            return Ok(true);
        }

        // Check specific role
        Ok(user.role == required_role)
    }
}
