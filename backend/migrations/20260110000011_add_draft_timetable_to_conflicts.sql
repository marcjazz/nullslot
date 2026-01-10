ALTER TABLE conflicts ADD COLUMN draft_timetable_id UUID NOT NULL REFERENCES draft_timetables(id) ON DELETE CASCADE;
