use crate::error::AppResult;
use crate::models::snapshot::TimetableSnapshot;
use crate::models::UserRole;
use crate::repository::{
    CourseRepository, RoomRepository, TimeSlotRepository, TimetableEntryRepository, UserRepository,
};
use tokio::try_join;

pub struct SnapshotService {
    course_repo: CourseRepository,
    room_repo: RoomRepository,
    time_slot_repo: TimeSlotRepository,
    timetable_entry_repo: TimetableEntryRepository,
    user_repo: UserRepository,
}

impl SnapshotService {
    pub fn new(
        course_repo: CourseRepository,
        room_repo: RoomRepository,
        time_slot_repo: TimeSlotRepository,
        timetable_entry_repo: TimetableEntryRepository,
        user_repo: UserRepository,
    ) -> Self {
        Self {
            course_repo,
            room_repo,
            time_slot_repo,
            timetable_entry_repo,
            user_repo,
        }
    }

    pub async fn get_timetable_snapshot(&self) -> AppResult<TimetableSnapshot> {
        let (courses, rooms, time_slots, timetable_entries, all_users) = try_join!(
            self.course_repo.find_all(),
            self.room_repo.find_all(),
            self.time_slot_repo.find_all(),
            self.timetable_entry_repo.find_all(),
            self.user_repo.find_all()
        )?;

        let teachers = all_users
            .into_iter()
            .filter(|u| u.role == UserRole::Teacher)
            .collect();

        Ok(TimetableSnapshot {
            courses,
            rooms,
            time_slots,
            teachers,
            timetable_entries,
        })
    }
}
