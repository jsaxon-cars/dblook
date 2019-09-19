extern crate clap;

extern crate reqwest;

#[macro_use]
extern crate mysql;

use mysql as my;

use indicatif::{ProgressBar, ProgressStyle};

use std::error::Error;

use clap::{App, Arg};
use reqwest::Url;

fn main() {
    let matches = App::new("dblook")
        .version("0.0.1")
        .author("James Saxon <jsaxon@cars.com>")
        .about("Looks at tables in a mysql database")
        .arg(
            Arg::with_name("uri")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("the uri of the database"),
        )
        .get_matches();
    let uri = matches.value_of("uri").unwrap();

    match show_tables(uri, true) {
        Ok(_) => println!("YAY"),
        _ => println!("BOO"),
    }
}

fn show_tables(dburi: &str, quiet: bool) -> Result<(), Box<Error>> {
    let msg = format!("Getting Tables For: {}", dburi);
    let bar = progress_bar(quiet, &msg, Some(100));

    let conn = Url::parse(dburi).unwrap();
    bar.inc(25);
    println!("Got a connection scheme:  {:?}", conn.scheme());

    let pool = my::Pool::new(dburi).unwrap();
    bar.inc(50);
    match pool
        // .prep_exec(r"SELECT * FROM `leads` LIMIT 3;", ())
        .prep_exec(r"SHOW TABLES;", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                println!("    | TABLE: {:?}", row);
            })
        }) {
        Ok(_) => println!("SUCCESS"),
        Err(thing) => println!("{:?}", thing),
    }
    bar.finish();

    Result::Ok(())
}

fn progress_bar(quiet: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet {
        true => ProgressBar::hidden(),
        false => match length {
            Some(len) => ProgressBar::new(len),
            None => ProgressBar::new_spinner(),
        },
    };

    bar.set_message(msg);
    match length {
        Some(_) => bar
            .set_style(ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_predcise}] [{widie)bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
                .progress_chars("=> ")),
        None => bar.set_style(ProgressStyle::default_spinner()),
    };

    bar
}

#[cfg(test)]
mod test {

    #[test]
    fn foo() {
        assert!(true)
    }
}
