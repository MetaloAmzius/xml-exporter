use crate::models::Attribute;
use crate::models::Image;
use crate::models::Product;
use crate::models::SimpleProduct;
use crate::models::VariantProduct;
use crate::CData;
use crate::Category;
use crate::Root;
use either::Left;
use either::Right;
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

    fn load_products(&self) -> Vec<Product> {
        let mut products = Vec::new();
        products.extend(self.load_simple_products());
        products.extend(self.load_variant_products());

        products
    }

    fn load_variant_products(&self) -> Vec<Product> {
        let mut products = Vec::new();

        let mut client = Client::connect(&self.connection_string, NoTls).unwrap();
        for row in client
            .query(
                "
select concat('https://metaloamzius.lt/produktas/', p.name_with_slug) as url,
       p.id,
       p.sku,
       p.name as title,
       p.description,
       p.price,
       p.price,
       p.stock_quantity as quantity
from products p
left join products c on p.id = c.parent_id
where c.id is not null and p.active = 't';
",
                &[],
            )
            .unwrap()
        {
            let id: i32 = match row.get(1){
                Some(val) => val,
                None => panic!("Failed to read Product Id, value was null")
            };
            products.push(Product {
                url: match row.get(0){
                    Some(val) => val,
                    None => panic!("Failed to read Product Url, value was null")
                },
                id,
                title: CData { data: match row.get(3) {
                    Some(val) => val,
                    None => panic!("Failed to read Product Title, value was null")
                }},
                description: CData { data: match row.get(4) {
                    Some(val) => val,
                    None => panic!("Failed to read Product Description, value was null")
                }},
                categories: self.get_product_categories(id),
                manufacturer: self.get_product_manufacturer(id),
                warranty: None,

                ty: Right(VariantProduct {
                    sku: row.get(2),
                    variants: self.get_product_variants(id),
                    quantity: self.get_variations_quantity(id),
                }),
                weight: None,
                images: self.get_product_images(id),
            })
        }

        products
    }

    fn get_product_variants(&self, id: i32) -> Vec<SimpleProduct> {
        let mut client = Client::connect(&self.connection_string, NoTls).unwrap();
        let mut result = Vec::new();
        for row in client
            .query(
                "
select cp.price, cp.sku, cp.id, cp.price from products cp
where parent_id = $1;
",
                &[&id],
            )
            .unwrap()
        {
            let id: i32 = match row.get(2){
                Some(val) => val,
                None => panic!("Failed to read product id, value was null")
            };
            result.push(SimpleProduct {
                attributes: self.get_product_attributes(id),
                price: match row.get(0){
                    Some(val) => val,
                    None => panic!("Failed to read product price, value was null")
                },
                price_old: match row.get(0){
                    Some(val) => val,
                    None => panic!("Failed to read product price, value was null")
                },
                quantity: self.get_product_quantity(id),
                sku: match row.get(1){
                    Some(val) => val,
                    None => panic!("Failed to read product sku, value was null")
                },
            })
        }
        result
    }

    fn get_variations_quantity(&self, id: i32) -> i64 {
        let mut client = Client::connect(&self.connection_string, NoTls).unwrap();
        for row in client
            .query(
                "
select sum(pr.count)
from products cp
inner join product_remainers pr on pr.product_id = cp.id
where parent_id = $1;
",
                &[&id],
            )
            .unwrap()
        {
            return match row.get(0){
                Some(val) => val,
                None => {
                    warn!("Failed to read remainders count, value was null: {}", id);
                    0
                }
            };
        }
        panic!("No Count")
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
            let id = match row.get(1){
                Some(val) => val,
                None => panic!("Failed to read product_id, value was null")
            };
            products.push(Product {
                url: match row.get(0){
                    Some(val) => val,
                    None => panic!("Failed to read product url, value was null")
                },
                id,
                ty: Left(SimpleProduct {
                    attributes: self.get_product_attributes(id),
                    price: match row.get(6) {
                        Some(result) => result,
                        None => {
                            warn!("Product with no price");
                            "".to_string()
                        }
                    },
                    price_old: match row.get(7){
                        Some(val) => val,
                        None => panic!("Failed to read product price, value was null")
                    },
                    sku: match row.get(2){
                        Some(val) => val,
                        None => panic!("Failed to read product price, value was null")
                    },
                    quantity: self.get_product_quantity(id),
                }),
                categories: self.get_product_categories(id),
                title: CData { data: match row.get(4) {
                    Some(val) => val,
                    None => panic!("Failed to read product title, value was null")
                }},
                description: CData {
                    data: match row.get(5) {
                        Some(result) => result,
                        None => {
                            warn!("Product with no description");
                            "".to_string()
                        }
                    },
                },
                warranty: row.get(9),
                weight: row.get(10),
                manufacturer: self.get_product_manufacturer(id),
                images: self.get_product_images(id),
            })
        }
        products
    }

    fn get_product_attributes(&self, id: i32) -> Vec<Attribute> {
        let mut client = Client::connect(&self.connection_string, NoTls).unwrap();
        let mut attributes = Vec::new();
        for row in client
            .query(
                "
select distinct key, title
from product_metadata pm
where attribute_owner_id = $1;",
                &[&id],
            )
            .unwrap()
        {
            attributes.push(Attribute {
                name: match row.get(0){
                    Some(val) => val,
                    None => panic!("Failed to read attributes key, value was null")
                },
                value: CData { data: match row.get(1){
                    Some(val) => val,
                    None => panic!("Failed to read attributes value, value was null")
                }},
            })
        }
        attributes
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
            categories.push(match row.get(0){
                Some(val) => val,
                None => panic!("Failed to read categories id, value was null")
            });
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
            return Some(CData { data: match row.get(1) {
                Some(val) => val,
                None => panic!("Failed to read category name, value was null")
            }});
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
