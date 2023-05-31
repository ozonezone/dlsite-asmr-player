use sea_orm::{Linked, RelationDef, RelationTrait};

use crate::entities::product_genre;

#[derive(Debug)]
pub struct ProductToGenreViaProductGenre;

impl Linked for ProductToGenreViaProductGenre {
    type FromEntity = crate::entities::product::Entity;

    type ToEntity = crate::entities::genre::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            product_genre::Relation::Product.def().rev(),
            product_genre::Relation::Genre.def(),
        ]
    }
}
