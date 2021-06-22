use crate::pigu::models::root::Property;
use crate::pigu::models::root::Attributes;
use crate::pigu::models::root::Barcode;
use crate::pigu::models::root::Colour;
use crate::pigu::models::root::Image;
use crate::pigu::models::root::Modification;
use crate::pigu::models::root::Product;
use crate::pigu::models::root::Root;
use crate::write::calculate_ean_checksum_digit;
use crate::Write;

impl Write for Property {
    fn write(&self) -> std::string::String {
        format!(
            "<property>
<id>{}</id>
<values>{}</values>
</property>",
            self.id,
            self.values
                .iter()
                .map(|v| format!("<value>{}</value>", v))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Write for Attributes {
    fn write(&self) -> std::string::String {
        format!(
            "<barcodes>{}</barcodes>
<supplier-code><![CDATA[{}]]></supplier-code>",
            self.barcodes.write(),
            self.supplier_code,
        )
    }
}

impl Write for Barcode {
    fn write(&self) -> std::string::String {
        format!(
            "<barcode><![CDATA[{}{}]]></barcode>",
            self.barcode,
            calculate_ean_checksum_digit(&self.barcode)
        )
    }
}

impl Write for Colour {
    fn write(&self) -> std::string::String {
        format!(
            "<colour>
<modifications>{}</modifications>
<images>{}</images>
</colour>",
            self.modifications.write(),
            self.images.write()
        )
    }
}

impl Write for Root {
    fn write(&self) -> std::string::String {
        format!("<products>{}</products>", self.products.write())
    }
}

impl Write for Image {
    fn write(&self) -> std::string::String {
        format!(
            "<image>
<md5><![CDATA[{}]]></md5>
<url><![CDATA[{}]]></url>
</image>",
            self.md5, self.url,
        )
    }
}

impl Write for Modification {
    fn write(&self) -> std::string::String {
        format!(
            "<modification>
<attributes>{}</attributes>
<height>{}</height>
<length>{}</length>
<weight>{}</weight>
<width>{}</width>
</modification>",
            self.attributes.write(),
            self.height,
            self.length,
            self.weight,
            self.width
        )
    }
}

impl Write for Product {
    fn write(&self) -> std::string::String {
        format!(
            "<product>
<category-id><![CDATA[{}]]></category-id>
<category-name><![CDATA[{}]]></category-name>
<colours>{}</colours>
<long-description><![CDATA[{}]]></long-description>
<title><![CDATA[{}]]></title>
</product>",
            self.category_id,
            self.category_name,
            self.colours.write(),
            self.long_description,
            self.title,
        )
    }
}
