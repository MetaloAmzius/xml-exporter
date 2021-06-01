use crate::models::CData;
use rust_decimal::Decimal;
use postgres::NoTls;
use postgres::Client;
use crate::Database;
use crate::pigu::models::Root;
use crate::database::Loadable;
use crate::database::get_product_images;
use crate::pigu::models::Product;

pub fn load(db: &Database, rivile_db:Vec<rivile_client::models::Product>) -> Root {
    Root {
        products: Product::load_all(db).into_iter()
                                       .filter_map(|p| match rivile_db.iter()
                                                   .find(|rp| rp.code == p.sku){
                                                       Some(rp) => {
                                                           Some(Product {
                                                               weight: rp.weight,
                                                               length: rp.length,
                                                               width: rp.width,
                                                               height: rp.height,
                                                               ..p
                                                           })
                                                       },
                                                       None => None
                                                   })
                                       .collect(),
    }
}

impl Loadable for Product {
    fn load_all(db: &Database) -> Vec<Product> {
        let mut client = Client::connect(&db.connection_string, NoTls).unwrap();
        let mut products = vec!();

        for row in client.query("
    select p.id,
           sku,
           barcode,
           name,
           pm.title,
           p.description as modification
      from products p
inner join product_metadata pm on p.id = pm.attribute_owner_id
                                and pm.key in ('Tūris', 'Talpa', 'Diametras', 'Galia', 'Skersmuo', 'Dydis')
where not exists (select null
                    from products
                   where parent_id = p.id) --exclude parent products
      and p.active = 't'
      and barcode is not null
order by name;
", &[]).unwrap()
        {
            let id: i32 = row.try_get(0).unwrap();
            let sku: String = row.try_get(1).unwrap();

            products.push(Product {
                sku,
                barcode: row.try_get(2).unwrap(),
                images: get_product_images(db, id),
                title: row.try_get(3).unwrap(),
                modification: row.try_get(4).unwrap(),
                description: CData {
                    data: row.try_get(5).unwrap()
                },
                weight: Decimal::new(0, 0),
                length: Decimal::new(0, 0),
                width: Decimal::new(0, 0),
                height: Decimal::new(0, 0),

            })
        }
        products
    }
}

