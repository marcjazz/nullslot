-- Add workspace_id column to top-level entities
ALTER TABLE resources ADD COLUMN workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE;
ALTER TABLE draft_timetables ADD COLUMN workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE;
ALTER TABLE published_timetables ADD COLUMN workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE;
ALTER TABLE courses ADD COLUMN workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE;
ALTER TABLE rooms ADD COLUMN workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE;
ALTER TABLE time_slots ADD COLUMN workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE;
ALTER TABLE availability ADD COLUMN workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE;

-- Data Migration: Create a default workspace for each existing user
DO $$
DECLARE
    u_rec RECORD;
    new_ws_id UUID;
BEGIN
    FOR u_rec IN SELECT id, COALESCE(username, email) as name FROM users LOOP
        -- Create a unique ID for the new workspace
        new_ws_id := gen_random_uuid();
        
        -- Create workspace
        INSERT INTO workspaces (id, name) 
        VALUES (new_ws_id, u_rec.name || '''s Workspace');
        
        -- Add member as Owner
        INSERT INTO workspace_members (workspace_id, user_id, role)
        VALUES (new_ws_id, u_rec.id, 'Owner');
        
        -- Assign resources owned by this user to their new workspace
        UPDATE resources SET workspace_id = new_ws_id WHERE owner_id = u_rec.id;
    END LOOP;
    
    -- For entities that were "global" (no owner_id), assign them to the first workspace
    -- This handles courses, rooms, time_slots, and timetables that don't have owner_id
    DECLARE
        first_ws_id UUID;
    BEGIN
        SELECT id INTO first_ws_id FROM workspaces ORDER BY created_at ASC LIMIT 1;
        IF first_ws_id IS NOT NULL THEN
            UPDATE draft_timetables SET workspace_id = first_ws_id WHERE workspace_id IS NULL;
            UPDATE published_timetables SET workspace_id = first_ws_id WHERE workspace_id IS NULL;
            UPDATE courses SET workspace_id = first_ws_id WHERE workspace_id IS NULL;
            UPDATE rooms SET workspace_id = first_ws_id WHERE workspace_id IS NULL;
            UPDATE time_slots SET workspace_id = first_ws_id WHERE workspace_id IS NULL;
            UPDATE availability SET workspace_id = first_ws_id WHERE workspace_id IS NULL;
        END IF;
    END;
END $$;

-- Update constraints
-- Courses: unique(workspace_id, code) instead of unique(code)
ALTER TABLE courses DROP CONSTRAINT IF EXISTS courses_code_key;
ALTER TABLE courses ADD CONSTRAINT courses_workspace_code_key UNIQUE (workspace_id, code);

-- Rooms: unique(workspace_id, name) instead of unique(name)
ALTER TABLE rooms DROP CONSTRAINT IF EXISTS rooms_name_key;
ALTER TABLE rooms ADD CONSTRAINT rooms_workspace_name_key UNIQUE (workspace_id, name);

-- Make workspace_id NOT NULL after data migration
ALTER TABLE resources ALTER COLUMN workspace_id SET NOT NULL;
ALTER TABLE draft_timetables ALTER COLUMN workspace_id SET NOT NULL;
ALTER TABLE published_timetables ALTER COLUMN workspace_id SET NOT NULL;
ALTER TABLE courses ALTER COLUMN workspace_id SET NOT NULL;
ALTER TABLE rooms ALTER COLUMN workspace_id SET NOT NULL;
ALTER TABLE time_slots ALTER COLUMN workspace_id SET NOT NULL;
ALTER TABLE availability ALTER COLUMN workspace_id SET NOT NULL;
