-- CreateEnum
CREATE TYPE "ProductAge" AS ENUM ('ALL_AGE', 'R_RATED', 'ADULT');

-- CreateTable
CREATE TABLE "Product" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT,
    "series" TEXT,
    "seriesName" TEXT,
    "circleId" TEXT NOT NULL,
    "actor" TEXT[],
    "author" TEXT[],
    "illustrator" TEXT[],
    "price" INTEGER NOT NULL,
    "saleCount" INTEGER NOT NULL,
    "age" "ProductAge" NOT NULL,
    "releasedAt" TIMESTAMP(3) NOT NULL,
    "rating" DOUBLE PRECISION,
    "ratingCount" INTEGER NOT NULL,
    "commentCount" INTEGER NOT NULL,
    "path" TEXT NOT NULL,

    CONSTRAINT "Product_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Genre" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,

    CONSTRAINT "Genre_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ProductUserGenre" (
    "productId" TEXT NOT NULL,
    "genreId" TEXT NOT NULL,
    "voteCount" INTEGER NOT NULL,

    CONSTRAINT "ProductUserGenre_pkey" PRIMARY KEY ("genreId","productId")
);

-- CreateTable
CREATE TABLE "Circle" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,

    CONSTRAINT "Circle_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "_GenreToProduct" (
    "A" TEXT NOT NULL,
    "B" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "_GenreToProduct_AB_unique" ON "_GenreToProduct"("A", "B");

-- CreateIndex
CREATE INDEX "_GenreToProduct_B_index" ON "_GenreToProduct"("B");

-- AddForeignKey
ALTER TABLE "Product" ADD CONSTRAINT "Product_circleId_fkey" FOREIGN KEY ("circleId") REFERENCES "Circle"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ProductUserGenre" ADD CONSTRAINT "ProductUserGenre_productId_fkey" FOREIGN KEY ("productId") REFERENCES "Product"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ProductUserGenre" ADD CONSTRAINT "ProductUserGenre_genreId_fkey" FOREIGN KEY ("genreId") REFERENCES "Genre"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_GenreToProduct" ADD CONSTRAINT "_GenreToProduct_A_fkey" FOREIGN KEY ("A") REFERENCES "Genre"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_GenreToProduct" ADD CONSTRAINT "_GenreToProduct_B_fkey" FOREIGN KEY ("B") REFERENCES "Product"("id") ON DELETE CASCADE ON UPDATE CASCADE;
