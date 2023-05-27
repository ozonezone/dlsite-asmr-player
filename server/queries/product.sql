--! product_ids
SELECT id FROM product;

--! upsert_product(description?,series?,rating?)
INSERT INTO product(id, name, description, remote_image, series, circle_id, actor, author, illustrator, price, sale_count, age, released_at, rating, rating_count, comment_count, path) 
VALUES (:id, :name, :description, :remote_image, :series, :circle_id, :actor, :author, :illustrator, :price, :sale_count, :age, :released_at, :rating, :rating_count, :comment_count, :path)
ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name,
  description = EXCLUDED.description,
  series = EXCLUDED.series,
  remote_image = EXCLUDED.remote_image,
  circle_id = EXCLUDED.circle_id,
  actor = EXCLUDED.actor,
  author = EXCLUDED.author,
  illustrator = EXCLUDED.illustrator,
  price = EXCLUDED.price,
  sale_count = EXCLUDED.sale_count,
  age = EXCLUDED.age,
  released_at = EXCLUDED.released_at,
  rating = EXCLUDED.rating,
  rating_count = EXCLUDED.rating_count,
  comment_count = EXCLUDED.comment_count,
  path = EXCLUDED.path;

--! delete_product
DELETE FROM product WHERE id = ANY(:ids);
--! delete_product_genre
DELETE FROM product_genre WHERE product_id = ANY(:ids);
--! delete_product_usergenre
DELETE FROM product_usergenre WHERE product_id = ANY(:ids);

--! get_product_released_at_asc
SELECT * FROM product  
ORDER BY released_at ASC LIMIT :limit OFFSET :offset
INNER JOIN circle ON product.circle_id = circle.id;
--! get_product_released_at_desc
SELECT * FROM product ORDER BY released_at DESC LIMIT :limit OFFSET :offset;
--! get_product_name_asc
SELECT * FROM product ORDER BY name ASC LIMIT :limit OFFSET :offset;
--! get_product_name_at_desc
SELECT * FROM product ORDER BY name DESC LIMIT :limit OFFSET :offset;

--! get_product_released_at_asc_by_circle
SELECT * FROM product ORDER BY released_at ASC LIMIT :limit OFFSET :offset WHERE circle_id  = :circle_id;
--! get_product_released_at_desc_by_circle
SELECT * FROM product ORDER BY released_at DESC LIMIT :limit OFFSET :offset WHERE circle_id  = :circle_id;
--! get_product_name_asc_by_circle
SELECT * FROM product ORDER BY name ASC LIMIT :limit OFFSET :offset WHERE circle_id  = :circle_id;
--! get_product_name_at_desc_by_circle
SELECT * FROM product ORDER BY name DESC LIMIT :limit OFFSET :offset WHERE circle_id  = :circle_id;
