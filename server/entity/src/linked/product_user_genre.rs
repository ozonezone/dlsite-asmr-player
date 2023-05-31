use sea_orm::{Linked, RelationDef, RelationTrait};

use crate::entities::{product_genre, product_user_genre};

#[derive(Debug)]
pub struct ProductToGenreViaProductUserGenre;

impl Linked for ProductToGenreViaProductUserGenre {
    type FromEntity = crate::entities::product::Entity;

    type ToEntity = crate::entities::genre::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            product_user_genre::Relation::Product.def().rev(),
            product_user_genre::Relation::Genre.def(),
        ]
    }
}
