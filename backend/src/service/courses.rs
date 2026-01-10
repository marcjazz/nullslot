use uuid::Uuid;
use chrono::Utc;
use crate::models::Course;
use crate::repository::CourseRepository;
use crate::error::AppResult;

pub struct CourseService {
    repo: CourseRepository,
}

impl CourseService {
    pub fn new(repo: CourseRepository) -> Self {
        Self { repo }
    }

    pub async fn create_course(
        &self,
        code: String,
        name: String,
        description: Option<String>,
    ) -> AppResult<Course> {
        let course = Course {
            id: Uuid::new_v4(),
            code,
            name,
            description,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.repo.create(course).await
    }

    pub async fn get_course(&self, id: Uuid) -> AppResult<Option<Course>> {
        self.repo.find_by_id(id).await
    }

    pub async fn get_all_courses(&self) -> AppResult<Vec<Course>> {
        self.repo.find_all().await
    }

    pub async fn update_course(
        &self,
        id: Uuid,
        code: Option<String>,
        name: Option<String>,
        description: Option<String>,
    ) -> AppResult<Course> {
        let mut course = self.repo.find_by_id(id).await?.ok_or(crate::error::AppError::NotFound)?;
        
        if let Some(c) = code {
            course.code = c;
        }
        if let Some(n) = name {
            course.name = n;
        }
        if let Some(d) = description {
            course.description = Some(d);
        }
        
        course.updated_at = Utc::now();
        self.repo.update(course).await
    }

    pub async fn delete_course(&self, id: Uuid) -> AppResult<()> {
        self.repo.delete(id).await
    }
}
