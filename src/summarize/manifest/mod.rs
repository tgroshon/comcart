mod handler;
mod builder;
mod index_tracker;

use common::{ Manifest };
use std::io::{BufReader};
use zip::read::{ZipFile};
use summarize::manifest::handler::ManifestHandler;
use summarize::utils::handle_parse;

pub fn parse(manifest: ZipFile) -> Manifest {
    let buffer = BufReader::new(manifest);
    let mut handler = ManifestHandler::new();
    handle_parse(buffer, &mut handler);
    let manifest = handler.finalize_manifest();
    manifest
}
