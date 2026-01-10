-- Create magic_links table
CREATE TABLE IF NOT EXISTS magic_links (
    token_hash TEXT PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    used BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for faster lookups by user_id
CREATE INDEX idx_magic_links_user_id ON magic_links(user_id);
