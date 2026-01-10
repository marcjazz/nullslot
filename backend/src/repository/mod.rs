pub mod auth;
pub mod users;
pub mod resources;
pub mod courses;
pub mod rooms;
pub mod time_slots;
pub mod timetable_entries;
pub mod substitutions;

pub use users::UserRepository;
pub use resources::ResourceRepository;
pub use courses::CourseRepository;
pub use rooms::RoomRepository;
pub use time_slots::TimeSlotRepository;
pub use timetable_entries::TimetableEntryRepository;
pub use substitutions::SubstitutionRepository;
pub use auth::AuthRepository;
