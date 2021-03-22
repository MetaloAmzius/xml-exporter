use crate::models::Category;

pub trait Write {
    fn write(&self) -> String;
}

impl Write for Vec<Category> {
    fn write(&self) -> std::string::String {
        format!(
            "<categories>{}</categories>",
            self.iter()
                .map(|cat| cat.write())
                .collect::<Vec<String>>()
                .join("")
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
