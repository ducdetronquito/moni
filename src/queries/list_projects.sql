SELECT id, name, owner_id, created, updated
FROM project
WHERE owner_id = $1;
