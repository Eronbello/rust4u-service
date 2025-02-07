-- Create projects table
CREATE TABLE IF NOT EXISTS projects (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id        UUID NOT NULL,
    name            VARCHAR(100) NOT NULL,
    description     TEXT,
    github_link     VARCHAR(255),
    tags            TEXT[], -- Postgres array of text for project tags
    created_at      TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at      TIMESTAMP WITH TIME ZONE
);

-- Foreign key for owner_id -> users(id)
ALTER TABLE projects
ADD CONSTRAINT fk_projects_owner
FOREIGN KEY (owner_id)
REFERENCES users(id)
ON DELETE CASCADE;
