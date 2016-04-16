pub mod manifest;
pub mod utils;

use std::path::Path;
use zip::{ZipArchive};
use std::io::{Read, Seek, Result, BufReader};
use common::{Summary, Resource, ItemType};

pub fn summarize<R: Read + Seek>(mut archive: ZipArchive<R>) -> Result<Summary> {
    let manifest = manifest::parse(try!(archive.by_name("imsmanifest.xml")));
    // lookup_resources(&mut archive, &manifest.resources);

    let summary = Summary::new(manifest);
    Ok(summary)
}

fn lookup_resources<R: Read + Seek>(archive: &mut ZipArchive<R>, resources: &Vec<Resource>) -> Result<()>{
    for resource in resources {
        match resource.item_type {
            ItemType::Assignment => {
                let href = &resource.href.clone().unwrap();
                let file = try!(archive.by_name(href.as_str()));
                let buffer = BufReader::new(file);
                // TODO parse XML
            },
            ItemType::Assessment => {
                println!("Resource: {:?}", resource);
                let identifier = &resource.identifier.clone();
                match Path::new(identifier).join("assessment_qti.xml").to_str() {
                    Some(path) => {
                        let file = try!(archive.by_name(path));
                        let buffer = BufReader::new(file);
                        // TODO parse XML
                    },
                    None => {}
                }
            },
            ItemType::DiscussionTopic => {
                // TODO implement for Discussions
            },
            _ => {}
        }
    }
    Ok(())
}
