use crate::Database;
use super::models::Root;
use super::models::Category;
use super::models::Product;

mod database;
mod models;

pub fn load(db: &Database) -> Root {
    Root {
        categories: Category::load_all(db),
        products: Product::load_all(db),
    }
}
