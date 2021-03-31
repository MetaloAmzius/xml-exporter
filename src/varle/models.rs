use crate::models::CData;
use crate::models::Category;
use crate::models::Image;

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

pub struct Root {
    pub categories: Vec<Category>,
    pub products: Vec<Product>,
}

#[derive(Clone, Debug)]
pub struct Product {
    pub url: String,
    pub id: String,
    pub title: CData,
    pub description: CData,
    pub categories: Vec<i32>,
    pub manufacturer: Option<CData>,
    pub warranty: Option<String>,
    pub attributes: Vec<Attribute>,
    pub sku: String,
    pub quantity: i64,
    pub price: String,
    pub price_old: String,
    pub prime_costs: String,
    pub images: Vec<Image>,
    pub weight: Option<String>,
}
