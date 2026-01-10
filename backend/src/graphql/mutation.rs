use async_graphql::{Context, Object, InputObject, Result};
use chrono::NaiveTime;
use uuid::Uuid;
use crate::models::{Resource, Token, Course, Room, TimeSlot, TimetableEntry, Substitution, User, UserRole};
use crate::service::{
    UserService, ResourceService, CourseService, RoomService, 
    TimeSlotService, TimetableEntryService, SubstitutionService
};
use crate::error::AppError;

pub struct Mutation;

#[derive(InputObject)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct CreateResourceInput {
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(InputObject)]
pub struct CreateCourseInput {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(InputObject)]
pub struct UpdateCourseInput {
    pub id: Uuid,
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(InputObject)]
pub struct CreateRoomInput {
    pub name: String,
    pub capacity: i32,
}

#[derive(InputObject)]
pub struct UpdateRoomInput {
    pub id: Uuid,
    pub name: Option<String>,
    pub capacity: Option<i32>,
}

#[derive(InputObject)]
pub struct CreateTimeSlotInput {
    pub day_of_week: i32,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(InputObject)]
pub struct UpdateTimeSlotInput {
    pub id: Uuid,
    pub day_of_week: Option<i32>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
}

#[derive(InputObject)]
pub struct CreateTimetableEntryInput {
    pub course_id: Uuid,
    pub room_id: Uuid,
    pub time_slot_id: Uuid,
    pub teacher_id: Uuid,
}

#[derive(InputObject)]
pub struct UpdateTimetableEntryInput {
    pub id: Uuid,
    pub course_id: Option<Uuid>,
    pub room_id: Option<Uuid>,
    pub time_slot_id: Option<Uuid>,
    pub teacher_id: Option<Uuid>,
}

#[Object]
impl Mutation {
    async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<Token> {
        let service = ctx.data::<UserService>()?;
        let _user = service.create_user(input.username, input.email, input.password).await?;
        
        // Mock token for now as per original implementation
        Ok(Token {
            access_token: "mock_access_token".to_string(),
            refresh_token: "mock_refresh_token".to_string(),
            expires_in: 3600,
        })
    }

    async fn create_resource(&self, ctx: &Context<'_>, input: CreateResourceInput) -> Result<Resource> {
        let service = ctx.data::<ResourceService>()?;
        
        let resource = service.create_resource(
            input.owner_id,
            input.name,
            input.description,
            input.metadata,
        ).await?;
        
        Ok(resource)
    }

    async fn create_course(&self, ctx: &Context<'_>, input: CreateCourseInput) -> Result<Course> {
        let service = ctx.data::<CourseService>()?;
        Ok(service.create_course(input.code, input.name, input.description).await?)
    }

    async fn update_course(&self, ctx: &Context<'_>, input: UpdateCourseInput) -> Result<Course> {
        let service = ctx.data::<CourseService>()?;
        Ok(service.update_course(input.id, input.code, input.name, input.description).await?)
    }

    async fn delete_course(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let service = ctx.data::<CourseService>()?;
        service.delete_course(id).await?;
        Ok(true)
    }

    async fn create_room(&self, ctx: &Context<'_>, input: CreateRoomInput) -> Result<Room> {
        let service = ctx.data::<RoomService>()?;
        Ok(service.create_room(input.name, input.capacity).await?)
    }

    async fn update_room(&self, ctx: &Context<'_>, input: UpdateRoomInput) -> Result<Room> {
        let service = ctx.data::<RoomService>()?;
        Ok(service.update_room(input.id, input.name, input.capacity).await?)
    }

    async fn delete_room(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let service = ctx.data::<RoomService>()?;
        service.delete_room(id).await?;
        Ok(true)
    }

    async fn create_time_slot(&self, ctx: &Context<'_>, input: CreateTimeSlotInput) -> Result<TimeSlot> {
        let service = ctx.data::<TimeSlotService>()?;
        Ok(service.create_time_slot(input.day_of_week, input.start_time, input.end_time).await?)
    }

    async fn update_time_slot(&self, ctx: &Context<'_>, input: UpdateTimeSlotInput) -> Result<TimeSlot> {
        let service = ctx.data::<TimeSlotService>()?;
        Ok(service.update_time_slot(input.id, input.day_of_week, input.start_time, input.end_time).await?)
    }

    async fn delete_time_slot(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let service = ctx.data::<TimeSlotService>()?;
        service.delete_time_slot(id).await?;
        Ok(true)
    }

    async fn create_timetable_entry(&self, ctx: &Context<'_>, input: CreateTimetableEntryInput) -> Result<TimetableEntry> {
        let service = ctx.data::<TimetableEntryService>()?;
        Ok(service.create_timetable_entry(input.course_id, input.room_id, input.time_slot_id, input.teacher_id).await?)
    }

    async fn update_timetable_entry(&self, ctx: &Context<'_>, input: UpdateTimetableEntryInput) -> Result<TimetableEntry> {
        let service = ctx.data::<TimetableEntryService>()?;
        Ok(service.update_timetable_entry(input.id, input.course_id, input.room_id, input.time_slot_id, input.teacher_id).await?)
    }

    async fn delete_timetable_entry(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let service = ctx.data::<TimetableEntryService>()?;
        service.delete_timetable_entry(id).await?;
        Ok(true)
    }

    async fn request_substitution(&self, ctx: &Context<'_>, timetable_entry_id: Uuid) -> Result<Substitution> {
        let user = ctx.data::<Option<User>>()?.as_ref().ok_or(AppError::Unauthorized)?;
        if user.role != UserRole::Admin {
            return Err(AppError::Forbidden("Only admins can request substitutions".to_string()).into());
        }

        let service = ctx.data::<SubstitutionService>()?;
        Ok(service.request_substitution(timetable_entry_id).await?)
    }

    async fn accept_substitution(&self, ctx: &Context<'_>, substitution_id: Uuid) -> Result<Substitution> {
        let user = ctx.data::<Option<User>>()?.as_ref().ok_or(AppError::Unauthorized)?;
        if user.role != UserRole::Teacher {
            return Err(AppError::Forbidden("Only teachers can accept substitutions".to_string()).into());
        }

        let service = ctx.data::<SubstitutionService>()?;
        Ok(service.accept_substitution(substitution_id, user.id).await?)
    }

    async fn reject_substitution(&self, ctx: &Context<'_>, substitution_id: Uuid) -> Result<Substitution> {
        let _user = ctx.data::<Option<User>>()?.as_ref().ok_or(AppError::Unauthorized)?;
        // Based on instructions, anyone authorized (Admin or Teacher) might be able to reject?
        // Usually it's the admin or the teacher who was assigned.
        // The prompt says: "Add a rejectSubstitution(substitution_id: UUID) mutation." without specifying role restriction, 
        // but typically it's also restricted.
        
        let service = ctx.data::<SubstitutionService>()?;
        Ok(service.reject_substitution(substitution_id).await?)
    }
}
