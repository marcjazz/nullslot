CREATE TABLE IF NOT EXISTS published_timetables (
    id UUID PRIMARY KEY,
    draft_timetable_id UUID NOT NULL REFERENCES draft_timetables(id) ON DELETE RESTRICT,
    published_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    valid_from DATE NOT NULL,
    valid_to DATE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_published_timetables_updated_at
BEFORE UPDATE ON published_timetables
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();
