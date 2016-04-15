#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate xml;
extern crate zip;

use std::io::{Result};
use std::fs::File;

mod summarize;
mod common;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let result = match args.get(1) {
        Some(path) => process(path),
        None => {
            println!("Must pass a path string");
            std::process::exit(1);
        }
    };
    match result {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e)
    };
}

pub fn process(path: &str) -> Result<common::Summary> {
    let f = try!(File::open(path));
    let zip_file = try!(zip::ZipArchive::new(f));
    let summary = try!(summarize::summarize(zip_file));
    // println!("Summary: {:?}", summary);
    Ok(summary)
}

