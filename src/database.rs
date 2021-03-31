use log::warn;
use postgres::Client;
use postgres::NoTls;
use super::models::CData;
use super::models::Category;
use super::models::Image;

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


pub fn get_product_categories(db: &Database, id: i32) -> Vec<i32> {
    let mut client = Client::connect(&db.connection_string, NoTls).unwrap();
    let mut categories = Vec::new();
    for row in client
        .query(
            "
select pcr.category_id
from product_categories_relations pcr
inner join categories c on c.id = pcr.category_id
where product_id = $1;
",
            &[&id],
        )
        .unwrap()
    {
        categories.push(match row.get(0){
            Some(val) => val,
            None => panic!("Failed to read categories id, value was null")
        });
    }

    categories
}

pub fn get_product_images(db: &Database, id: i32) -> Vec<Image> {
    let mut client = Client::connect(&db.connection_string, NoTls).unwrap();
    let mut images = Vec::new();
    for row in client
        .query(
            "
select image from images
where product_id = $1;
",
            &[&id],
        )
        .unwrap()
    {
        use regex::Regex;
        //{"FileName":"24802260-4004.jpg","Url":"/system/images/36723/image/24802260-4004.20200811133336957856.jpg"}
        let re = Regex::new(r#".*"Url":"(.*)".*"#).unwrap();
        match row.get(0) {
            Some(image_json) => {
                for cap in re.captures_iter(image_json) {
                    if cap.len() != 2 {
                        continue;
                    }
                    images.push(Image {
                        data: format!("https://metaloamzius.lt{}", cap[1].to_string()),
                    });
                }
            }
            None => {
                warn!("Product has no image: {}", id);
            }
        }
    }

    images
}

pub fn get_product_manufacturer(db: &Database, id: i32) -> Option<CData> {
    let mut client = Client::connect(&db.connection_string, NoTls).unwrap();
    for row in client
        .query(
            "
select p.id, c.name from products p
inner join product_categories_relations pcr on p.id = pcr.product_id
inner join categories c on pcr.category_id = c.id
where p.id = $1 and c.category_id = 851;
",
            &[&id],
        )
        .unwrap()
    {
        return Some(CData { data: match row.get(1) {
            Some(val) => val,
            None => panic!("Failed to read category name, value was null")
        }});
    }

    None
}

pub fn get_product_quantity(db: &Database, id: i32) -> i64 {
    let mut client = Client::connect(&db.connection_string, NoTls).unwrap();
    for row in client
        .query(
            "
select sum(count) from product_remainers
where product_id = $1;
",
            &[&id],
        )
        .unwrap()
    {
        return match row.get(0) {
            Some(count) => count,
            None => {
                warn!("Failed to get count for product {}", id);
                0
            }
        };
    }

    panic!("No quantity from database")
}
