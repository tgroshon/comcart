extern crate zip;
extern crate xml;

use std::io;
use std::fs::File;
use std::str::FromStr;
use xml::reader::{EventReader, XmlEvent};
use xml::name::{Name, OwnedName};
use xml::attribute::{OwnedAttribute};
use std::collections::HashMap;

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

struct Node {
    name: xml::name::OwnedName,
    attributes: Vec<xml::attribute::OwnedAttribute>
}

impl Node {
    fn new(name: OwnedName, attrs: Vec<OwnedAttribute>) -> Node {
        Node {
            name: name,
            attributes: attrs
        }
    }
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}


fn process(path: &str) -> io::Result<()> {
    let f = try!(File::open(path));
    let mut zip_file = try!(zip::ZipArchive::new(f));
    let manifest = try!(zip_file.by_name("imsmanifest.xml"));

    let buf = io::BufReader::new(manifest);
    let summary = summarize_xml(buf);

    println!("Found {} modules", summary.modules.len());
    println!("Found {} modules contents", summary.modules_contents.len());
    println!("Found {} resources", summary.resources.len());
    for (key, _) in &summary.modules_contents {
        println!("Key {}", key)
    }
    Ok(())
}

struct Summary {
    modules: Vec<Node>,
    resources: Vec<Node>,
    modules_contents: HashMap<String, Node>
}

impl Summary {
    fn new() -> Summary {
        Summary {
            modules: Vec::new(),
            resources: Vec::new(),
            modules_contents: HashMap::new()
        }
    }
}

fn summarize_xml<R: io::Read>(buf: R) -> Summary {
    let parser = EventReader::new(buf);
    let mut summary = Summary::new();

    let mut depth = 0;
    let mut current_tag = OwnedName::from_str("unmatchable:start").unwrap();

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                depth += 1;
                println!("{}+{}:{}", indent(depth), depth, name.local_name);
                current_tag = name.clone();
                match name.local_name.as_str() {
                    "item" => {
                        if depth == 5 {
                            summary.modules.push(Node::new(name, attributes));
                        } else if depth == 6 {
                            if let Some(key) = find_identifierref(&attributes) {
                                summary.modules_contents.insert(key, Node::new(name, attributes));
                            }
                        }
                    }
                    "resource" => {
                        if depth == 3 {
                            summary.resources.push(Node::new(name, attributes));
                        }
                    }
                    _ => {}
                }
            }
            Ok(XmlEvent::EndElement {..}) => {
                depth -= 1;
            }
            Ok(XmlEvent::EndDocument) => {
                println!("Xml Done!");
            }
            Ok(XmlEvent::Characters(chars)) => {
                if depth == 6 && current_tag.local_name.as_str() == "title" {
                    println!("{}{}", indent(depth + 1), chars);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    summary
}

fn find_identifierref(attrs: &Vec<OwnedAttribute>) -> Option<String> {
    for attr in attrs {
        if attr.name.local_name.as_str() == "identifierref" {
            return Some(attr.value.clone());
        }
    }
    None
}
