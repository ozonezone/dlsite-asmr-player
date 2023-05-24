--! exist_product
SELECT id FROM product WHERE id = ANY(:ids);

--! insert_product(description?,series?,rating?)
INSERT INTO product(id, name, description, series, circle_id, actor, author, illustrator, price, sale_count, age, released_at, rating, rating_count, comment_count, path) 
VALUES (:id, :name, :description, :series, :circle_id, :actor, :author, :illustrator, :price, :sale_count, :age, :released_at, :rating, :rating_count, :comment_count, :path);

--! upsert_circle
INSERT INTO circle(id, name)
VALUES (:id, :name)
ON CONFLICT (id) DO UPDATE SET name = :name;

--! upsert_genre
INSERT INTO genre(id, name)
VALUES (:id, :name)
ON CONFLICT (id) DO UPDATE SET name = :name;

--! insert_product_genre
INSERT INTO product_genre(product_id, genre_id)
VALUES (:product_id, :genre_id)
ON CONFLICT (product_id, genre_id) DO NOTHING;

--! upsert_product_usergenre
INSERT INTO product_usergenre(product_id, genre_id, count)
VALUES (:product_id, :genre_id, :count)
ON CONFLICT (product_id, genre_id) DO UPDATE SET count = :count;
