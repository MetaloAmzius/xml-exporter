use crate::models::CData;
use either::Either;
use crate::models::Category;

pub struct Root {
    pub categories: Vec<Category>,
    pub products: Vec<Product>,
}

#[derive(Clone, Debug)]
pub struct Image {
    pub data: String,
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub value: CData,
}

#[derive(Clone, Debug)]
pub struct VariantProduct {
    pub sku: Option<String>,
    pub variants: Vec<SimpleProduct>,
    pub quantity: i64,
}
#[derive(Clone, Debug)]
pub struct SimpleProduct {
    pub attributes: Vec<Attribute>,
    pub sku: String,
    pub quantity: i64,
    pub price: String,
    pub price_old: String,
}

#[derive(Clone, Debug)]
pub struct Product {
    pub url: String,
    pub id: i32,
    pub title: CData,
    pub description: CData,
    pub categories: Vec<i32>,
    pub manufacturer: Option<CData>,
    pub warranty: Option<String>,
    pub ty: Either<SimpleProduct, VariantProduct>,
    pub weight: Option<String>,
    pub images: Vec<Image>,
}
