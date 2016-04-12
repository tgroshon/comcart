use std::io::Read;
use std::collections::HashMap;
use std::str::FromStr;
use xml::reader::{EventReader, XmlEvent};
use xml::name::{OwnedName};
use xml::attribute::{OwnedAttribute};

pub struct Node {
    pub name: OwnedName,
    pub attributes: Vec<OwnedAttribute>
}

impl Node {
    fn new(name: OwnedName, attrs: Vec<OwnedAttribute>) -> Node {
        Node {
            name: name,
            attributes: attrs
        }
    }
}

pub struct Summary {
    pub modules: Vec<Node>,
    pub resources: Vec<Node>,
    pub modules_contents: HashMap<String, Node>
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

fn find_identifierref(attrs: &Vec<OwnedAttribute>) -> Option<String> {
    for attr in attrs {
        if attr.name.local_name.as_str() == "identifierref" {
            return Some(attr.value.clone());
        }
    }
    None
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}


pub fn summarize_xml<R: Read>(reader: R) -> Summary {
    let parser = EventReader::new(reader);
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
