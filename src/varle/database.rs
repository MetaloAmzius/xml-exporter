use crate::models::CData;
use crate::database::*;
use crate::models::Category;
use log::warn;
use postgres::Client;
use postgres::NoTls;
use super::models::Product;
use super::models::Root;

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
--Get all child products
    select c.id,
           concat('https://metaloamzius.lt', c.name_with_slug) as url,
           c.price,
           c.price as price_old,
           c.sku,
           c.name as title,
           c.description
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
       p.description
from products p
left join products c on p.id = c.parent_id
where c.id is null
      and (p.parent_id is null or p.parent_id = 0)
      and p.active = 't';
",
                &[],
            )
            .unwrap()
        {
            let id = match row.get(0){
                Some(val) => val,
                None => panic!("Failed to read product_id, value was null")
            };
            products.push(Product {
                url: match row.get(1){
                    Some(val) => val,
                    None => panic!("Failed to read product url, value was null")
                },
                id: match row.get(4){
                    Some(val) => val,
                    None => panic!("Failed to read product sku, value was null")
                },
                title: CData { data: match row.get(5) {
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
                    }
                },
                categories: get_product_categories(db, id),
                manufacturer: get_product_manufacturer(db, id),
                warranty: None,
                attributes: get_product_attributes(db, id),
                sku: match row.get(4){
                    Some(val) => val,
                    None => panic!("Failed to read product sku, value was null")
                },
                quantity: get_product_quantity(db, id),
                price: match row.get(2) {
                    Some(result) => result,
                    None => {
                        warn!("Product ({}) with no price", id);
                        "".to_string()
                    }
                },
                price_old: match row.get(3) {
                    Some(result) => result,
                    None => {
                        warn!("Product ({}) with no old_price", id);
                        "".to_string()
                    },
                },
                images: get_product_images(db, id),
                weight: None,
            })
        }
        products
    }
}
