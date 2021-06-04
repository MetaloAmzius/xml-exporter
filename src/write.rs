use crate::models::Image;

pub trait Write {
    fn write(&self) -> String;
}

impl<T> Write for Vec<T>
where T: Write
{
    fn write(&self) -> std::string::String {
        self.iter()
            .map(|p| p.write())
            .collect::<Vec<String>>()
            .join("").to_string()
    }
}

impl Write for Image {
    fn write(&self) -> std::string::String {
        format!("<image>{}</image>", self.data)
    }
}
