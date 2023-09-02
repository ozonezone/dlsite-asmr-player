use prisma_client_rust::queries::Result as DbResult;

use crate::{prisma::product, Db};

use super::product_detailed;

pub async fn get_product(db: Db, product_id: String) -> DbResult<Option<product_detailed::Data>> {
    db.product()
        .find_unique(product::id::equals(product_id))
        .include(product_detailed::include())
        .exec()
        .await
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
