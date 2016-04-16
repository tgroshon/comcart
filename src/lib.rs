#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate xml;
extern crate zip;

use std::io::{Result, BufReader, Read, Seek};

mod summarize;
mod common;

pub fn process<R: Read + Seek>(reader: BufReader<R>) -> Result<common::Summary> {
    let zip_file = try!(zip::ZipArchive::new(reader));
    let summary = try!(summarize::summarize(zip_file));
    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::process;
    use std::fs::File;
    use std::io::{BufReader};

    fn get_zip_file() -> File {
        if let Ok(file) = File::open("test.imscc") {
            file
        } else {
            panic!("Could not find test file 'test.imscc'")
        }
    }

    #[test]
    #[ignore]
    fn test_process() {
        let file = get_zip_file();
        match process(BufReader::new(file)) {
            Ok(summary) => {
                assert_eq!(summary.general.title, "Tommy's Awesome Course");
                assert_eq!(summary.general.copyright, "Private (Copyrighted) - http://en.wikipedia.org/wiki/Copyright");
                assert_eq!(summary.general.description, "");
                assert_eq!(summary.modules.len(), 105);
            }
            Err(e) => {
                panic!(e)
            }
        }
    }
}
