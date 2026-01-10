use async_graphql::{Context, Object, Result};
use uuid::Uuid;
use crate::models::{User, Resource, Course, Room, TimeSlot, TimetableEntry, Substitution, snapshot::TimetableSnapshot};
use crate::service::{
    UserService, ResourceService, CourseService, RoomService,
    TimeSlotService, TimetableEntryService, SubstitutionService,
    SnapshotService
};

pub struct Query;

#[Object]
impl Query {
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
}
