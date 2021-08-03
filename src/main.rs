extern crate env_logger;

use crate::database::Database;
use crate::write::Write;
use clap::Clap;
use log::debug;
use log::info;
use rivile_client;
use std::fs::File;
use std::io::Write as OtherWrite;

mod database;
mod models;
mod pigu;
mod shopzone;
mod varle;
mod write;

#[derive(Clap)]
#[clap(version = "0.3.3", author = "Ignas Lapėnas <ignas@lapenas.dev>")]
struct Opts {
    /// Sets the postgresql connection string to the database
    /// Ex. "host=localhost user=root password=rootpw dbname=metaloamzius_web"
    #[clap(short, long, default_value="host=localhost user=metaloamzius password=metaloamziuspasw dbname=metaloamzius_web")]
    connection_string: String,

    ///Sets the rivile API key to use during (Pigu.lt) xml generation
    #[clap(short, long, default_value="")]
    api_key: String,

    /// Sets the export output file (Optional)
    /// Default value: output.xml
    /// Ex. "output.xml"
    #[clap(short, long, default_value="output.xml")]
    output_file: String,

    /// Sets the exported output file style
    #[clap(short, long, default_value="2")]
    /// Ex. 1 - Shopzone.lt, 2 - Varle.lt, 3 - Pigu.lt (Products), 4 - Pigu.lt (Remainders)
    style: i32,
}

fn main() {
    env_logger::init();
    check_xmllint_version();

    //load data
    let opts: Opts = Opts::parse();

    let mut file = File::create("temp.xml").unwrap();
    let db = Database::new(&opts.connection_string);

    match opts.style {
        1 => file
            .write_all(Write::write(&shopzone::database::load(&db)).as_bytes())
            .expect("Failed to generate shopzone xml"),
        2 => file
            .write_all(Write::write(&varle::database::load(&db)).as_bytes())
            .expect("Failed to generate varle xml"),
        3 => {
            let (tx, rx) = std::sync::mpsc::channel();
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let client = rivile_client::Client::new(opts.api_key.clone());
                let measured_products = client.retrieve_fully_measured_products().await;
                tx.send(measured_products).unwrap();
            });
            let measured_products = rx.recv().unwrap();
            info!("Exporting {} products", measured_products.iter().count());

            file.write_all(
                Write::write(&pigu::database::root::load(&db, measured_products)).as_bytes(),
            )
            .expect("Failed to generate pigu lt xml");
        }
        4 => file
            .write_all(Write::write(&pigu::database::remainders::load(&db)).as_bytes())
            .expect("Failed to generate pigu.lt remainders xml"),
        _ => panic!("incorrect style argument: {}", opts.style),
    };

    let result = match opts.style {
        1 => std::process::Command::new("xmllint")
            .arg("-format")
            .arg("temp.xml")
            .output()
            .unwrap(),
        2 => std::process::Command::new("cat")
            .arg("temp.xml")
            .output()
            .unwrap(),
        3 | 4 => std::process::Command::new("xmllint")
            .arg("-format")
            .arg("temp.xml")
            .output()
            .unwrap(),
        _ => panic!("incorrect style argument"),
    };

    let mut formatted = File::create(&opts.output_file).unwrap();
    formatted.write_all(&result.stdout).unwrap();
    std::fs::remove_file("temp.xml").unwrap();
}

fn check_xmllint_version() {
    let output = std::process::Command::new("xmllint")
        .arg("--version")
        .output()
        .expect("Failed to check xmllint version...");
    //TODO: add version check
    debug!("xmllint version: {:#?}", output);
}
