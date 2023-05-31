use std::path::PathBuf;

use chrono::Datelike;
use dlsite::product::Product;
use entity::entities::{circle, product, product_genre, product_user_genre};
use migration::{Expr, PgFunc};
use sea_orm::{
    DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set, TransactionError, TransactionTrait,
};
use tracing::error;

pub async fn create_product(
    pool: &DatabaseConnection,
    product: Product,
    path: PathBuf,
) -> Result<(), anyhow::Error> {
    circle::Entity::insert(circle::ActiveModel {
        id: Set(product.circle.id.clone()),
        name: Set(product.circle.name),
    })
    .exec(pool)
    .await?;

    upsert_product()
        .params(
            &client,
            &UpsertProductParams {
                id: product.id.clone(),
                name: product.title,
                description: None::<&str>,
                series: product.series,
                circle_id: product.circle.id,
                remote_image: product
                    .images
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>(),
                actor: product.people.voice_actor.unwrap_or_default(),
                author: product.people.author.unwrap_or_default(),
                illustrator: product.people.illustrator.unwrap_or_default(),
                price: product.price,
                sale_count: product.sale_count,
                age: product.age_rating.into(),
                // convert chrono date to "time" crate date
                released_at: time::Date::from_calendar_date(
                    product.released_at.year(),
                    time::Month::try_from(u8::try_from(product.released_at.month()).unwrap())
                        .unwrap(),
                    product.released_at.day().try_into().unwrap(),
                )
                .unwrap(),
                rating: product.rating,
                rating_count: product.rate_count.unwrap_or(0),
                comment_count: product.review_count.unwrap_or(0),
                path: path.to_string_lossy(),
            },
        )
        .await?;

    for genre in product.genre {
        let transaction = client.transaction().await?;
        upsert_genre()
            .params(
                &transaction,
                &UpsertGenreParams {
                    id: genre.id.clone(),
                    name: genre.name.clone(),
                },
            )
            .await?;
        insert_product_genre()
            .params(
                &transaction,
                &InsertProductGenreParams {
                    product_id: product.id.clone(),
                    genre_id: genre.id,
                },
            )
            .await?;

        transaction.commit().await.map_err(|e| {
            error!("Could not commit transaction: {}", e);
            e
        })?;
    }

    for (genre, count) in product.reviewer_genre {
        let transaction = client.transaction().await?;
        upsert_genre()
            .params(
                &transaction,
                &UpsertGenreParams {
                    id: genre.id.clone(),
                    name: genre.name.clone(),
                },
            )
            .await?;
        upsert_product_usergenre()
            .params(
                &transaction,
                &UpsertProductUsergenreParams {
                    product_id: product.id.clone(),
                    genre_id: genre.id.clone(),
                    count,
                },
            )
            .await?;
        transaction.commit().await.map_err(|e| {
            error!("Could not commit transaction: {}", e);
            e
        })?;
    }

    Ok(())
}

pub async fn delete_product_and_relations(
    pool: &DatabaseConnection,
    ids: &Vec<String>,
) -> Result<(), TransactionError<DbErr>> {
    pool.transaction::<_, (), DbErr>(|txn| {
        let ids = ids.clone();
        Box::pin(async move {
            product_genre::Entity::delete_many()
                .filter(Expr::eq(
                    Expr::val(ids.clone()),
                    Expr::expr(PgFunc::any(Expr::col(product_genre::Column::ProductId))),
                ))
                .exec(txn)
                .await?;
            product_user_genre::Entity::delete_many()
                .filter(Expr::eq(
                    Expr::val(ids.clone()),
                    Expr::expr(PgFunc::any(Expr::col(
                        product_user_genre::Column::ProductId,
                    ))),
                ))
                .exec(txn)
                .await?;
            product::Entity::delete_many()
                .filter(Expr::eq(
                    Expr::val(ids),
                    Expr::expr(PgFunc::any(Expr::col(product::Column::Id))),
                ))
                .exec(txn)
                .await?;

            Ok(())
        })
    })
    .await?;

    Ok(())
}
