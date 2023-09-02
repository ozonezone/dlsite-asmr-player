-- CreateEnum
CREATE TYPE "AgeCategory" AS ENUM ('General', 'R15', 'Adult');

-- CreateEnum
CREATE TYPE "CreatorRole" AS ENUM ('VoiceActor', 'Creator', 'Illustrator', 'ScenarioWriter');

-- CreateTable
CREATE TABLE "User" (
    "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "User_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Product" (
    "id" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "circleId" TEXT NOT NULL,
    "price" INTEGER NOT NULL,
    "sale_count" INTEGER NOT NULL,
    "age" "AgeCategory" NOT NULL,
    "released_at" TIMESTAMP(3) NOT NULL,
    "rate_count" INTEGER NOT NULL,
    "review_count" INTEGER NOT NULL,
    "path" TEXT NOT NULL,
    "images" TEXT[],
    "description" TEXT,
    "series" TEXT,
    "rating" DOUBLE PRECISION,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "Product_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Circle" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,

    CONSTRAINT "Circle_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ProductGenre" (
    "productId" TEXT NOT NULL,
    "genreId" TEXT NOT NULL,

    CONSTRAINT "ProductGenre_pkey" PRIMARY KEY ("productId","genreId")
);

-- CreateTable
CREATE TABLE "ProductUserGenre" (
    "productId" TEXT NOT NULL,
    "genreId" TEXT NOT NULL,
    "count" INTEGER NOT NULL,

    CONSTRAINT "ProductUserGenre_pkey" PRIMARY KEY ("productId","genreId")
);

-- CreateTable
CREATE TABLE "Genre" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,

    CONSTRAINT "Genre_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ProductCreator" (
    "productId" TEXT NOT NULL,
    "creatorName" TEXT NOT NULL,
    "role" "CreatorRole" NOT NULL,

    CONSTRAINT "ProductCreator_pkey" PRIMARY KEY ("productId","creatorName")
);

-- CreateTable
CREATE TABLE "Creator" (
    "name" TEXT NOT NULL,

    CONSTRAINT "Creator_pkey" PRIMARY KEY ("name")
);

-- AddForeignKey
ALTER TABLE "Product" ADD CONSTRAINT "Product_circleId_fkey" FOREIGN KEY ("circleId") REFERENCES "Circle"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ProductGenre" ADD CONSTRAINT "ProductGenre_productId_fkey" FOREIGN KEY ("productId") REFERENCES "Product"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ProductGenre" ADD CONSTRAINT "ProductGenre_genreId_fkey" FOREIGN KEY ("genreId") REFERENCES "Genre"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ProductUserGenre" ADD CONSTRAINT "ProductUserGenre_productId_fkey" FOREIGN KEY ("productId") REFERENCES "Product"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ProductUserGenre" ADD CONSTRAINT "ProductUserGenre_genreId_fkey" FOREIGN KEY ("genreId") REFERENCES "Genre"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ProductCreator" ADD CONSTRAINT "ProductCreator_productId_fkey" FOREIGN KEY ("productId") REFERENCES "Product"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ProductCreator" ADD CONSTRAINT "ProductCreator_creatorName_fkey" FOREIGN KEY ("creatorName") REFERENCES "Creator"("name") ON DELETE RESTRICT ON UPDATE CASCADE;
