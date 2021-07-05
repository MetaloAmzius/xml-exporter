use crate::pigu::models::remainders::Product;
use crate::pigu::models::remainders::Root;
use crate::write::calculate_ean_checksum_digit;
use crate::Write;

impl Write for Product {
    fn write(&self) -> std::string::String {
        format!(
            "
<product>
<sku>{}</sku>
<ean>{}{}</ean>
<price>{}</price>
<stock>{}</stock>
<collection_hours>{}</collection_hours>
</product>",
            self.sku,
            self.ean,
            calculate_ean_checksum_digit(&self.ean),
            self.price,
            self.stock,
            self.collection_hours
        )
    }
}

impl Write for Root {
    fn write(&self) -> std::string::String {
        format!("<products>{}</products>", self.products.write())
    }
}
