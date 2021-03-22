use crate::database::Database;
use crate::database::Loadable;
use crate::models::CData;
use either::Left;
use either::Right;
use log::warn;
use postgres::Client;
use postgres::NoTls;
use super::models::Attribute;
use crate::models::Category;
use super::models::Image;
use super::models::Product;
use super::models::Root;

pub fn load(db: &Database) -> Root {
    Root {
        categories: Category::load_all(db),
        products: Product::load_all(db),
    }
}
