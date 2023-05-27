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

--! get_genre
SELECT product_id, genre_id, name FROM product_genre JOIN genre g on product_genre.product_id = :genre and g.id = product_genre.genre_id;
--! get_genres
SELECT product_id, genre_id, name FROM product_genre JOIN genre g on product_genre.product_id = ANY(:genres) and g.id = product_genre.genre_id;

--! get_usergenre
SELECT product_id, genre_id, name FROM product_usergenre JOIN genre g on product_usergenre.product_id = :genre and g.id = product_usergenre.genre_id;
--! get_usergenres
SELECT product_id, genre_id, name, count FROM product_usergenre JOIN genre g on product_usergenre.product_id = ANY(:genres) and g.id = product_usergenre.genre_id;
