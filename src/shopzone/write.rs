use super::models::Attribute;
use super::models::Category;
use super::models::Product;
use super::models::Root;
use super::models::SimpleProduct;
use super::models::VariantProduct;
use crate::models::*;
use crate::write::Write;
use either::Either;
use either::Left;
use either::Right;

impl Write for Option<CData> {
    fn write(&self) -> std::string::String {
        match self {
            Some(cdata) => cdata.write(),
            None => "".to_string(),
        }
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
<images>{}</images>
{}
</product>\n",
            self.id,
            self.title.write(),
            self.description.write(),
            self.manufacturer.write(),
            self.images.write(),
            self.ty.write(),
        )
    }
}

impl Write for Attribute {
    fn write(&self) -> std::string::String {
        format!(
            r#"<attribute title="{}">{}</attribute>"#,
            self.name,
            self.value.write()
        )
    }
}

impl Write for SimpleProduct {
    fn write(&self) -> std::string::String {
        format!(
            "<sku>{}</sku>
<quantity>{}</quantity>
<price>{}</price>
<price_old>{}</price_old>
<attributes>{}</attributes>",
            self.sku,
            self.quantity,
            self.price,
            self.price_old,
            self.attributes.write()
        )
    }
}

impl Write for VariantProduct {
    fn write(&self) -> std::string::String {
        format!(
            "<sku>{}</sku><quantity_total>{}</quantity_total><variants>{}</variants>",
            match &self.sku {
                Some(sku) => sku,
                None => "",
            },
            self.quantity,
            self.variants
                .iter()
                .map(|p| format!("<variant>{}</variant>", p.write()))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl Write for Either<SimpleProduct, VariantProduct> {
    fn write(&self) -> std::string::String {
        match self {
            Left(simple) => simple.write(),
            Right(variant) => variant.write(),
        }
    }
}

impl Write for CData {
    fn write(&self) -> String {
        format!("<![CDATA[{}]]>", self.data)
    }
}

impl Write for Root {
    fn write(&self) -> std::string::String {
        format!(
            "<root><categories>{}</categories><products>{}</products></root>",
            Write::write(&self.categories),
            Write::write(&self.products)
        )
    }
}

impl Write for Category {
    fn write(&self) -> std::string::String {
        format!(
            "<category><id>{}</id><parent>{}</parent><name>{}</name>\n</category>",
            self.id,
            self.parent_id,
            self.name.write()
        )
    }
}
