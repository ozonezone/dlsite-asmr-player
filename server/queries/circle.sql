--! upsert_circle
INSERT INTO circle(id, name)
VALUES (:id, :name)
ON CONFLICT (id) DO UPDATE SET name = :name;
