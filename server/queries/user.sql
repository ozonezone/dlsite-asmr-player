--! insert_user
INSERT INTO users(id, password) 
VALUES (:id, :password)
ON CONFLICT (id) DO NOTHING;

--! get_user
SELECT * FROM users WHERE id = :id;

--! exist_user
SELECT COUNT(*) FROM users WHERE id = :id;

--! change_password
UPDATE users SET password = :password WHERE id = :id;
