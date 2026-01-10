use async_graphql::{EmptySubscription, Schema};
use std::sync::Arc;
use crate::ws::Broadcaster;
use crate::graphql::query::Query;
use crate::graphql::mutation::Mutation;
use crate::repository::{
    UserRepository, ResourceRepository, CourseRepository, RoomRepository,
    TimeSlotRepository, TimetableEntryRepository, SubstitutionRepository,
    AvailabilityRepository, ConflictRepository, DraftTimetableRepository,
    PublishedTimetableRepository, DraftEntryRepository
};
use crate::service::{
    UserService, ResourceService, CourseService, RoomService,
    TimeSlotService, TimetableEntryService, SubstitutionService,
    NotificationService, SnapshotService, AvailabilityService,
    ConflictService, DraftTimetableService, PublishedTimetableService,
    DraftEntryService
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(pool: sqlx::PgPool, broadcaster: Arc<Broadcaster>) -> AppSchema {
    let user_repo = UserRepository::new(pool.clone());
    let resource_repo = ResourceRepository::new(pool.clone());
    let course_repo = CourseRepository::new(pool.clone());
    let room_repo = RoomRepository::new(pool.clone());
    let time_slot_repo = TimeSlotRepository::new(pool.clone());
    let timetable_entry_repo = TimetableEntryRepository::new(pool.clone());
    let substitution_repo = SubstitutionRepository::new(pool.clone());
    let availability_repo = AvailabilityRepository::new(pool.clone());
    let conflict_repo = ConflictRepository::new(pool.clone());
    let draft_timetable_repo = DraftTimetableRepository::new(pool.clone());
    let published_timetable_repo = PublishedTimetableRepository::new(pool.clone());
    let draft_entry_repo = DraftEntryRepository::new(pool.clone());
    
    let user_service = UserService::new(user_repo.clone());
    let resource_service = ResourceService::new(resource_repo.clone());
    let course_service = CourseService::new(course_repo.clone());
    let room_service = RoomService::new(room_repo.clone());
    let time_slot_service = TimeSlotService::new(time_slot_repo.clone());
    let timetable_entry_service =
        TimetableEntryService::new(timetable_entry_repo.clone(), broadcaster.clone());
    let notification_service = NotificationService::new();
    let substitution_service = SubstitutionService::new(
        substitution_repo.clone(),
        notification_service.clone(),
        broadcaster.clone(),
    );
    let snapshot_service = SnapshotService::new(
        course_repo,
        room_repo,
        time_slot_repo.clone(),
        timetable_entry_repo,
        user_repo,
    );
    let availability_service = Arc::new(AvailabilityService::new(availability_repo));
    let draft_entry_service = Arc::new(DraftEntryService::new(draft_entry_repo));
    let conflict_service = Arc::new(ConflictService::new(
        conflict_repo,
        draft_entry_service.clone(),
        availability_service.clone(),
        time_slot_repo,
    ));
    let draft_timetable_service = Arc::new(DraftTimetableService::new(draft_timetable_repo));
    let published_timetable_service = PublishedTimetableService::new(
        published_timetable_repo,
        draft_timetable_service.clone(),
        conflict_service.clone(),
    );

    Schema::build(Query, Mutation, EmptySubscription)
        .data(pool)
        .data(user_service)
        .data(resource_service)
        .data(course_service)
        .data(room_service)
        .data(time_slot_service)
        .data(timetable_entry_service)
        .data(notification_service)
        .data(substitution_service)
        .data(snapshot_service)
        .data(availability_service)
        .data(conflict_service)
        .data(draft_timetable_service)
        .data(draft_entry_service)
        .data(published_timetable_service)
        .finish()
}
