use crate::Database;
use crate::models::Category;
use models::Product;
use models::Root;

mod database;
mod models;

pub fn load(db: &Database) -> Root {
    Root {
        categories: Category::load_all(db),
        products: Product::load_all(db),
    }
}
