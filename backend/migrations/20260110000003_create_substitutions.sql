-- Create substitution_status enum
CREATE TYPE substitution_status AS ENUM ('Pending', 'Accepted', 'Rejected');

-- Add 'Teacher' to user_role enum
ALTER TYPE user_role ADD VALUE 'Teacher';

-- Create substitutions table
CREATE TABLE IF NOT EXISTS substitutions (
    id UUID PRIMARY KEY,
    timetable_entry_id UUID NOT NULL REFERENCES timetable_entries(id) ON DELETE CASCADE,
    substituting_teacher_id UUID REFERENCES users(id) ON DELETE SET NULL,
    status substitution_status NOT NULL DEFAULT 'Pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_substitutions_updated_at
BEFORE UPDATE ON substitutions
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();
