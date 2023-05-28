-- Create "circle" table
CREATE TABLE "public"."circle" (
    "id" text NOT NULL, "name" text NOT NULL, PRIMARY KEY ("id")
);
-- Create "users" table
CREATE TABLE "public"."users" (
    "id" integer NOT NULL, "password" text NOT NULL, PRIMARY KEY ("id")
);
-- Create enum type "age"
CREATE TYPE "public"."age" AS ENUM ('all_ages', 'r', 'adult');
-- Create "product" table
CREATE TABLE "public"."product" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    "description" text NULL,
    "series" text NULL,
    "circle_id" text NOT NULL,
    "actor" text [] NOT NULL,
    "author" text [] NOT NULL,
    "illustrator" text [] NOT NULL,
    "price" integer NOT NULL,
    "sale_count" integer NOT NULL,
    "age" "public" . "age" NOT NULL,
    "released_at" date NOT NULL,
    "rating" double precision NULL,
    "rating_count" integer NOT NULL,
    "comment_count" integer NOT NULL,
    "path" text NOT NULL,
    "remote_image" text [] NOT NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT "circle_fk" FOREIGN KEY (
        "circle_id"
    ) REFERENCES "public"."circle" (
        "id"
    ) ON UPDATE NO ACTION ON DELETE NO ACTION
);
-- Create "genre" table
CREATE TABLE "public"."genre" (
    "id" text NOT NULL, "name" text NOT NULL, PRIMARY KEY ("id")
);
-- Create "product_genre" table
CREATE TABLE "public"."product_genre" (
    "product_id" text NOT NULL,
    "genre_id" text NOT NULL,
    PRIMARY KEY ("product_id", "genre_id"),
    CONSTRAINT "genre_fk" FOREIGN KEY (
        "genre_id"
    ) REFERENCES "public"."genre" (
        "id"
    ) ON UPDATE NO ACTION ON DELETE NO ACTION,
    CONSTRAINT "product_fk" FOREIGN KEY (
        "product_id"
    ) REFERENCES "public"."product" (
        "id"
    ) ON UPDATE NO ACTION ON DELETE NO ACTION
);
-- Create "product_usergenre" table
CREATE TABLE "public"."product_usergenre" (
    "product_id" text NOT NULL,
    "genre_id" text NOT NULL,
    "count" integer NOT NULL,
    PRIMARY KEY ("product_id", "genre_id"),
    CONSTRAINT "genre_fk" FOREIGN KEY (
        "genre_id"
    ) REFERENCES "public"."genre" (
        "id"
    ) ON UPDATE NO ACTION ON DELETE NO ACTION,
    CONSTRAINT "product_fk" FOREIGN KEY (
        "product_id"
    ) REFERENCES "public"."product" (
        "id"
    ) ON UPDATE NO ACTION ON DELETE NO ACTION
);
