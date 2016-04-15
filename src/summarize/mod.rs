pub mod manifest;
pub mod utils;

use zip::{ZipArchive};
use std::io::{Read, Seek, Result};
use common::Summary;

pub fn summarize<R: Read + Seek>(mut archive: ZipArchive<R>) -> Result<Summary> {
    let manifest = try!(archive.by_name("imsmanifest.xml"));
    let manifest = manifest::parse(manifest);

    let summary = Summary::new(manifest);
    Ok(summary)
}
