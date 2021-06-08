use rust_decimal::Decimal;

#[derive(Clone, Debug)]
pub struct Attributes {
    pub barcodes: Vec<Barcode>,
    pub supplier_code: String,
}

#[derive(Clone, Debug)]
pub struct Barcode {
    pub barcode: String,
}

#[derive(Clone, Debug)]
pub struct Colour {
    pub images: Vec<Image>,
    pub modifications: Vec<Modification>,
    // pub title: Option<String>,
    // pub title_ee: Option<String>,
    // pub title_lv: Option<String>,
    // pub title_ru: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Dimension {
    pub height: Decimal,
    pub length: Decimal,
    // pub package_barcode: Option<String>,
    pub weight: Decimal,
    pub width: Decimal,
}

#[derive(Clone, Debug)]
pub struct Image {
    pub md5: String,
    pub url: String,
}

#[derive(Clone, Debug)]
pub struct Modification {
    pub attributes: Attributes,
    pub height: Decimal,
    pub length: Decimal,
    // pub multi_dimensions: Option<Vec<Dimension>>,
    // pub package_barcode: String,
    // pub title: Option<String>,
    // pub title_ee: Option<String>,
    // pub title_lv: Option<String>,
    // pub title_ru: Option<String>,
    pub weight: Decimal,
    pub width: Decimal,
}

#[derive(Clone, Debug)]
pub struct Product {
    pub category_id: String,
    pub category_name: String,
    pub colours: Vec<Colour>,
    // pub comments: Option<String>,
    // pub delivery_hours: Option<u32>,
    // pub guarantee: Option<u32>,
    pub long_description: String,
    // pub long_description_ee: Option<String>,
    // pub long_description_lv: Option<String>,
    // pub long_description_ru: Option<String>,
    // pub properties: Option<Vec<Property>>,
    pub title: String,
    // pub title_ee: Option<String>,
    // pub title_lv: Option<String>,
    // pub title_ru: Option<String>,
    // pub video_youtube: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Property {
    pub id: String,
    pub values: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Root {
    pub products: Vec<Product>,
}
