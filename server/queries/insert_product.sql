--! exist_product
SELECT id FROM product WHERE id = ANY(:ids);

--! insert_product(description?,series?,rating?)
INSERT INTO product(id, name, description, series, circle_id, actor, author, illustrator, price, sale_count, age, released_at, rating, rating_count, comment_count, path) 
VALUES (:id, :name, :description, :series, :circle_id, :actor, :author, :illustrator, :price, :sale_count, :age, :released_at, :rating, :rating_count, :comment_count, :path);
