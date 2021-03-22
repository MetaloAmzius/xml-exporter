extern crate env_logger;

use crate::write::Write;
use crate::database::Database;
use log::debug;
use std::fs::File;
use std::io::Write as OtherWrite;

mod shopzone;
mod models;
mod database;
mod write;
mod varle;
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
    #[clap(short, long, default_value="1")]
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
        1 => file.write_all(Write::write(&shopzone::database::load(&db)).as_bytes())
                 .expect("Failed to generate shopzone xml"),
        2 => file.write_all(Write::write(&varle::database::load(&db)).as_bytes())
            .expect("Failed to generate varle xml"),
        _ => panic!("incorrect style argument")
    };

    let result = std::process::Command::new("xmllint")
        .arg("-format")
        .arg("temp.xml")
        .output()
        .unwrap();

    let mut formatted = File::create(&opts.output_file).unwrap();
    formatted.write_all(&result.stdout).unwrap();
    std::fs::remove_file("temp.xml").unwrap();
}

fn check_xmllint_version() {
    let output = std::process::Command::new("xmllint").arg("--version")
                                                      .output()
                                                      .expect("Failed to check xmllint version...");
    //TODO: add version check
    debug!("xmllint version: {:#?}", output);
}
