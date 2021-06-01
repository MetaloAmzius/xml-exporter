use rust_decimal::Decimal;
use crate::models::CData;
use crate::models::Image;

#[derive(Clone, Debug)]
pub struct Product {
    pub sku: String,
    pub barcode: String,
    pub images: Vec<Image>,
    pub title: String,
    pub modification: String,
    pub description: CData,

    pub weight: Decimal,
    pub length: Decimal,
    pub width: Decimal,
    pub height: Decimal
}

pub struct Root {
    pub products: Vec<Product>
}
