use async_graphql::{Context, Object, Result};
use uuid::Uuid;
use chrono::NaiveDate;
use crate::models::{User, Resource, Course, Room, TimeSlot, TimetableEntry, Substitution, snapshot::TimetableSnapshot};
use crate::graphql::types::{Availability, Conflict, DraftTimetable, PublishedTimetable};
use crate::service::{
    UserService, ResourceService, CourseService, RoomService,
    TimeSlotService, TimetableEntryService, SubstitutionService,
    SnapshotService, AvailabilityService, ConflictService,
    DraftTimetableService, PublishedTimetableService,
    auth::Claims
};
use crate::error::AppError;
use async_graphql::ErrorExtensions;

pub struct Query;

#[Object]
impl Query {
    async fn me(&self, ctx: &Context<'_>) -> Result<User> {
        let claims = ctx.data::<Claims>().map_err(|_| AppError::Unauthorized.extend())?;
        let service = ctx.data::<UserService>()?;
        service
            .get_user(claims.sub)
            .await?
            .ok_or_else(|| AppError::NotFound.extend())
    }

    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let service = ctx.data::<UserService>()?;
        Ok(service.get_all_users().await?)
    }

    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<User>> {
        let service = ctx.data::<UserService>()?;
        Ok(service.get_user(id).await?)
    }

    async fn resources(&self, ctx: &Context<'_>) -> Result<Vec<Resource>> {
        let service = ctx.data::<ResourceService>()?;
        Ok(service.get_all_resources().await?)
    }

    async fn resource(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Resource>> {
        let service = ctx.data::<ResourceService>()?;
        Ok(service.get_resource(id).await?)
    }

    async fn courses(&self, ctx: &Context<'_>) -> Result<Vec<Course>> {
        let service = ctx.data::<CourseService>()?;
        Ok(service.get_all_courses().await?)
    }

    async fn course(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Course>> {
        let service = ctx.data::<CourseService>()?;
        Ok(service.get_course(id).await?)
    }

    async fn rooms(&self, ctx: &Context<'_>) -> Result<Vec<Room>> {
        let service = ctx.data::<RoomService>()?;
        Ok(service.get_all_rooms().await?)
    }

    async fn room(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Room>> {
        let service = ctx.data::<RoomService>()?;
        Ok(service.get_room(id).await?)
    }

    async fn time_slots(&self, ctx: &Context<'_>) -> Result<Vec<TimeSlot>> {
        let service = ctx.data::<TimeSlotService>()?;
        Ok(service.get_all_time_slots().await?)
    }

    async fn time_slot(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<TimeSlot>> {
        let service = ctx.data::<TimeSlotService>()?;
        Ok(service.get_time_slot(id).await?)
    }

    async fn timetable_entries(&self, ctx: &Context<'_>) -> Result<Vec<TimetableEntry>> {
        let service = ctx.data::<TimetableEntryService>()?;
        Ok(service.get_all_timetable_entries().await?)
    }

    async fn timetable_entry(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<TimetableEntry>> {
        let service = ctx.data::<TimetableEntryService>()?;
        Ok(service.get_timetable_entry(id).await?)
    }

    async fn substitutions(&self, ctx: &Context<'_>) -> Result<Vec<Substitution>> {
        let service = ctx.data::<SubstitutionService>()?;
        Ok(service.get_all_substitutions().await?)
    }

    async fn substitution(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Substitution>> {
        let service = ctx.data::<SubstitutionService>()?;
        Ok(service.get_substitution(id).await?)
    }

    async fn timetable_snapshot(&self, ctx: &Context<'_>) -> Result<TimetableSnapshot> {
        let service = ctx.data::<SnapshotService>()?;
        Ok(service.get_timetable_snapshot().await?)
    }

    async fn availability(&self, ctx: &Context<'_>, teacher_id: Uuid, date: NaiveDate) -> Result<Vec<Availability>> {
        let service = ctx.data::<AvailabilityService>()?;
        Ok(service.get_availability(teacher_id, date).await?)
    }

    async fn conflicts(&self, ctx: &Context<'_>, draft_timetable_id: Uuid) -> Result<Vec<Conflict>> {
        let service = ctx.data::<ConflictService>()?;
        Ok(service.get_conflicts(draft_timetable_id).await?)
    }

    async fn draft_timetable(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<DraftTimetable>> {
        let service = ctx.data::<DraftTimetableService>()?;
        Ok(service.get_draft_timetable(id).await?)
    }

    async fn published_timetable(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<PublishedTimetable>> {
        let service = ctx.data::<PublishedTimetableService>()?;
        Ok(service.get_published_timetable(id).await?)
    }

    async fn latest_published_timetable(&self, ctx: &Context<'_>) -> Result<Option<PublishedTimetable>> {
        let service = ctx.data::<PublishedTimetableService>()?;
        Ok(service.get_latest_published_timetable().await?)
    }
}
