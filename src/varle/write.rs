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
            "<product><id>{}</id>
<categories>{}</categories>
<title>{}</title>
<description>{}</description>
<warranty/>
<weight/>
<manufacturer>{}</manufacturer>
{}
<sku>{}</sku>
<quantity>{}</quantity>
<price>{}</price>
<price_old>{}</price_old>
<prime_costs>{}</prime_costs>
{}
</product>\n",
            self.id,
            self.categories
                .iter()
                .map(|c| format!("<category>{}</category>", c))
                .collect::<Vec<String>>()
                .join(""),
            self.title.write(),
            self.description.write(),
            self.manufacturer.write(),
            self.images.write(),
            self.sku,
            self.quantity,
            self.price,
            self.price_old,
            self.prime_costs,
            self.attributes.write()
        )
    }
}

impl Write for Attribute {
    fn write(&self) -> std::string::String {
        format!(
            r#"<attribute title="{}">{}</attribute>"#,
            self.name,
            self.value
        )
    }
}

impl Write for Vec<Attribute> {
    fn write(&self) -> std::string::String {
        format!(
            "<attributes>{}</attributes>",
            self.iter()
                .map(|a| a.write())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
