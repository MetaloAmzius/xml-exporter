use crate::models::CData;
use crate::models::Image;
use either::Either;
use rust_decimal::Decimal;

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub value: CData,
}

pub struct Category {
    pub id: i32,
    pub parent_id: i32,
    pub name: CData,
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

pub struct Root {
    pub categories: Vec<Category>,
    pub products: Vec<Product>,
}

#[derive(Clone, Debug)]
pub struct SimpleProduct {
    pub attributes: Vec<Attribute>,
    pub sku: String,
    pub quantity: i64,
    pub price: Decimal,
    pub price_old: Decimal,
}

#[derive(Clone, Debug)]
pub struct VariantProduct {
    pub sku: Option<String>,
    pub variants: Vec<SimpleProduct>,
    pub quantity: i64,
}
