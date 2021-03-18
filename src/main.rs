extern crate env_logger;
use crate::database::Database;
use crate::models::CData;
use crate::models::Category;
use crate::models::Root;
use crate::write::Write;
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
    #[clap(short, long)]
    connection_string: String,

    /// Sets the export output file (Optional)
    /// Default value: output.xml
    /// Ex. "output.xml"
    #[clap(short, long, default_value="output.xml")]
    output_file: String,
}

fn main() {
    env_logger::init();

    //load data
    let opts: Opts = Opts::parse();

    let mut file = File::create(&opts.output_file).unwrap();
    file.write_all(Write::write(&Database::new(&opts.connection_string).load()).as_bytes())
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
