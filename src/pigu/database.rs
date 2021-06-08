use crate::database::get_product_images;
use crate::database::Loadable;
use crate::pigu::models::Attributes;
use crate::pigu::models::Barcode;
use crate::pigu::models::Colour;
use crate::pigu::models::Image;
use crate::pigu::models::Modification;
use crate::pigu::models::Product;
use crate::pigu::models::Root;
use crate::Database;
use postgres::Client;
use postgres::NoTls;
use rust_decimal::Decimal;

pub fn load(db: &Database, rivile_db: Vec<rivile_client::models::Product>) -> Root {
    Root {
        products: Product::load_all(db)
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
            .collect(),
    }
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
           p.name,
           pm.title,
           p.description as modification
      from products p
inner join product_metadata pm on p.id = pm.attribute_owner_id
                                and pm.key in ('Tūris', 'Talpa', 'Diametras', 'Galia', 'Skersmuo', 'Dydis')
cross join lateral (    select pcr.product_id, c.* from product_categories_relations pcr
             inner join categories c on pcr.category_id = c.id
                  where pcr.product_id = p.id and c.category_id = 543
               order by c.id desc
                  limit 1) pc
inner join categories c on c.id = pc.category_id
where not exists (select null
                    from products
                   where parent_id = p.id) --exclude parent products
      and p.active = 't'
      and barcode is not null
order by c.name;
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
                                    supplier_code: sku
                                },
                                height: Decimal::new(0, 0),
                                length: Decimal::new(0, 0),
                                weight: Decimal::new(0, 0),
                                width: Decimal::new(0, 0),
                            }
                        },
                        images: get_product_images(db, id).into_iter().map(|i| Image {
                            url: i.data.clone(),
                            md5: calculate_md5(&i.data)
                        }).collect()
                    }
                },
                title: row.try_get(5).unwrap(),
                long_description: row.try_get(7).unwrap(),
            })
        }
        products
    }
}

pub fn calculate_md5(image_url: &str) -> String {
    println!("Url: {}", image_url);

    let (tx, rx) = std::sync::mpsc::channel();
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let client = reqwest::ClientBuilder::new()
            // .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        let response = client.get(image_url).send().await.unwrap();
        let data = response.text().await.unwrap();
        let hash = md5::compute(data);
        println!("Hash: {:#?}", hash);
        tx.send(hash).unwrap();
    });
    let hash = rx.recv().unwrap();

    // hasher.update();
    format!("{:x}", hash)
}
