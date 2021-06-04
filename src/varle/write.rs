use crate::write::calculate_ean_checksum_digit;
use crate::Write;
use super::models::*;

impl Write for Root {
    fn write(&self) -> std::string::String {
        format!(
            "<root>
<categories>{}</categories>
<products>{}</products>
</root>",
            Write::write(&self.categories),
            Write::write(&self.products)
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
<images>{}<images>
<sku>{}</sku>
<quantity>{}</quantity>
<price>{}</price>
<price_old>{}</price_old>
<prime_costs>{}</prime_costs>
<attributes>{}</attributes>
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
            self.attributes.write(),
            match &self.barcode {
                Some(barcode) => format!("<barcode_format>EAN</barcode_format>\n<barcode>{}{}</barcode>",
                                         barcode, calculate_ean_checksum_digit(barcode)),
                None => "".to_string()
            }

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

impl Write for Category {
    fn write(&self) -> std::string::String {
        format!(
            "<category><id>{}</id>{}<name>{}</name>\n</category>",
            self.id,
            match self.parent_id {
                Some(val) => format!("<parent>{}</parent>", val),
                None => "<parent/>".to_string()
            },
            self.name.write()
        )
    }
}
