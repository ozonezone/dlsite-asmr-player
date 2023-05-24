--! insert_product
INSERT INTO product(id, name, description, series, circle_id, actor, author, illustrator, price, sale_count, age, released_at, rating, rating_count, comment_count, path)
SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::int[], $6::text[][], $7::text[][], $8::text[][], $9::int[], $10::int[], $11::text[], $12::date[], $13::float[], $14::int[], $15::int[], $16::text[])
AS _
ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name,
  description = EXCLUDED.description,
  series = EXCLUDED.series,
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
