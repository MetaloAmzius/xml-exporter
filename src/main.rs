use crate::database::Database;
use crate::models::CData;
use crate::models::Category;
use crate::models::Root;
use crate::write::Write;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::fs::File;
use std::io::Write as OtherWrite;

mod database;
mod models;
mod write;
use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "Ignas Lapėnas <ignas@lapenas.dev>")]
struct Opts {
    /// Sets the postgresql connection string to the database
    /// Ex. "host=localhost user=root password=rootpw dbname=metaloamzius_web"
    #[clap(short, long)]
    connection_string: String,
}

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_module_level("xml_exporter", LevelFilter::Debug)
        .init()
        .unwrap();
    //load data
    let opts: Opts = Opts::parse();

    let mut file = File::create("output.xml").unwrap();
    file.write_all(Write::write(&Database::new(&opts.connection_string).load()).as_bytes())
        .unwrap();

    let result = std::process::Command::new("xmllint")
        .arg("-format")
        .arg("output.xml")
        .output()
        .unwrap();
    let mut formatted = File::create("formatted.xml").unwrap();
    formatted.write_all(&result.stdout).unwrap();
    std::fs::remove_file("output.xml").unwrap();
}
