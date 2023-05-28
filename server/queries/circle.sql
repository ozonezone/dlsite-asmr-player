--! upsert_circle
INSERT INTO circle(id, name)
VALUES (:id, :name)
ON CONFLICT (id) DO UPDATE SET name = :name;

--! get_circle_product_released_at_asc : (description?,series?,rating?)
SELECT product.*, c.name circle_name FROM product
  JOIN circle c on product.circle_id = :circle_id and c.id = product.circle_id 
ORDER BY released_at ASC LIMIT :limit OFFSET :offset;
--! get_circle_product_released_at_desc : (description?,series?,rating?)
SELECT product.*, c.name circle_name FROM product
  JOIN circle c on product.circle_id = :circle_id and c.id = product.circle_id 
ORDER BY released_at DESC LIMIT :limit OFFSET :offset;

--! get_circle_product_name_asc : (description?,series?,rating?)
SELECT product.*, c.name circle_name FROM product 
  JOIN circle c on product.circle_id = :circle_id and c.id = product.circle_id 
ORDER BY name ASC LIMIT :limit OFFSET :offset;
--! get_circle_product_name_desc : (description?,series?,rating?)
SELECT product.*, c.name circle_name FROM product 
  JOIN circle c on product.circle_id = :circle_id and c.id = product.circle_id 
ORDER BY name DESC LIMIT :limit OFFSET :offset;

--! count_circle_product
SELECT COUNT(*) FROM product WHERE circle_id = :circle_id;
