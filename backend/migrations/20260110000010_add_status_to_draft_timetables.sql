CREATE TYPE draft_timetable_status AS ENUM ('draft', 'published', 'archived');

ALTER TABLE draft_timetables ADD COLUMN status draft_timetable_status NOT NULL DEFAULT 'draft';
