use std::path::PathBuf;

use chrono::Datelike;
use cornucopia_async::Params;
use deadpool_postgres::Pool;
use dlsite::product::Product;
use tracing::error;

use crate::cornucopia::queries::{
    circle::{upsert_circle, UpsertCircleParams},
    genre::{
        insert_product_genre, upsert_genre, upsert_product_usergenre, InsertProductGenreParams,
        UpsertGenreParams, UpsertProductUsergenreParams,
    },
    product::{
        delete_product, delete_product_genre, delete_product_usergenre, upsert_product,
        UpsertProductParams,
    },
};

pub async fn create_product(
    pool: &Pool,
    product: Product,
    path: PathBuf,
) -> Result<(), anyhow::Error> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    upsert_circle()
        .params(
            &transaction,
            &UpsertCircleParams {
                id: product.circle.id.clone(),
                name: product.circle.name,
            },
        )
        .await?;

    upsert_product()
        .params(
            &transaction,
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
    }

    for (genre, count) in product.reviewer_genre {
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
    }

    transaction.commit().await.map_err(|e| {
        error!("Could not commit transaction: {}", e);
        e
    })?;

    Ok(())
}

pub async fn delete_product_and_relations(
    pool: &Pool,
    ids: &Vec<String>,
) -> Result<(), anyhow::Error> {
    let mut client = pool.get().await.map_err(|e| {
        error!("Could not get client from pool");
        e
    })?;
    let transaction = client.transaction().await.map_err(|e| {
        error!("Could not start transaction");
        e
    })?;

    delete_product_genre().bind(&transaction, ids).await?;
    delete_product_usergenre().bind(&transaction, ids).await?;
    delete_product().bind(&transaction, ids).await?;

    transaction.commit().await.map_err(|e| {
        error!("Could not commit transaction: {}", e);
        e
    })?;

    Ok(())
}
