use super::models::Attribute;
use super::models::Category;
use super::models::Product;
use super::models::Root;
use crate::database::*;
use crate::models::CData;
use log::error;
use log::warn;
use postgres::Client;
use postgres::NoTls;
use rust_decimal::Decimal;

pub fn load(db: &Database) -> Root {
    Root {
        categories: Category::load_all(db),
        products: Product::load_all(db),
    }
}

impl Loadable for Product {
    fn load_all(db: &Database) -> Vec<Self> {
        let mut client = Client::connect(&db.connection_string, NoTls).unwrap();
        let mut products = Vec::new();
        for row in client
            .query(
                "
select * from (
--Get all child products
    select c.id,
           concat('https://metaloamzius.lt', c.name_with_slug) as url,
           c.price,
           c.price as price_old,
           c.sku,
           c.name as title,
           c.description,
           cast(cast(c.price as decimal) / 1.21 / 1.3 as text) as prime_cost,
           c.barcode
      from products p
inner join products c on p.id = c.parent_id
     where c.active = 't'

 union all
--Get all child-less products
    select p.id,
           concat('https://metaloamzius.lt', p.name_with_slug) as url,
           p.price,
           p.price as price_old,
           p.sku,
           p.name as title,
           p.description,
           cast(cast(p.price as decimal) / 1.21 / 1.3 as text) as prime_cost,
           p.barcode
      from products p
 left join products c on p.id = c.parent_id
     where c.id is null
       and (p.parent_id is null or p.parent_id = 0)
       and p.active = 't'
) p
where not exists (select null
                    from product_categories_relations
                   where category_id = 1628 and product_id = p.id) --Exclude ASBIS
  and not exists (select null
                    from product_categories_relations
                   where category_id = 1639 and product_id = p.id) --Exclude Style De Vie
  and not exists (select null
                    from product_categories_relations
                   where category_id = 1237 and product_id = p.id); --Exclude Westmark
",
                &[],
            )
            .unwrap()
        {
            let id = match row.get(0) {
                Some(val) => val,
                None => panic!("Failed to read product_id, value was null"),
            };
            products.push(Product {
                url: match row.get(1) {
                    Some(val) => val,
                    None => panic!("Failed to read product url, value was null"),
                },
                id: match row.get(4) {
                    Some(val) => val,
                    None => {
                        error!("Failed to read product sku, value was null: {}", &id);
                        continue;
                    }
                },
                title: CData {
                    data: match row.get(5) {
                        Some(val) => val,
                        None => panic!("Failed to read product title, value was null"),
                    },
                },
                description: CData {
                    data: match row.get(6) {
                        Some(result) => result,
                        None => {
                            warn!("Product with no description");
                            "".to_string()
                        }
                    },
                },
                categories: get_product_categories(db, id),
                manufacturer: get_product_manufacturer(db, id),
                warranty: None,
                attributes: get_product_attributes(db, id),
                sku: match row.get(4) {
                    Some(val) => val,
                    None => panic!("Failed to read product sku, value was null"),
                },
                quantity: get_product_quantity(db, id),
                price: row.get::<usize, Option<Decimal>>(2).unwrap_or_else(|| {
                    warn!("Product ({}) with no price", id);
                    Decimal::ZERO
                }),
                price_old: match row.get(3) {
                    Some(result) => result,
                    None => {
                        warn!("Product ({}) with no old_price", id);
                        Decimal::ZERO
                    }
                },
                prime_costs: match row.get(7) {
                    Some(result) => result,
                    None => {
                        warn!("Product ({}) with no prime_costs", id);
                        "".to_string()
                    }
                },
                images: get_product_images(db, id),
                weight: None,
                barcode: row.get(8),
            })
        }
        products
    }
}

pub fn get_product_attributes(db: &Database, id: i32) -> Vec<Attribute> {
    let mut client = Client::connect(&db.connection_string, NoTls).unwrap();
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
            name: insert_escaped_characters(match row.get(0) {
                Some(val) => val,
                None => panic!("Failed to read attributes key, value was null"),
            }),
            value: insert_escaped_characters(match row.get(1) {
                Some(val) => val,
                None => {
                    error!(
                        "Failed to read attributes value (null value) for product id: {}",
                        id
                    );
                    continue;
                }
            }),
        })
    }
    attributes
}

pub fn insert_escaped_characters(val: String) -> String {
    let re = regex::Regex::new("&#x[0-9]{2,4};").unwrap();
    // println!("{:?}", &val);
    if re.is_match(&val) {
        println!("{:?}", &val);
    }

    val
}

impl Loadable for Category {
    fn load_all(db: &Database) -> Vec<Self> {
        let mut client = Client::connect(&db.connection_string, NoTls).unwrap();
        let mut categories = Vec::new();
        for row in client
            .query("select id, category_id, name from categories;", &[])
            .unwrap()
        {
            let mut result = Category {
                id: match row.get(0) {
                    Some(val) => val,
                    None => panic!("Failed to read Category ID, value was null"),
                },
                parent_id: match row.get(1) {
                    Some(val) => val,
                    None => {
                        error!("Failed to read Category parent_id, value was null... Skipping");
                        continue;
                    }
                },
                name: CData {
                    data: match row.get(2) {
                        Some(val) => val,
                        None => {
                            error!("Failed to read Category name, value was null... Skipping");
                            continue;
                        }
                    },
                },
            };

            if result.parent_id == Some(0) {
                result.parent_id = None;
            }

            if result.id != 0 {
                categories.push(result);
            }
        }

        categories
    }
}
