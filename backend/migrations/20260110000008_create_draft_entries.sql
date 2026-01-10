CREATE TABLE IF NOT EXISTS draft_entries (
    id UUID PRIMARY KEY,
    draft_timetable_id UUID NOT NULL REFERENCES draft_timetables(id) ON DELETE CASCADE,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    teacher_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    room_id UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    time_slot_id UUID NOT NULL REFERENCES time_slots(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_draft_entries_updated_at
BEFORE UPDATE ON draft_entries
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();
