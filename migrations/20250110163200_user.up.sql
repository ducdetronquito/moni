
CREATE EXTENSION citext;

-- Email validation follows HTML5 spec
-- which is more restrictive than email spec (RFC 5322) but
-- it should be more than enough for moni's use case. 
-- Cf: https://html.spec.whatwg.org/multipage/input.html#e-mail-state-(type=email)
CREATE DOMAIN email AS citext
CHECK ( value ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

CREATE TABLE IF NOT EXISTS "user"
(
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email email NOT NULL UNIQUE,
    created TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc'),
    updated TIMESTAMPTZ NOT NULL
);

CREATE TRIGGER bump_user_updated_column
BEFORE INSERT OR UPDATE ON "user"
FOR EACH ROW EXECUTE PROCEDURE bump_updated_column();
