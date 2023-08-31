CREATE TABLE IF NOT EXISTS "user" (
    "id" INT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "password" TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS "circle" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL
);

CREATE TYPE age_category AS ENUM ('general', 'r15', 'adult');

CREATE TABLE IF NOT EXISTS "product" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "circle_id" TEXT NOT NULL,
    "price" INT NOT NULL,
    "sale_count" INT NOT NULL,
    "age" age_category NOT NULL,
    "released_at" DATE NOT NULL,
    "rating_count" INT NOT NULL,
    "comment_count" INT NOT NULL,
    "path" TEXT NOT NULL,
    "image" TEXT[] NOT NULL,
    "description" TEXT,
    "series" TEXT,
    "rating" REAL,
    "created_at" timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY ("circle_id") REFERENCES "circle" ("id")
);

CREATE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'UPDATE') THEN
        NEW.updated_at := now();
        return NEW;
    END IF;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER product_update BEFORE UPDATE ON product FOR EACH ROW EXECUTE PROCEDURE set_updated_at();

CREATE TABLE IF NOT EXISTS "genre" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS "product_genre" (
    "product_id" TEXT NOT NULL,
    "genre_id" TEXT NOT NULL,
    FOREIGN KEY ("product_id") REFERENCES "product" ("id"),
    FOREIGN KEY ("genre_id") REFERENCES "genre" ("id"),
    PRIMARY KEY ("product_id", "genre_id")
);
CREATE TABLE IF NOT EXISTS "product_user_genre" (
    "product_id" TEXT NOT NULL,
    "genre_id" TEXT NOT NULL,
    "count" INT NOT NULL,
    FOREIGN KEY ("product_id") REFERENCES "product" ("id"),
    FOREIGN KEY ("genre_id") REFERENCES "genre" ("id"),
    PRIMARY KEY ("product_id", "genre_id")
);

CREATE TYPE creator_role AS ENUM ('voice_actor', 'creator', 'illustrator');

CREATE TABLE IF NOT EXISTS "creator" (
    "id" SERIAL NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS "product_creator" (
    "product_id" TEXT NOT NULL,
    "creator_id" SERIAL NOT NULL,
    "role" creator_role NOT NULL,
    FOREIGN KEY ("product_id") REFERENCES "product" ("id"),
    FOREIGN KEY ("creator_id") REFERENCES "creator" ("id"),
    PRIMARY KEY ("product_id", "creator_id")
);
