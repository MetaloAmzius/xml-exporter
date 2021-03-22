use crate::Write;
use super::models::*;

impl Write for Root {
    fn write(&self) -> std::string::String {
        format!(
            "<root>{}{}</root>",
            Write::write(&self.categories),
            Write::write(&self.products)
        )
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
        format!(
            "<product>
<id>{}</id>
<title>{}</title>
<description>{}</description>
<warranty/>
<weight/>
<manufacturer>{}</manufacturer>
{}
<sku>{}</sku>
<quantity>{}</quantity>
<price>{}</price>
<price_old>{}</price_old>{}
{}
</product>\n",
            self.id,
            self.title.write(),
            self.description.write(),
            self.manufacturer.write(),
            self.images.write(),
            self.ty.write(),
            self.sku,
            self.quantity,
            self.price,
            self.price_old,
            self.attributes.write()
        )
    }
}
