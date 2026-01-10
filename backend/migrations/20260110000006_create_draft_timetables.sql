CREATE TABLE IF NOT EXISTS draft_timetables (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    term TEXT NOT NULL, -- e.g., "Spring 2026"
    year INT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_draft_timetables_updated_at
BEFORE UPDATE ON draft_timetables
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();
