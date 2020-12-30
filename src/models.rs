use either::Either;

pub struct Root {
    pub categories: Vec<Category>,
    pub products: Vec<Product>,
}

pub struct CData {
    pub data: String,
}

pub struct Category {
    pub id: i32,
    pub parent_id: i32,
    pub name: CData,
}

#[derive(Debug)]
pub struct Image {
    pub data: String,
}

pub struct Variant {
    _attributes: Vec<VariantAttribute>,
    _price: String,
    _quantity: i32,
    _sku: String,
}

pub struct VariantAttribute {}

pub struct VariantProduct {
    pub sku: Option<String>,
    pub variants: Vec<Variant>,
    pub quantity: Option<i32>,
}

pub struct SimpleProduct {
    pub sku: String,
    pub quantity: i64,
}

pub struct Product {
    pub url: String,
    pub id: i32,
    pub title: CData,
    pub description: CData,
    pub price: String,
    pub price_old: String,

    pub categories: Vec<i32>,
    pub manufacturer: Option<CData>,
    pub warranty: Option<String>,
    pub ty: Either<SimpleProduct, VariantProduct>,
    pub weight: Option<String>,
    pub images: Vec<Image>,
}
