use rust_decimal::Decimal;
use crate::database::Loadable;
use crate::pigu::models::remainders::Product;
use crate::pigu::models::remainders::Root;
use crate::Database;
use postgres::Client;
use postgres::NoTls;
use std::str::FromStr;

pub fn load(db: &Database) -> Root {
    Root {
        products: Product::load_all(db),
    }
}

impl Loadable for Product {
    fn load_all(db: &Database) -> Vec<Product> {
        let mut client = Client::connect(&db.connection_string, NoTls).unwrap();
        let mut products = vec![];

        for row in client.query("
    select p.id,
           p.sku,
           p.barcode,
           p.price,
           sum(coalesce(pr.count, 0)) as stock
      from products p
inner join product_remainers pr on p.id = pr.product_id
     where p.barcode is not null
           and not exists (select null
                             from product_categories_relations
                            where category_id = 1237 and product_id = p.id) --exlude Westmark
       and p.sku is not null
  group by p.id, p.sku, p.barcode, p.price
    having sum(coalesce(pr.count, 0)) > 0;
", &[]).unwrap()
        {
            products.push(Product {
                sku: row.try_get(1).unwrap(),
                ean: row.try_get(2).unwrap(),
                price: Decimal::from_str(row.try_get(3).unwrap()).unwrap(),
                stock: row.try_get(4).unwrap(),
                collection_hours: 72,
            })
        }
        products
    }
}
