
#[derive(Clone, Debug)]
pub struct CData {
    pub data: String,
}

pub struct Category {
    pub id: i32,
    pub parent_id: i32,
    pub name: CData,
}

#[derive(Clone, Debug)]
pub struct Image {
    pub data: String,
}
