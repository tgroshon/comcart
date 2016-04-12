extern crate zip;
extern crate xml;

use std::io::{BufReader, Result};
use std::fs::File;
mod summarize;

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



fn process(path: &str) -> Result<()> {
    let f = try!(File::open(path));
    let mut zip_file = try!(zip::ZipArchive::new(f));
    let manifest = try!(zip_file.by_name("imsmanifest.xml"));

    let buf = BufReader::new(manifest);
    let summary = summarize::summarize_xml(buf);

    println!("Found {} modules", summary.modules.len());
    println!("Found {} modules contents", summary.modules_contents.len());
    println!("Found {} resources", summary.resources.len());
    for (key, _) in &summary.modules_contents {
        println!("Key {}", key)
    }
    Ok(())
}

