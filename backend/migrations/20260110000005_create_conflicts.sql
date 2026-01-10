CREATE TYPE conflict_status AS ENUM ('Open', 'Resolved', 'Ignored');

CREATE TABLE IF NOT EXISTS conflicts (
    id UUID PRIMARY KEY,
    description TEXT NOT NULL,
    teacher_id UUID REFERENCES users(id) ON DELETE CASCADE,
    room_id UUID REFERENCES rooms(id) ON DELETE CASCADE,
    time_slot_id UUID REFERENCES time_slots(id) ON DELETE CASCADE,
    status conflict_status NOT NULL DEFAULT 'Open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_conflicts_updated_at
BEFORE UPDATE ON conflicts
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();
