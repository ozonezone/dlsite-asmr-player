schema "public" {
}

enum "age" {
  schema = schema.public
  values = ["all_ages", "r", "adult"]
}

table "product" {
  schema = schema.public
  column "id" {
    null = false
    type = text
  }
  column "name" {
    null = false
    type = text
  }
  column "description" {
    null = true
    type = text
  }
  column "series" {
    null = true
    type = text
  }
  column "circle_id" {
    null = false
    type = text
  }
  column "actor" {
    null = false
    type = sql("text[]")
  }
  column "author" {
    null = false
    type = sql("text[]")
  }
  column "illustrator" {
    null = false
    type = sql("text[]")
  }
  column "price" {
    null = false
    type = int
  }
  column "sale_count" {
    null = false
    type = int
  }
  column "age" {
    null = false
    type = enum.age
  }
  column "released_at" {
    null = false
    type = date
  }
  column "rating" {
    null = true
    type = float
  }
  column "rating_count" {
    null = false
    type = int
  }
  column "comment_count" {
    null = false
    type = int
  }
  column "path" {
    null = false
    type = text
  }
  column "remote_image" {
    null = false
    type = sql("text[]")
  }
  primary_key {
    columns = [column.id]
  }
  foreign_key "circle_fk" {
    columns     = [column.circle_id]
    ref_columns = [table.circle.column.id]
  }
}

table "circle" {
  schema = schema.public
  column "id" {
    null = false
    type = text
  }
  column "name" {
    null = false
    type = text
  }
  primary_key {
    columns = [column.id]
  }
}

table "genre" {
  schema = schema.public
  column "id" {
    null = false
    type = text
  }
  column "name" {
    null = false
    type = text
  }
  primary_key {
    columns = [column.id]
  }
}

table "product_genre" {
  schema = schema.public
  column "product_id" {
    null = false
    type = text
  }
  column "genre_id" {
    null = false
    type = text
  }
  primary_key {
    columns = [column.product_id, column.genre_id]
  }
  foreign_key "product_fk" {
    columns     = [column.product_id]
    ref_columns = [table.product.column.id]
  }
  foreign_key "genre_fk" {
    columns     = [column.genre_id]
    ref_columns = [table.genre.column.id]
  }
}

table "product_usergenre" {
  schema = schema.public
  column "product_id" {
    null = false
    type = text
  }
  column "genre_id" {
    null = false
    type = text
  }
  column "count" {
    null = false
    type = int
  }
  primary_key {
    columns = [column.product_id, column.genre_id]
  }
  foreign_key "product_fk" {
    columns     = [column.product_id]
    ref_columns = [table.product.column.id]
  }
  foreign_key "genre_fk" {
    columns     = [column.genre_id]
    ref_columns = [table.genre.column.id]
  }
}
