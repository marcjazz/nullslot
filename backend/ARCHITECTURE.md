# Backend Architecture Design (GraphQL)

## 1. Overview

The backend is a Rust service designed with a clean modular architecture to support a GraphQL API. It emphasizes a clear separation of concerns between API resolvers, domain logic, persistence, and infrastructure. It also serves offline snapshots for the frontend.

- Language: Rust
- Web framework: Axum
- API: GraphQL (async-graphql)
- Database: PostgreSQL
- ORM/Query: sqlx
- Authentication: JWT (HS256)
- Observability: tracing, structured logging

## 2. API Endpoints

### GraphQL API
- POST /graphql: The main entry point for all GraphQL queries and mutations.

## 3. GraphQL Schema (Simplified)

### Queries
- `users`: Returns a list of users.
- `user(id: UUID)`: Returns a specific user.
- `courses`: Returns a list of courses.
- `course(id: UUID)`: Returns a specific course.
- `rooms`: Returns a list of rooms.
- `room(id: UUID)`: Returns a specific room.
- `teachers`: Returns a list of teachers.
- `teacher(id: UUID)`: Returns a specific teacher.
- `timetable(from: DateTime, to: DateTime)`: Returns timetable entries for a given period.
- `substitutions`: Returns a list of substitution requests and their statuses.

### Mutations
- `register(input: RegisterInput)`: Registers a new user and returns tokens.
- `login(input: LoginInput)`: Authenticates a user and returns tokens.
- `createCourse(input: CreateCourseInput)`: Creates a new course.
- `createRoom(input: CreateRoomInput)`: Creates a new room.
- `createTeacher(input: CreateTeacherInput)`: Creates a new teacher.
- `createTimetableEntry(input: CreateTimetableEntryInput)`: Creates a new timetable entry.
- `requestSubstitution(input: RequestSubstitutionInput)`: Allows an admin to mark a class for substitution.
- `acceptSubstitution(input: AcceptSubstitutionInput)`: Allows a teacher to accept a substitution request.

## 4. Data Models

### User
```rust
struct User {
  id: uuid::Uuid,
  username: String,
  email: String,
  role: UserRole, // Admin, Teacher, Student
  created_at: chrono::DateTime<chrono::Utc>,
}
```

### Course
```rust
struct Course {
  id: uuid::Uuid,
  name: String,
  credits: i32,
}
```

### Room
```rust
struct Room {
  id: uuid::Uuid,
  name: String,
  capacity: i32,
}
```

### TimetableEntry
```rust
struct TimetableEntry {
  id: uuid::Uuid,
  course_id: uuid::Uuid,
  teacher_id: uuid::Uuid,
  room_id: uuid::Uuid,
  start_time: chrono::DateTime<chrono::Utc>,
  end_time: chrono::DateTime<chrono::Utc>,
}
```

### SubstitutionLog
```rust
struct SubstitutionLog {
    id: uuid::Uuid,
    timetable_entry_id: uuid::Uuid,
    original_teacher_id: uuid::Uuid,
    substitute_teacher_id: Option<uuid::Uuid>,
    status: SubstitutionStatus, // Requested, Accepted, Rejected
    requested_at: chrono::DateTime<chrono::Utc>,
}
```

## 5. Persistence and Domain Layer

- PostgreSQL database with `sqlx`.
- Repository pattern for DB access.
- Service layer for domain logic.
- Tables for: Users, Courses, Rooms, Timetable entries, Substitution logs, and Notifications history.

## 6. Additional Features

- **WebSockets**: For instant timetable updates to connected clients.
- **Push Notifications**: For mobile/desktop notifications about schedule changes.
- **Audit Logs**: For tracking important changes and maintaining accountability.
