use rust_decimal::Decimal;

pub struct Product {
    pub sku: String,
    pub ean: String,
    pub price: Decimal,
    pub stock: i64,
    pub collection_hours: i32,
}

pub struct Root {
    pub products: Vec<Product>,
}
