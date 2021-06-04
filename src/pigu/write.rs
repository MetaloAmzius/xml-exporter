use crate::write::calculate_ean_checksum_digit;
use crate::pigu::models::Barcode;
use crate::pigu::models::Attributes;
use crate::Write;
use crate::pigu::models::Colour;
use crate::pigu::models::Modification;
use crate::pigu::models::Product;
use crate::pigu::models::Root;

impl Write for Root {
    fn write(&self) -> std::string::String {
        format!("<root><products>{}</products></root>", self.products.write())
    }
}


impl Write for Product {
    fn write(&self) -> std::string::String {
        format!("<product>
<category-id>{}</category-id>
<category-name>{}</category-name>
<colours>{}</colours>
<title>{}</title>
</product>",
                self.category_id,
                self.category_name,
                self.colours.write(),
                self.title
        )
    }

}

impl Write for Colour {
    fn write(&self) -> std::string::String {
        format!("<colour>
<modifications>{}</modifications>
</colour>",
                self.modifications.write())
    }
}

impl Write for Modification {
    fn write(&self) -> std::string::String {
        format!("<modification>
<attributes>{}</attributes>
<height>{}</height>
<length>{}</length>
<package_barcode>{}{}</package_barcode>
<weight>{}</weight>
<width>{}</width>
</modification>",
                self.attributes.write(),
                self.height,
                self.length,
                self.package_barcode, calculate_ean_checksum_digit(&self.package_barcode),
                self.weight,
                self.width)
    }
}

impl Write for Attributes {
    fn write(&self) -> std::string::String {
        format!("<barcodes>{}</barcodes>
<supplier-code>{}</supplier-code>",
                self.barcodes.write(),
                self.supplier_code
        )
    }
}

impl Write for Barcode {
    fn write(&self) -> std::string::String {
        format!("<barcode>{}{}</barcode>",
                self.barcode, calculate_ean_checksum_digit(&self.barcode))
    }
}
