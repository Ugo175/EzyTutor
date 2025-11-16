use crate::database::DbPool;
use crate::errors::{AppError, AppResult};
use crate::models::{
    TutorProfile, CreateTutorProfileRequest, UpdateTutorProfileRequest, TutorResponse,
    TutorReview, CreateReviewRequest, ReviewResponse, UserRole
};
use chrono::Utc;
use sqlx::Row;
use uuid::Uuid;
use validator::Validate;

pub struct TutorService;

impl TutorService {
    pub async fn create_tutor_profile(
        pool: &DbPool,
        user_id: Uuid,
        request: CreateTutorProfileRequest,
    ) -> AppResult<TutorResponse> {
        // Validate input
        request.validate()
            .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;

        // Check if user exists and is a tutor
        let user = sqlx::query(
            "SELECT first_name, last_name, email, role FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let role: UserRole = user.get("role");
        if role != UserRole::Tutor && role != UserRole::Admin {
            return Err(AppError::Authorization("Only tutors can create tutor profiles".to_string()));
        }

        // Check if tutor profile already exists
        let existing_profile = sqlx::query("SELECT id FROM tutors WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

        if existing_profile.is_some() {
            return Err(AppError::BadRequest("Tutor profile already exists".to_string()));
        }

        // Insert tutor profile
        let tutor_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO tutors (id, user_id, bio, specializations, hourly_rate, years_experience, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(tutor_id)
        .bind(user_id)
        .bind(&request.bio)
        .bind(&request.specializations)
        .bind(request.hourly_rate)
        .bind(request.years_experience)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;

        // Return tutor response
        Ok(TutorResponse {
            id: tutor_id,
            user_id,
            first_name: user.get("first_name"),
            last_name: user.get("last_name"),
            email: user.get("email"),
            bio: request.bio,
            specializations: request.specializations,
            hourly_rate: request.hourly_rate,
            years_experience: request.years_experience,
            rating: None,
            total_reviews: 0,
            is_verified: false,
            is_available: true,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_all_tutors(pool: &DbPool) -> AppResult<Vec<TutorResponse>> {
        let tutors = sqlx::query(
            r#"
            SELECT t.*, u.first_name, u.last_name, u.email
            FROM tutors t
            JOIN users u ON t.user_id = u.id
            WHERE t.is_available = true
            ORDER BY t.rating DESC NULLS LAST, t.created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        let mut tutor_responses = Vec::new();
        for row in tutors {
            tutor_responses.push(TutorResponse {
                id: row.get("id"),
                user_id: row.get("user_id"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                email: row.get("email"),
                bio: row.get("bio"),
                specializations: row.get("specializations"),
                hourly_rate: row.get("hourly_rate"),
                years_experience: row.get("years_experience"),
                rating: row.get("rating"),
                total_reviews: row.get("total_reviews"),
                is_verified: row.get("is_verified"),
                is_available: row.get("is_available"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(tutor_responses)
    }

    pub async fn get_tutor_by_id(pool: &DbPool, tutor_id: Uuid) -> AppResult<TutorResponse> {
        let tutor = sqlx::query(
            r#"
            SELECT t.*, u.first_name, u.last_name, u.email
            FROM tutors t
            JOIN users u ON t.user_id = u.id
            WHERE t.id = $1
            "#
        )
        .bind(tutor_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Tutor not found".to_string()))?;

        Ok(TutorResponse {
            id: tutor.get("id"),
            user_id: tutor.get("user_id"),
            first_name: tutor.get("first_name"),
            last_name: tutor.get("last_name"),
            email: tutor.get("email"),
            bio: tutor.get("bio"),
            specializations: tutor.get("specializations"),
            hourly_rate: tutor.get("hourly_rate"),
            years_experience: tutor.get("years_experience"),
            rating: tutor.get("rating"),
            total_reviews: tutor.get("total_reviews"),
            is_verified: tutor.get("is_verified"),
            is_available: tutor.get("is_available"),
            created_at: tutor.get("created_at"),
            updated_at: tutor.get("updated_at"),
        })
    }

    pub async fn update_tutor_profile(
        pool: &DbPool,
        user_id: Uuid,
        request: UpdateTutorProfileRequest,
    ) -> AppResult<TutorResponse> {
        // Validate input
        request.validate()
            .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;

        // Check if tutor profile exists
        let existing_tutor = sqlx::query("SELECT id FROM tutors WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Tutor profile not found".to_string()))?;

        let tutor_id: Uuid = existing_tutor.get("id");

        // Build update query dynamically
        let mut query_parts = Vec::new();
        let mut bind_count = 1;

        if request.bio.is_some() {
            query_parts.push(format!("bio = ${}", bind_count));
            bind_count += 1;
        }
        if request.specializations.is_some() {
            query_parts.push(format!("specializations = ${}", bind_count));
            bind_count += 1;
        }
        if request.hourly_rate.is_some() {
            query_parts.push(format!("hourly_rate = ${}", bind_count));
            bind_count += 1;
        }
        if request.years_experience.is_some() {
            query_parts.push(format!("years_experience = ${}", bind_count));
            bind_count += 1;
        }
        if request.is_available.is_some() {
            query_parts.push(format!("is_available = ${}", bind_count));
            bind_count += 1;
        }

        if query_parts.is_empty() {
            return Err(AppError::BadRequest("No fields to update".to_string()));
        }

        query_parts.push(format!("updated_at = ${}", bind_count));
        let query = format!(
            "UPDATE tutors SET {} WHERE user_id = ${}",
            query_parts.join(", "),
            bind_count + 1
        );

        let mut query_builder = sqlx::query(&query);

        // Bind values in the same order as the query parts
        if let Some(bio) = &request.bio {
            query_builder = query_builder.bind(bio);
        }
        if let Some(specializations) = &request.specializations {
            query_builder = query_builder.bind(specializations);
        }
        if let Some(hourly_rate) = request.hourly_rate {
            query_builder = query_builder.bind(hourly_rate);
        }
        if let Some(years_experience) = request.years_experience {
            query_builder = query_builder.bind(years_experience);
        }
        if let Some(is_available) = request.is_available {
            query_builder = query_builder.bind(is_available);
        }

        query_builder = query_builder.bind(Utc::now()).bind(user_id);

        query_builder.execute(pool).await?;

        // Return updated tutor
        Self::get_tutor_by_id(pool, tutor_id).await
    }

    pub async fn create_review(
        pool: &DbPool,
        tutor_id: Uuid,
        student_id: Uuid,
        request: CreateReviewRequest,
    ) -> AppResult<ReviewResponse> {
        // Validate input
        request.validate()
            .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;

        // Check if tutor exists
        let tutor_exists = sqlx::query("SELECT id FROM tutors WHERE id = $1")
            .bind(tutor_id)
            .fetch_optional(pool)
            .await?;

        if tutor_exists.is_none() {
            return Err(AppError::NotFound("Tutor not found".to_string()));
        }

        // Check if review already exists
        let existing_review = sqlx::query(
            "SELECT id FROM tutor_reviews WHERE tutor_id = $1 AND student_id = $2"
        )
        .bind(tutor_id)
        .bind(student_id)
        .fetch_optional(pool)
        .await?;

        if existing_review.is_some() {
            return Err(AppError::BadRequest("Review already exists for this tutor".to_string()));
        }

        // Insert review
        let review_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO tutor_reviews (id, tutor_id, student_id, rating, comment, created_at) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(review_id)
        .bind(tutor_id)
        .bind(student_id)
        .bind(request.rating)
        .bind(&request.comment)
        .bind(now)
        .execute(pool)
        .await?;

        // Update tutor rating and review count
        Self::update_tutor_rating(pool, tutor_id).await?;

        // Get student name for response
        let student = sqlx::query("SELECT first_name, last_name FROM users WHERE id = $1")
            .bind(student_id)
            .fetch_one(pool)
            .await?;

        let student_name = format!("{} {}", 
            student.get::<String, _>("first_name"),
            student.get::<String, _>("last_name")
        );

        Ok(ReviewResponse {
            id: review_id,
            tutor_id,
            student_name,
            rating: request.rating,
            comment: request.comment,
            created_at: now,
        })
    }

    pub async fn get_tutor_reviews(pool: &DbPool, tutor_id: Uuid) -> AppResult<Vec<ReviewResponse>> {
        let reviews = sqlx::query(
            r#"
            SELECT r.id, r.tutor_id, r.rating, r.comment, r.created_at,
                   u.first_name, u.last_name
            FROM tutor_reviews r
            JOIN users u ON r.student_id = u.id
            WHERE r.tutor_id = $1
            ORDER BY r.created_at DESC
            "#
        )
        .bind(tutor_id)
        .fetch_all(pool)
        .await?;

        let mut review_responses = Vec::new();
        for row in reviews {
            let student_name = format!("{} {}", 
                row.get::<String, _>("first_name"),
                row.get::<String, _>("last_name")
            );

            review_responses.push(ReviewResponse {
                id: row.get("id"),
                tutor_id: row.get("tutor_id"),
                student_name,
                rating: row.get("rating"),
                comment: row.get("comment"),
                created_at: row.get("created_at"),
            });
        }

        Ok(review_responses)
    }

    async fn update_tutor_rating(pool: &DbPool, tutor_id: Uuid) -> AppResult<()> {
        // Calculate new rating and review count
        let stats = sqlx::query(
            "SELECT AVG(rating) as avg_rating, COUNT(*) as total_reviews FROM tutor_reviews WHERE tutor_id = $1"
        )
        .bind(tutor_id)
        .fetch_one(pool)
        .await?;

        let avg_rating: Option<f64> = stats.get("avg_rating");
        let total_reviews: i64 = stats.get("total_reviews");

        // Update tutor record
        sqlx::query(
            "UPDATE tutors SET rating = $1, total_reviews = $2, updated_at = $3 WHERE id = $4"
        )
        .bind(avg_rating.map(|r| r as f32))
        .bind(total_reviews as i32)
        .bind(Utc::now())
        .bind(tutor_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn search_tutors_by_specialization(
        pool: &DbPool,
        specialization: &str,
    ) -> AppResult<Vec<TutorResponse>> {
        let tutors = sqlx::query(
            r#"
            SELECT t.*, u.first_name, u.last_name, u.email
            FROM tutors t
            JOIN users u ON t.user_id = u.id
            WHERE t.is_available = true AND $1 = ANY(t.specializations)
            ORDER BY t.rating DESC NULLS LAST, t.created_at DESC
            "#
        )
        .bind(specialization)
        .fetch_all(pool)
        .await?;

        let mut tutor_responses = Vec::new();
        for row in tutors {
            tutor_responses.push(TutorResponse {
                id: row.get("id"),
                user_id: row.get("user_id"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                email: row.get("email"),
                bio: row.get("bio"),
                specializations: row.get("specializations"),
                hourly_rate: row.get("hourly_rate"),
                years_experience: row.get("years_experience"),
                rating: row.get("rating"),
                total_reviews: row.get("total_reviews"),
                is_verified: row.get("is_verified"),
                is_available: row.get("is_available"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(tutor_responses)
    }
}
