use crate::database::get_product_images;
use crate::database::Loadable;
use crate::pigu::models::root::Attributes;
use crate::pigu::models::root::Barcode;
use crate::pigu::models::root::Colour;
use crate::pigu::models::root::Image;
use crate::pigu::models::root::Modification;
use crate::pigu::models::root::Product;
use crate::pigu::models::root::Property;
use crate::pigu::models::root::Root;
use crate::Database;
use log::info;
use postgres::Client;
use postgres::NoTls;
use rust_decimal::Decimal;

pub fn load(db: &Database, rivile_db: Vec<rivile_client::models::Product>) -> Root {
    let products = Product::load_all(db);
    info!(
        "Loaded {} products from metaloamzius.lt database",
        products.len()
    );

    info!("Applying box sizes filter");

    let products: Vec<Product> = products
        .into_iter()
        .filter_map(|p| {
            match rivile_db.iter().find(|rp| {
                rp.code
                    == p.colours
                        .first()
                        .unwrap()
                        .modifications
                        .first()
                        .unwrap()
                        .attributes
                        .supplier_code
            }) {
                Some(rp) => Some(Product {
                    colours: vec![Colour {
                        modifications: vec![Modification {
                            height: rp.height,
                            length: rp.length,
                            weight: rp.weight,
                            width: rp.width,
                            ..p.colours
                                .first()
                                .unwrap()
                                .modifications
                                .first()
                                .unwrap()
                                .clone()
                        }],
                        ..p.colours.first().unwrap().clone()
                    }],
                    ..p
                }),
                None => None,
            }
        })
        .collect();

    info!("{} products remaining", products.len());
    Root { products }
}

impl Loadable for Product {
    fn load_all(db: &Database) -> Vec<Product> {
        let mut client = Client::connect(&db.connection_string, NoTls).unwrap();
        let mut products = vec![];

        for row in client.query("
    select p.id,
           p.sku,
           pc.id,
           pc.name,
           barcode,
           case when trim(coalesce(p.pigu_lt_name, '')) = ''
                then p.name
                else p.pigu_lt_name
           end as name,
           p.description as modification
      from products p
cross join lateral ( select plc.category_id as id,
                            plc.name
                       from product_categories_relations pcr
                 inner join pigu_lt_categories_local_categories plclc on pcr.category_id = plclc.category_id
                 inner join pigu_lt_categories plc on plc.id = plclc.pigu_lt_category_id
                      where pcr.product_id = p.id
                      limit 1
) pc
     where p.active = 't'
       and p.sku is not null
       and p.barcode is not null
       and not exists (select null
                             from products
                            where parent_id = p.id) --exclude parent products
       and not exists (select null
                        from product_categories_relations
                       where category_id = 1237 and product_id = p.id) --exlude Westmark
       and p.price >= 15; --exclude all that costs little (company loses money)
", &[]).unwrap()
        {
            let id: i32 = row.try_get(0).unwrap();
            let sku: String = row.try_get(1).unwrap();

            products.push(Product {
                category_id: row.try_get::<'_, _, i32>(2).unwrap().to_string(),
                category_name: row.try_get(3).unwrap(),
                colours: vec!{
                    Colour {
                        modifications: vec!{
                            Modification {
                                attributes: Attributes {
                                    barcodes: vec!{ Barcode {
                                        barcode: row.try_get(4).unwrap() }
                                    },
                                    supplier_code: sku,
                                },
                                height: Decimal::new(0, 0),
                                length: Decimal::new(0, 0),
                                weight: Decimal::new(0, 0),
                                width: Decimal::new(0, 0),
                            }
                        },
                        images: get_product_images(db, id).into_iter().map(|i| Image {
                            url: i.data.clone(),
                        }).collect()
                    }
                },
                title: row.try_get(5).unwrap(),
                long_description: row.try_get(6).unwrap(),
                properties: get_product_properties(db, id)
            })
        }
        products
    }
}

pub fn get_product_properties(db: &Database, id: i32) -> Vec<Property> {
    let mut client = Client::connect(&db.connection_string, NoTls).unwrap();
    let mut attributes = Vec::new();
    for row in client
        .query(
            "
select key, title
from product_metadata pm
where attribute_owner_id = $1;",
            &[&id],
        )
        .unwrap()
    {
        attributes.push(Property {
            id: row
                .try_get(0)
                .expect("Failed to read attributes key, value was null"),
            values: vec![match row.try_get(1) {
                Ok(val) => val,
                Err(_) => "".to_string(),
            }],
        })
    }
    attributes
}
