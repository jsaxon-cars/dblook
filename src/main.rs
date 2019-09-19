extern crate clap;
use indicatif::{ProgressBar, ProgressStyle};

use clap::{App, Arg};

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

    let msg = format!("Getting Tables For: {}", uri);
    progress_bar(true, &msg, Some(100));

    println!("Getting Tables For: {}", uri);
}

fn download(target: &str, quiet: bool) -> Result<(), Box<::std::error::Error>> {
    let url = parse_url(target)?;
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