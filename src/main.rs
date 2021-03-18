extern crate env_logger;

use crate::database::Database;
use crate::models::CData;
use crate::models::Category;
use crate::models::Root;
use crate::models::Product;
use crate::write::Write;
use either::Either;
use log::debug;
use std::fs::File;
use std::io::Write as OtherWrite;

mod database;
mod models;
mod write;
use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.2.0", author = "Ignas Lapėnas <ignas@lapenas.dev>")]
struct Opts {
    /// Sets the postgresql connection string to the database
    /// Ex. "host=localhost user=root password=rootpw dbname=metaloamzius_web"
    #[clap(short, long, default_value="host=localhost user=metaloamzius password=metaloamziuspasw dbname=metaloamzius_web")]
    connection_string: String,

    /// Sets the export output file (Optional)
    /// Default value: output.xml
    /// Ex. "output.xml"
    #[clap(short, long, default_value="output.xml")]
    output_file: String,

    /// Sets the exported output file style
    /// Ex. 1 - Merge variants, 2 - Non merged variants
    #[clap(short, long, default_value="2")]
    style: i32,
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    check_xmllint_version();

    //load data
    let opts: Opts = Opts::parse();

    let mut file = File::create(&opts.output_file).unwrap();
    let mut root = Database::new(&opts.connection_string).load();

    if opts.style == 2 {
        root.products = root.products.into_iter().map(|p| match &p.ty {
            either::Left(_) => vec![p],
            either::Right(variant) => {
                variant.variants.iter().map(|v| Product {
                    url: p.url.clone(),
                    id: p.id,
                    title: p.title.clone(),
                    description: p.description.clone(),
                    categories: p.categories.clone(),
                    manufacturer: p.manufacturer.clone(),
                    warranty: p.warranty.clone(),
                    ty: Either::Left(v.clone()),
                    weight: p.weight.clone(),
                    images: p.images.clone(),
                }).collect::<Vec<Product>>()
            }
        }).collect::<Vec<Vec<Product>>>().concat();
    }

    file.write_all(Write::write(&root).as_bytes())
        .unwrap();

    let result = std::process::Command::new("xmllint")
        .arg("-format")
        .arg(&opts.output_file)
        .output()
        .unwrap();

    let mut formatted = File::create("formatted.xml").unwrap();
    formatted.write_all(&result.stdout).unwrap();
    std::fs::remove_file(opts.output_file).unwrap();
}

fn check_xmllint_version() {
    let output = std::process::Command::new("xmllint").arg("--version")
                                                      .output()
                                                      .expect("Failed to check xmllint version...");
    //TODO: add version check
    debug!("xmllint version: {:#?}", output);
}
