use postgres::Client;
use postgres::NoTls;
use super::models::CData;
use super::models::Category;

pub trait Loadable {
    fn load_all(db: &Database) -> Vec<Self>
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

impl Loadable for Category {
    fn load_all(db: &Database) -> Vec<Self> {
        let mut client = Client::connect(&db.connection_string, NoTls).unwrap();
        let mut categories = Vec::new();
        for row in client
            .query("select id, category_id, name from categories;", &[])
            .unwrap()
        {
            categories.push(Category {
                id: match row.get(0)
                {
                    Some(val) => val,
                    None => panic!("Failed to read Category ID, value was null")
                },
                parent_id: match row.get(1){
                    Some(val) => val,
                    None => panic!("Failed to read Category parent_id, value was null")
                },
                name: CData { data: match row.get(2) {
                    Some(val) => val,
                    None => panic!("Failed to read Category name, value was null")
                }},
            })
        }

        categories
    }
}
