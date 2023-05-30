use sea_orm_migration::{
    prelude::*,
    sea_orm::{DbBackend, DeriveActiveEnum, EnumIter},
    sea_query::extension::postgres::Type,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Product {
    Table,
    Id,
    Name,
    Description,
    Series,
    CircleId,
    Actor,
    Author,
    Illustrator,
    Price,
    SaleCount,
    Age,
    ReleasedAt,
    Rating,
    RatingCount,
    CommentCount,
    Path,
    Image,
}
#[derive(Iden)]
enum Circle {
    Table,
    Id,
    Name,
}
#[derive(Iden)]
enum User {
    Table,
    Id,
    Name,
}
#[derive(Iden)]
enum Genre {
    Table,
    Id,
    Name,
}
#[derive(Iden)]
enum ProductGenre {
    Table,
    ProductId,
    GenreId,
}
#[derive(Iden)]
enum ProductUserGenre {
    Table,
    ProductId,
    GenreId,
    Count,
}

#[derive(Iden)]
enum Age {
    #[iden = "age"]
    Enum,
    #[iden = "All"]
    All,
    #[iden = "R"]
    R,
    #[iden = "Adult"]
    Adult,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {}
            DbBackend::Postgres => {
                manager.create_type(Type::create().as_enum(Age::Enum).values([Age::All, Age::R, Age::Adult]).to_owned()).await?;
            }
        }

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Circle::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Circle::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Circle::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Product::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(ColumnDef::new(Product::CircleId).string().not_null())
                    .col(ColumnDef::new(Product::Actor).array(ColumnType::String(None)).not_null())
                    .col(ColumnDef::new(Product::Author).array(ColumnType::String(None)).not_null())
                    .col(ColumnDef::new(Product::Illustrator).array(ColumnType::String(None)).not_null())
                    .col(ColumnDef::new(Product::Price).integer().not_null())
                    .col(ColumnDef::new(Product::SaleCount).integer().not_null())
                    .col(ColumnDef::new(Product::Age).enumeration(Age::Enum, [Age::All, Age::R, Age::Adult]).not_null())
                    .col(ColumnDef::new(Product::ReleasedAt).date().not_null())
                    .col(ColumnDef::new(Product::RatingCount).integer().not_null())
                    .col(ColumnDef::new(Product::CommentCount).integer().not_null())
                    .col(ColumnDef::new(Product::Path).string().not_null())
                    .col(ColumnDef::new(Product::Image).string().not_null())
                    // nullable
                    .col(ColumnDef::new(Product::Description).string())
                    .col(ColumnDef::new(Product::Series).string())
                    .col(ColumnDef::new(Product::Rating).float())
                    .foreign_key(ForeignKey::create().name("fk_product_circle").from(Product::Table, Product::CircleId).to(Circle::Table, Circle::Id))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Genre::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Genre::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Genre::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProductGenre::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ProductGenre::ProductId).string().not_null())
                    .col(ColumnDef::new(ProductGenre::GenreId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product")
                            .from(ProductGenre::Table, ProductGenre::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .foreign_key(ForeignKey::create().name("fk_genre").from(ProductGenre::Table, ProductGenre::GenreId).to(Genre::Table, Genre::Id))
                    .primary_key(Index::create().col(ProductGenre::ProductId).col(ProductGenre::GenreId))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProductUserGenre::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ProductUserGenre::ProductId).string().not_null())
                    .col(ColumnDef::new(ProductUserGenre::GenreId).string().not_null())
                    .col(ColumnDef::new(ProductUserGenre::Count).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product")
                            .from(ProductUserGenre::Table, ProductUserGenre::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_genre")
                            .from(ProductUserGenre::Table, ProductUserGenre::GenreId)
                            .to(Genre::Table, Genre::Id),
                    )
                    .primary_key(Index::create().col(ProductUserGenre::ProductId).col(ProductUserGenre::GenreId))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ProductUserGenre::Table).if_exists().to_owned()).await?;
        manager.drop_table(Table::drop().table(ProductGenre::Table).if_exists().to_owned()).await?;
        manager.drop_table(Table::drop().table(Product::Table).if_exists().to_owned()).await?;
        manager.drop_table(Table::drop().table(Genre::Table).if_exists().to_owned()).await?;
        manager.drop_table(Table::drop().table(Circle::Table).if_exists().to_owned()).await?;
        manager.drop_table(Table::drop().table(User::Table).if_exists().to_owned()).await?;
        manager.drop_type(Type::drop().name(Age::Enum).if_exists().to_owned()).await?;

        Ok(())
    }
}
