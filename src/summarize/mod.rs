pub mod manifest;
pub mod utils;

use zip::{ZipArchive};
use std::io::{Read, Seek, Result};
use common::Summary;

pub fn summarize<R: Read + Seek>(mut archive: ZipArchive<R>) -> Result<Summary> {
    let manifest = manifest::parse(try!(archive.by_name("imsmanifest.xml")));
    let summary = Summary::new(manifest);
    Ok(summary)
}
