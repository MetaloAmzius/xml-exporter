use crate::pigu::models::Product;
use crate::Write;
use crate::pigu::models::Root;

impl Write for Root {
    fn write(&self) -> std::string::String {
        format!("<root>{}</root>", Write::write(&self.products))
    }
}

impl Write for Vec<Product> {
    fn write(&self) -> std::string::String {
        format!(
            "<products>{}</products>",
            self.iter()
                .map(|p| p.write())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl Write for Product {
    fn write(&self) -> std::string::String {
        format!("<product>
<sku>{}</sku>
<barcode>{}</barcode>
{}
<title>{}</title>
<modification>{}</modification>
<description>{}</description>
<weight>{}</weight>
<length>{}</length>
<width>{}</width>
<height>{}</height>
</product>",
                self.sku,
                self.barcode,
                self.images.write(),
                self.title,
                self.modification,
                self.description.write(),
                self.weight,
                self.length,
                self.width,
                self.height
        )
    }

}
