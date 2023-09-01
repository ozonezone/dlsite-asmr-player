use prisma_client_rust::Result;

use crate::{
    db::product_read::{product_detailed, search},
    Db,
};

pub async fn search_product(db: Db, query: String) -> Result<Vec<product_detailed::Data>> {
    let mut words = vec![];
    let mut genres = vec![];
    let mut circles = vec![];
    let mut creators = vec![];

    query.split_whitespace().for_each(|keyword| {
        if let Some((k, v)) = keyword.split_once(':') {
            match k {
                "genre" => genres.push(v.to_string()),
                "circle" => circles.push(v.to_string()),
                "creator" => creators.push(v.to_string()),
                _ => words.push(keyword.to_string()),
            }
        } else {
            words.push(keyword.to_string())
        };
    });

    search(db, words, genres, circles, creators).await
}
