INSERT INTO "user" (id, name, email)
VALUES ($1, $2, $3::text)
RETURNING id, name, email, created, updated;