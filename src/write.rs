use crate::models::Image;

pub trait Write {
    fn write(&self) -> String;
}

impl<T> Write for Vec<T>
where
    T: Write,
{
    fn write(&self) -> std::string::String {
        self.iter()
            .map(|p| p.write())
            .collect::<Vec<String>>()
            .join("")
            .to_string()
    }
}

impl Write for Image {
    fn write(&self) -> std::string::String {
        format!("<image>{}</image>", self.data)
    }
}

pub fn calculate_ean_checksum_digit(barcode: &str) -> u32 {
    let mut alternator = 3;
    (10 - (barcode
        .chars()
        .map(|c| {
            alternator = match alternator {
                1 => 3,
                3 => 1,
                _ => 3,
            };
            c.to_digit(10).unwrap() * alternator
        })
        .sum::<u32>()
        % 10))
        % 10
}
