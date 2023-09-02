use dlsite::interface::AgeCategory;
use prisma_client_rust::{and, operator::or, or, queries::Result as DbResult};

use crate::{
    prisma::{circle, creator, genre, product, product_creator, product_genre},
    Db,
};

product::include!(product_detailed {
    circle
    genres: include {
        genre
    }
    user_genres: include {
        genre
    }
    creators
});

pub async fn get_product(db: Db, product_id: String) -> DbResult<Option<product_detailed::Data>> {
    db.product()
        .find_unique(product::id::equals(product_id))
        .include(product_detailed::include())
        .exec()
        .await
}

pub async fn browse(
    db: Db,
    page: i32,
    limit: i32,
    order: <product::Types as prisma_client_rust::ModelTypes>::OrderBy,
) -> DbResult<(Vec<product_detailed::Data>, i64)> {
    let products = db
        .product()
        .find_many(vec![])
        .skip(((page - 1) * limit).into())
        .order_by(order)
        .take(limit.into())
        .include(product_detailed::include())
        .exec()
        .await?;

    let count = db.product().count(vec![]).exec().await?;

    Ok((products, count))
}

product::select!(product_only_path { path });

pub async fn get_product_folder(db: Db, product_id: String) -> DbResult<Option<String>> {
    let path = db
        .product()
        .find_unique(product::id::equals(product_id))
        .select(product_only_path::select())
        .exec()
        .await?
        .map(|p| p.path);

    Ok(path)
}

product::select!(product_only_id { id });

pub async fn get_product_ids(db: Db) -> DbResult<Vec<String>> {
    let ids = db
        .product()
        .find_many(vec![])
        .select(product_only_id::select())
        .exec()
        .await?
        .into_iter()
        .map(|d| d.id)
        .collect::<Vec<_>>();

    Ok(ids)
}

pub async fn search(
    db: Db,
    words: Vec<String>,
    genres: Vec<String>,
    circles: Vec<String>,
    creators: Vec<String>,
    age_category: Option<AgeCategory>,
) -> DbResult<Vec<product_detailed::Data>> {
    dbg!(&words, &genres, &circles, &creators);

    // TODO: Make and search works
    let search_query = words
        .into_iter()
        .map(|word| product::title::contains(word))
        .chain(genres.into_iter().map(|genre| {
            product::genres::some(vec![product_genre::genre::is(vec![genre::name::equals(
                genre,
            )])])
        }))
        .chain(
            circles
                .into_iter()
                .map(|circle| product::circle::is(vec![circle::name::equals(circle)])),
        )
        .chain(creators.into_iter().map(|creator| {
            product::creators::some(vec![product_creator::creator::is(vec![
                creator::name::equals(creator),
            ])])
        }))
        .chain(
            std::iter::once(
                age_category.map(|age_category| product::age::equals(age_category.into())),
            )
            .flatten(),
        )
        .collect::<Vec<_>>();
    db.product()
        .find_many(search_query)
        .include(product_detailed::include())
        .exec()
        .await
}
