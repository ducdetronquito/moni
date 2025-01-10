CREATE TABLE IF NOT EXISTS project
(
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc'),
    updated TIMESTAMPTZ NOT NULL,
    owner_id UUID NOT NULL REFERENCES "user" (id)
);


CREATE TRIGGER bump_project_updated_column
BEFORE INSERT OR UPDATE ON project
FOR EACH ROW EXECUTE PROCEDURE bump_updated_column();


CREATE INDEX project_owner_id
ON project(owner_id);
