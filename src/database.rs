pub trait Loadable {
    fn load_all(db: Database) -> Vec<Self>
        where Self: Sized;
}

pub struct Database {
    pub connection_string: String,
}

impl Database {
    pub fn new(connection_string: &str) -> Database {
        Database {
            connection_string: connection_string.to_string(),
        }
    }
}
