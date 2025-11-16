use crate::database::DbPool;
use crate::errors::{AppError, AppResult};
use crate::models::{Course, CreateCourseRequest, UpdateCourseRequest, CourseResponse};
use chrono::Utc;
use sqlx::Row;
use uuid::Uuid;
use validator::Validate;

pub struct CourseService;

impl CourseService {
    pub async fn create_course(
        pool: &DbPool,
        tutor_id: Uuid,
        request: CreateCourseRequest,
    ) -> AppResult<CourseResponse> {
        // Validate input
        request.validate()
            .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;

        // Verify tutor exists
        let tutor_exists = sqlx::query("SELECT id FROM tutors WHERE user_id = $1")
            .bind(tutor_id)
            .fetch_optional(pool)
            .await?;

        let tutor_db_id = tutor_exists
            .ok_or_else(|| AppError::NotFound("Tutor profile not found".to_string()))?
            .get::<Uuid, _>("id");

        // Insert course
        let course_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO courses (id, title, description, tutor_id, price, duration_minutes, category, difficulty_level, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#
        )
        .bind(course_id)
        .bind(&request.title)
        .bind(&request.description)
        .bind(tutor_db_id)
        .bind(request.price)
        .bind(request.duration_minutes)
        .bind(&request.category)
        .bind(&request.difficulty_level)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;

        // Return course response
        let mut course_response = CourseResponse {
            id: course_id,
            title: request.title,
            description: request.description,
            tutor_id: tutor_db_id,
            tutor_name: None,
            price: request.price,
            duration_minutes: request.duration_minutes,
            category: request.category,
            difficulty_level: request.difficulty_level,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        // Get tutor name
        if let Ok(tutor_name) = Self::get_tutor_name(pool, tutor_db_id).await {
            course_response.tutor_name = Some(tutor_name);
        }

        Ok(course_response)
    }

    pub async fn get_all_courses(pool: &DbPool) -> AppResult<Vec<CourseResponse>> {
        let courses = sqlx::query_as::<_, Course>(
            r#"
            SELECT c.id, c.title, c.description, c.tutor_id, c.price, c.duration_minutes, 
                   c.category, c.difficulty_level, c.is_active, c.created_at, c.updated_at
            FROM courses c
            WHERE c.is_active = true
            ORDER BY c.created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        let mut course_responses = Vec::new();
        for course in courses {
            let mut course_response = CourseResponse::from(course);
            
            // Get tutor name
            if let Ok(tutor_name) = Self::get_tutor_name(pool, course_response.tutor_id).await {
                course_response.tutor_name = Some(tutor_name);
            }
            
            course_responses.push(course_response);
        }

        Ok(course_responses)
    }

    pub async fn get_course_by_id(pool: &DbPool, course_id: Uuid) -> AppResult<CourseResponse> {
        let course = sqlx::query_as::<_, Course>(
            "SELECT id, title, description, tutor_id, price, duration_minutes, category, difficulty_level, is_active, created_at, updated_at FROM courses WHERE id = $1"
        )
        .bind(course_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Course not found".to_string()))?;

        let mut course_response = CourseResponse::from(course);
        
        // Get tutor name
        if let Ok(tutor_name) = Self::get_tutor_name(pool, course_response.tutor_id).await {
            course_response.tutor_name = Some(tutor_name);
        }

        Ok(course_response)
    }

    pub async fn update_course(
        pool: &DbPool,
        course_id: Uuid,
        tutor_id: Uuid,
        request: UpdateCourseRequest,
    ) -> AppResult<CourseResponse> {
        // Validate input
        request.validate()
            .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;

        // Check if course exists and belongs to tutor
        let existing_course = sqlx::query(
            "SELECT c.id FROM courses c JOIN tutors t ON c.tutor_id = t.id WHERE c.id = $1 AND t.user_id = $2"
        )
        .bind(course_id)
        .bind(tutor_id)
        .fetch_optional(pool)
        .await?;

        if existing_course.is_none() {
            return Err(AppError::NotFound("Course not found or access denied".to_string()));
        }

        // Build update query dynamically
        let mut query_parts = Vec::new();
        let mut bind_count = 1;

        if request.title.is_some() {
            query_parts.push(format!("title = ${}", bind_count));
            bind_count += 1;
        }
        if request.description.is_some() {
            query_parts.push(format!("description = ${}", bind_count));
            bind_count += 1;
        }
        if request.price.is_some() {
            query_parts.push(format!("price = ${}", bind_count));
            bind_count += 1;
        }
        if request.duration_minutes.is_some() {
            query_parts.push(format!("duration_minutes = ${}", bind_count));
            bind_count += 1;
        }
        if request.category.is_some() {
            query_parts.push(format!("category = ${}", bind_count));
            bind_count += 1;
        }
        if request.difficulty_level.is_some() {
            query_parts.push(format!("difficulty_level = ${}", bind_count));
            bind_count += 1;
        }
        if request.is_active.is_some() {
            query_parts.push(format!("is_active = ${}", bind_count));
            bind_count += 1;
        }

        if query_parts.is_empty() {
            return Err(AppError::BadRequest("No fields to update".to_string()));
        }

        query_parts.push(format!("updated_at = ${}", bind_count));
        let query = format!(
            "UPDATE courses SET {} WHERE id = ${}",
            query_parts.join(", "),
            bind_count + 1
        );

        let mut query_builder = sqlx::query(&query);

        // Bind values in the same order as the query parts
        if let Some(title) = &request.title {
            query_builder = query_builder.bind(title);
        }
        if let Some(description) = &request.description {
            query_builder = query_builder.bind(description);
        }
        if let Some(price) = request.price {
            query_builder = query_builder.bind(price);
        }
        if let Some(duration_minutes) = request.duration_minutes {
            query_builder = query_builder.bind(duration_minutes);
        }
        if let Some(category) = &request.category {
            query_builder = query_builder.bind(category);
        }
        if let Some(difficulty_level) = &request.difficulty_level {
            query_builder = query_builder.bind(difficulty_level);
        }
        if let Some(is_active) = request.is_active {
            query_builder = query_builder.bind(is_active);
        }

        query_builder = query_builder.bind(Utc::now()).bind(course_id);

        query_builder.execute(pool).await?;

        // Return updated course
        Self::get_course_by_id(pool, course_id).await
    }

    pub async fn delete_course(
        pool: &DbPool,
        course_id: Uuid,
        tutor_id: Uuid,
    ) -> AppResult<()> {
        let result = sqlx::query(
            "DELETE FROM courses WHERE id = $1 AND tutor_id = (SELECT id FROM tutors WHERE user_id = $2)"
        )
        .bind(course_id)
        .bind(tutor_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Course not found or access denied".to_string()));
        }

        Ok(())
    }

    pub async fn get_courses_by_tutor(pool: &DbPool, tutor_id: Uuid) -> AppResult<Vec<CourseResponse>> {
        let courses = sqlx::query_as::<_, Course>(
            r#"
            SELECT c.id, c.title, c.description, c.tutor_id, c.price, c.duration_minutes, 
                   c.category, c.difficulty_level, c.is_active, c.created_at, c.updated_at
            FROM courses c
            JOIN tutors t ON c.tutor_id = t.id
            WHERE t.user_id = $1
            ORDER BY c.created_at DESC
            "#
        )
        .bind(tutor_id)
        .fetch_all(pool)
        .await?;

        let mut course_responses = Vec::new();
        for course in courses {
            let mut course_response = CourseResponse::from(course);
            
            // Get tutor name
            if let Ok(tutor_name) = Self::get_tutor_name(pool, course_response.tutor_id).await {
                course_response.tutor_name = Some(tutor_name);
            }
            
            course_responses.push(course_response);
        }

        Ok(course_responses)
    }

    async fn get_tutor_name(pool: &DbPool, tutor_id: Uuid) -> AppResult<String> {
        let row = sqlx::query(
            "SELECT u.first_name, u.last_name FROM users u JOIN tutors t ON u.id = t.user_id WHERE t.id = $1"
        )
        .bind(tutor_id)
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            let first_name: String = row.get("first_name");
            let last_name: String = row.get("last_name");
            Ok(format!("{} {}", first_name, last_name))
        } else {
            Err(AppError::NotFound("Tutor not found".to_string()))
        }
    }
}
