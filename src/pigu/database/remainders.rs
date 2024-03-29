use crate::database::Loadable;
use crate::pigu::models::remainders::Product;
use crate::pigu::models::remainders::Root;
use crate::Database;
use postgres::Client;
use postgres::NoTls;

pub fn load(db: &Database) -> Root {
    Root {
        products: Product::load_all(db),
    }
}

impl Loadable for Product {
    fn load_all(db: &Database) -> Vec<Product> {
        let mut client = Client::connect(&db.connection_string, NoTls).unwrap();
        let mut products = vec![];

        for row in client
            .query(
                "
    select p.id,
           p.sku,
           p.barcode,
           p.price,
           sum(coalesce(pr.count, 0)) as stock
      from products p
inner join product_remainers pr on p.id = pr.product_id
inner join stores s on s.id = pr.store_id and s.code not in ('TR', 'ES', '') -- filter out stores
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
                        where parent_id = p.id)
       and not exists (select null
                         from product_categories_relations
                        where category_id = 1628 and product_id = p.id) --Exclude ASBIS
       and not exists (select null
                         from product_categories_relations
                        where category_id = 1639 and product_id = p.id) --Exclude Style De Vie
       and not exists (select null
                         from product_categories_relations
                        where category_id = 1237 and product_id = p.id) --Exclude Westmark
  group by p.id, p.sku, p.barcode, p.price;
",
                &[],
            )
            .unwrap()
        {
            products.push(Product {
                sku: row.try_get(1).unwrap(),
                ean: row.try_get(2).unwrap(),
                price: row.try_get(3).unwrap(),
                stock: row.try_get(4).unwrap(),
                collection_hours: 48,
            })
        }
        products
    }
}
