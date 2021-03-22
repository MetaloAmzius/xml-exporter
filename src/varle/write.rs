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
        format!("")
    }
}
