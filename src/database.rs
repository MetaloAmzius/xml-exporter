use crate::models::Image;
use crate::models::Product;
use crate::models::SimpleProduct;
use crate::CData;
use crate::Category;
use crate::Root;
use either::Left;
use log::warn;
use postgres::Client;
use postgres::NoTls;

pub struct Database {
    connection_string: String,
}

impl Database {
    pub fn new(connection_string: &str) -> Database {
        Database {
            connection_string: connection_string.to_string(),
        }
    }

    pub fn load(&self) -> Root {
        Root {
            categories: self.load_categories(),
            products: self.load_products(),
        }
    }

    fn load_categories(&self) -> Vec<Category> {
        println!("{:?}", self.connection_string);
        let mut client = Client::connect(&self.connection_string, NoTls).unwrap();
        let mut categories = Vec::new();
        for row in client
            .query("select id, category_id, name from categories;", &[])
            .unwrap()
        {
            categories.push(Category {
                id: row.get(0),
                parent_id: row.get(1),
                name: CData { data: row.get(2) },
            })
        }

        categories
    }

    fn load_products(&self) -> Vec<Product> {
        let mut products = Vec::new();
        products.extend(self.load_simple_products());

        products
    }

    fn load_simple_products(&self) -> Vec<Product> {
        let mut client = Client::connect(&self.connection_string, NoTls).unwrap();
        let mut products = Vec::new();
        for row in client
            .query(
                "
select concat('https://metaloamzius.lt', p.name_with_slug) as url,
       p.id,
       p.sku,
       null as categories,
       --categories,
       p.name as title,
       p.description,
       p.price,
       p.price,
       p.stock_quantity as quantity,
       null as warranty,
       null as weight,
       null as manufacturer,
       null as images,
       null as attributes
from products p
left join products c on p.id = c.parent_id
where c.id is null and p.active = 't';
",
                &[],
            )
            .unwrap()
        {
            let id = row.get(1);
            products.push(Product {
                url: row.get(0),
                id,
                ty: Left(SimpleProduct {
                    sku: row.get(2),
                    quantity: self.get_product_quantity(id),
                }),
                categories: self.get_product_categories(id),
                title: CData { data: row.get(4) },
                description: CData {
                    data: match row.get(5) {
                        Some(result) => result,
                        None => {
                            warn!("Product with no description");
                            "".to_string()
                        }
                    },
                },
                price: match row.get(6) {
                    Some(result) => result,
                    None => {
                        warn!("Product with no price");
                        "".to_string()
                    }
                },
                price_old: row.get(7),
                warranty: row.get(9),
                weight: row.get(10),
                manufacturer: self.get_product_manufacturer(id),
                images: self.get_product_images(id),
            })
        }
        products
    }

    fn get_product_categories(&self, id: i32) -> Vec<i32> {
        let mut client = Client::connect(&self.connection_string, NoTls).unwrap();
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
            categories.push(row.get(0));
        }

        categories
    }

    fn get_product_manufacturer(&self, id: i32) -> Option<CData> {
        let mut client = Client::connect(&self.connection_string, NoTls).unwrap();
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
            return Some(CData { data: row.get(1) });
        }

        None
    }

    fn get_product_images(&self, id: i32) -> Vec<Image> {
        let mut client = Client::connect(&self.connection_string, NoTls).unwrap();
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

    fn get_product_quantity(&self, id: i32) -> i64 {
        let mut client = Client::connect(&self.connection_string, NoTls).unwrap();
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
}
