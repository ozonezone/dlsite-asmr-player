use std::path::PathBuf;

use chrono::{DateTime, FixedOffset, NaiveDateTime, NaiveTime};
use dlsite::product::Product;
use prisma_client_rust::queries::Result;

use crate::{
    prisma::{circle, creator, genre, product, product_creator, product_genre, product_user_genre},
    Db,
};

impl From<dlsite::interface::AgeCategory> for crate::prisma::AgeCategory {
    fn from(value: dlsite::interface::AgeCategory) -> Self {
        match value {
            dlsite::interface::AgeCategory::General => Self::General,
            dlsite::interface::AgeCategory::R15 => Self::R15,
            dlsite::interface::AgeCategory::Adult => Self::Adult,
        }
    }
}

#[tracing::instrument(err, skip_all)]
pub async fn upsert_product(db: Db, product: Product, path: PathBuf) -> Result<()> {
    db.circle()
        .upsert(
            circle::id::equals(product.circle_id.clone()),
            circle::create(
                product.circle_id.clone(),
                product.circle_name.clone(),
                vec![],
            ),
            vec![
                circle::id::set(product.circle_id.clone()),
                circle::name::set(product.circle_name),
            ],
        )
        .exec()
        .await?;

    let released_at = DateTime::from_local(
        NaiveDateTime::new(
            product.released_at,
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        ),
        FixedOffset::east_opt(9 * 3600).unwrap(),
    );
    db.product()
        .upsert(
            product::id::equals(product.id.clone()),
            product::create(
                product.id.clone(),
                product.title.clone(),
                circle::id::equals(product.circle_id.clone()),
                product.price,
                product.sale_count.unwrap_or(0),
                product.age_rating.clone().into(),
                released_at,
                product.rate_count.unwrap_or(0),
                product.review_count.unwrap_or(0),
                path.to_string_lossy().to_string(),
                vec![
                    product::series::set(product.series.clone()),
                    product::rating::set(product.rating.map(|r| r.into())),
                    product::images::set(product.images.iter().map(|u| u.to_string()).collect()),
                ],
            ),
            vec![
                product::id::set(product.id.clone()),
                product::title::set(product.title.clone()),
                product::circle_id::set(product.circle_id),
                product::price::set(product.price),
                product::sale_count::set(product.sale_count.unwrap_or(0)),
                product::age::set(product.age_rating.into()),
                product::released_at::set(released_at),
                product::rate_count::set(product.rate_count.unwrap_or(0)),
                product::review_count::set(product.review_count.unwrap_or(0)),
                product::path::set(path.to_string_lossy().to_string()),
                product::series::set(product.series),
                product::rating::set(product.rating.map(|r| r.into())),
                product::images::set(product.images.into_iter().map(|u| u.to_string()).collect()),
            ],
        )
        .exec()
        .await?;

    macro_rules! upsert_people {
        ($key:ident, $role:ident) => {
            if let Some(v) = product.people.$key {
                for value in v {
                    db._batch((
                        db.creator().upsert(
                            creator::name::equals(value.clone()),
                            creator::create(value.clone(), vec![]),
                            vec![creator::name::set(value.clone())],
                        ),
                        db.product_creator().upsert(
                            product_creator::product_id_creator_name(
                                product.id.clone(),
                                value.clone(),
                            ),
                            product_creator::create(
                                product::id::equals(product.id.clone()),
                                creator::name::equals(value.clone()),
                                crate::prisma::CreatorRole::$role,
                                vec![],
                            ),
                            vec![
                                product_creator::product_id::set(product.id.clone()),
                                product_creator::creator_name::set(value.clone()),
                                product_creator::role::set(crate::prisma::CreatorRole::$role),
                            ],
                        ),
                    ))
                    .await?;
                }
            }
        };
    }

    upsert_people!(author, Creator);
    upsert_people!(scenario, ScenarioWriter);
    upsert_people!(illustrator, Illustrator);
    upsert_people!(voice_actor, VoiceActor);

    for genre in product.genre {
        let product_id = product.id.clone();
        db._batch((
            db.genre().upsert(
                genre::id::equals(genre.id.clone()),
                genre::create(genre.id.clone(), genre.name.clone(), vec![]),
                vec![
                    genre::id::set(genre.id.clone()),
                    genre::name::set(genre.name),
                ],
            ),
            db.product_genre().upsert(
                product_genre::product_id_genre_id(product_id.clone(), genre.id.clone()),
                product_genre::create(
                    product::id::equals(product_id.clone()),
                    genre::id::equals(genre.id.clone()),
                    vec![],
                ),
                vec![
                    product_genre::product_id::set(product_id),
                    product_genre::genre_id::set(genre.id.clone()),
                ],
            ),
        ))
        .await?;
    }

    for (genre, count) in product.reviewer_genre {
        let product_id = product.id.clone();
        db._batch((
            db.genre().upsert(
                genre::id::equals(genre.id.clone()),
                genre::create(genre.id.clone(), genre.name.clone(), vec![]),
                vec![
                    genre::id::set(genre.id.clone()),
                    genre::name::set(genre.name),
                ],
            ),
            db.product_user_genre().upsert(
                product_user_genre::product_id_genre_id(product_id.clone(), genre.id.clone()),
                product_user_genre::create(
                    product::id::equals(product_id.clone()),
                    genre::id::equals(genre.id.clone()),
                    count,
                    vec![],
                ),
                vec![
                    product_user_genre::product_id::set(product_id),
                    product_user_genre::genre_id::set(genre.id.clone()),
                    product_user_genre::count::set(count),
                ],
            ),
        ))
        .await?;
    }

    Ok(())
}

pub async fn delete_product_and_relations(db: Db, ids: &Vec<String>) -> Result<i64> {
    if ids.is_empty() {
        return Ok(0);
    }
    db.product()
        .delete_many(
            ids.iter()
                .map(|id| product::id::equals(id.clone()))
                .collect::<Vec<_>>(),
        )
        .exec()
        .await
}
