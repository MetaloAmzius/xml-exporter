pub mod database;
pub mod models;
pub mod write;

use crate::database::*;
use crate::models::Category;
use models::Root;

pub fn load(db: &Database) -> Root {
    Root {
        categories: Category::load_all(db),
        products: vec![],
    }
}
