use std::path::PathBuf;

use dlsite::product::Product;
use entity::entities::{circle, genre, product, product_genre, product_user_genre};
use migration::OnConflict;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set, TransactionError,
    TransactionTrait,
};

pub async fn create_product(
    db: &DatabaseConnection,
    product: Product,
    path: PathBuf,
) -> Result<(), anyhow::Error> {
    circle::Entity::insert(circle::ActiveModel {
        id: Set(product.circle.id.clone()),
        name: Set(product.circle.name),
    })
    .on_conflict(
        OnConflict::column(circle::Column::Id)
            .update_columns([circle::Column::Name])
            .to_owned(),
    )
    .exec(db)
    .await?;

    product::Entity::insert(product::ActiveModel {
        id: Set(product.id.clone()),
        name: Set(product.title),
        description: Set(None),
        series: Set(product.series),
        circle_id: Set(product.circle.id),
        image: Set(product
            .images
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()),
        actor: Set(product.people.voice_actor.unwrap_or_default()),
        author: Set(product.people.author.unwrap_or_default()),
        illustrator: Set(product.people.illustrator.unwrap_or_default()),
        price: Set(product.price),
        sale_count: Set(product.sale_count),
        age: Set(match product.age_rating {
            dlsite::product::AgeRating::AllAges => entity::entities::sea_orm_active_enums::Age::All,
            dlsite::product::AgeRating::R => entity::entities::sea_orm_active_enums::Age::R,
            dlsite::product::AgeRating::Adult => entity::entities::sea_orm_active_enums::Age::Adult,
        }),
        released_at: Set(product.released_at),
        rating: Set(product.rating),
        rating_count: Set(product.rate_count.unwrap_or(0)),
        comment_count: Set(product.review_count.unwrap_or(0)),
        path: Set(path.to_string_lossy().to_string()),
    })
    .on_conflict(
        OnConflict::column(product::Column::Id)
            .update_columns([
                product::Column::Name,
                product::Column::Description,
                product::Column::Series,
                product::Column::CircleId,
                product::Column::Image,
                product::Column::Actor,
                product::Column::Author,
                product::Column::Illustrator,
                product::Column::Price,
                product::Column::SaleCount,
                product::Column::Age,
                product::Column::ReleasedAt,
                product::Column::Rating,
                product::Column::RatingCount,
                product::Column::CommentCount,
                product::Column::Path,
            ])
            .to_owned(),
    )
    .exec(db)
    .await?;

    let genre_on_conflict = OnConflict::column(genre::Column::Id)
        .update_columns([genre::Column::Id])
        .to_owned();

    for genre in product.genre {
        let product_id = product.id.clone();
        let genre_on_conflict = genre_on_conflict.clone();
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                genre::Entity::insert(genre::ActiveModel {
                    id: Set(genre.id.clone()),
                    name: Set(genre.name.clone()),
                })
                .on_conflict(genre_on_conflict)
                .exec(txn)
                .await?;

                product_genre::Entity::insert(product_genre::ActiveModel {
                    product_id: Set(product_id),
                    genre_id: Set(genre.id),
                })
                .on_conflict(
                    OnConflict::columns([
                        product_genre::Column::ProductId,
                        product_genre::Column::GenreId,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .exec(txn)
                .await?;

                Ok(())
            })
        })
        .await?;
    }

    for (genre, count) in product.reviewer_genre {
        let product_id = product.id.clone();
        let genre_on_conflict = genre_on_conflict.clone();
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                genre::Entity::insert(genre::ActiveModel {
                    id: Set(genre.id.clone()),
                    name: Set(genre.name.clone()),
                })
                .on_conflict(genre_on_conflict)
                .exec(txn)
                .await?;

                product_user_genre::Entity::insert(product_user_genre::ActiveModel {
                    product_id: Set(product_id),
                    genre_id: Set(genre.id),
                    count: Set(count),
                })
                .on_conflict(
                    OnConflict::columns([
                        product_user_genre::Column::ProductId,
                        product_user_genre::Column::GenreId,
                    ])
                    .update_column(product_user_genre::Column::Count)
                    .to_owned(),
                )
                .exec(txn)
                .await?;

                Ok(())
            })
        })
        .await?;
    }

    Ok(())
}

pub async fn delete_product_and_relations(
    db: &DatabaseConnection,
    ids: &Vec<String>,
) -> Result<(), TransactionError<DbErr>> {
    if ids.is_empty() {
        return Ok(());
    }
    db.transaction::<_, (), DbErr>(|txn| {
        let ids = ids.clone();
        Box::pin(async move {
            product_genre::Entity::delete_many()
                .filter(product_genre::Column::ProductId.is_in(ids.clone()))
                .exec(txn)
                .await?;
            product_user_genre::Entity::delete_many()
                .filter(product_user_genre::Column::ProductId.is_in(ids.clone()))
                .exec(txn)
                .await?;
            product::Entity::delete_many()
                .filter(product::Column::Id.is_in(ids))
                .exec(txn)
                .await?;

            Ok(())
        })
    })
    .await?;

    Ok(())
}
