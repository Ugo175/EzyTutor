use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

// User Models
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Student,
    Tutor,
    Admin,
}

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    pub role: UserRole,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

// Course Models
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "difficulty_level", rename_all = "lowercase")]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct Course {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub tutor_id: Uuid,
    pub price: i32, // Price in cents
    pub duration_minutes: i32,
    pub category: String,
    pub difficulty_level: DifficultyLevel,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCourseRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(min = 1, max = 1000))]
    pub description: String,
    #[validate(range(min = 0))]
    pub price: i32,
    #[validate(range(min = 15, max = 480))]
    pub duration_minutes: i32,
    #[validate(length(min = 1))]
    pub category: String,
    pub difficulty_level: DifficultyLevel,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCourseRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 1000))]
    pub description: Option<String>,
    #[validate(range(min = 0))]
    pub price: Option<i32>,
    #[validate(range(min = 15, max = 480))]
    pub duration_minutes: Option<i32>,
    #[validate(length(min = 1))]
    pub category: Option<String>,
    pub difficulty_level: Option<DifficultyLevel>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct CourseResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub tutor_id: Uuid,
    pub tutor_name: Option<String>,
    pub price: i32,
    pub duration_minutes: i32,
    pub category: String,
    pub difficulty_level: DifficultyLevel,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Tutor Models
#[derive(Debug, FromRow, Serialize, Clone)]
pub struct TutorProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub bio: String,
    pub specializations: Vec<String>,
    pub hourly_rate: i32, // Rate in cents
    pub years_experience: i32,
    pub rating: Option<f32>,
    pub total_reviews: i32,
    pub is_verified: bool,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTutorProfileRequest {
    #[validate(length(min = 50, max = 1000))]
    pub bio: String,
    pub specializations: Vec<String>,
    #[validate(range(min = 1000, max = 50000))]
    pub hourly_rate: i32,
    #[validate(range(min = 0, max = 50))]
    pub years_experience: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTutorProfileRequest {
    #[validate(length(min = 50, max = 1000))]
    pub bio: Option<String>,
    pub specializations: Option<Vec<String>>,
    #[validate(range(min = 1000, max = 50000))]
    pub hourly_rate: Option<i32>,
    #[validate(range(min = 0, max = 50))]
    pub years_experience: Option<i32>,
    pub is_available: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct TutorResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub bio: String,
    pub specializations: Vec<String>,
    pub hourly_rate: i32,
    pub years_experience: i32,
    pub rating: Option<f32>,
    pub total_reviews: i32,
    pub is_verified: bool,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Review Models
#[derive(Debug, FromRow, Serialize, Clone)]
pub struct TutorReview {
    pub id: Uuid,
    pub tutor_id: Uuid,
    pub student_id: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReviewRequest {
    #[validate(range(min = 1, max = 5))]
    pub rating: i32,
    #[validate(length(max = 500))]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReviewResponse {
    pub id: Uuid,
    pub tutor_id: Uuid,
    pub student_name: String,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Conversion implementations
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at,
        }
    }
}

impl From<Course> for CourseResponse {
    fn from(course: Course) -> Self {
        CourseResponse {
            id: course.id,
            title: course.title,
            description: course.description,
            tutor_id: course.tutor_id,
            tutor_name: None, // Will be populated by service layer
            price: course.price,
            duration_minutes: course.duration_minutes,
            category: course.category,
            difficulty_level: course.difficulty_level,
            is_active: course.is_active,
            created_at: course.created_at,
            updated_at: course.updated_at,
        }
    }
}
