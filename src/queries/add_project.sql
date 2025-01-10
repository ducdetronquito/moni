INSERT INTO project (id, name, owner_id)
VALUES ($1, $2, $3)
RETURNING id, name, owner_id, created, updated;