use crate::models::Image;

pub trait Write {
    fn write(&self) -> String;
}

impl Write for Vec<Image> {
    fn write(&self) -> std::string::String {
        format!(
            "<images>{}</images>",
            self.iter()
                .map(|i| i.write())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl Write for Image {
    fn write(&self) -> std::string::String {
        format!("<image>{}</image>", self.data)
    }
}
