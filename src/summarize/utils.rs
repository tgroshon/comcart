use common::{ItemType, ParseHandler};
use std::io::{Read};
use regex::Regex;
use xml::name::{OwnedName};
use xml::attribute::{OwnedAttribute};
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Clone)]
pub struct Node {
    pub name: OwnedName,
    pub attributes: Vec<OwnedAttribute>
}

impl Node {
    pub fn new(name: OwnedName, attrs: Vec<OwnedAttribute>) -> Node {
        Node {
            name: name,
            attributes: attrs
        }
    }

    pub fn name_str(&self) -> &str {
        &self.name.local_name.as_str()
    }

    pub fn has_name(&self, str: &str) -> bool {
        self.name_str() == str
    }

    pub fn find(&self, key: &str) -> Option<String> {
        for attr in &self.attributes {
            if attr.name.local_name.as_str() == key {
                return Some(attr.value.clone());
            }
        }
        None
    }
}

pub fn typestr_to_type(i_type: &str) -> ItemType {
    if i_type.is_empty() {
        return ItemType::NoType;
    }
    lazy_static! {
        static ref ASSIGNMENT_R: Regex = Regex::new(r"assignment|associatedcontent/imscc_xmlv1p1/learning-application-resource").unwrap();
        static ref ASSESSMENT_R: Regex = Regex::new(r"assessment|quiz").unwrap();
        static ref DISCUSSION_R: Regex = Regex::new(r"imsdt").unwrap();
        static ref WEBCONTENT_R: Regex = Regex::new(r"webcontent").unwrap();
        static ref WEBLINK_R: Regex = Regex::new(r"wl").unwrap();
    }
    if ASSIGNMENT_R.is_match(i_type) {
        ItemType::Assignment
    } else if ASSESSMENT_R.is_match(i_type) {
        ItemType::Assessment
    } else if DISCUSSION_R.is_match(i_type) {
        ItemType::DiscussionTopic
    } else if WEBCONTENT_R.is_match(i_type) {
        ItemType::WebContent
    } else if WEBLINK_R.is_match(i_type) {
        ItemType::WebLink
    } else {
        ItemType::Unknown{ type_string: i_type.to_string() }
    }
}

pub const MODULE_DEPTH: usize = 5;
pub const MODULE_ITEM_DEPTH: usize = 6;

pub fn handle_parse<R: Read, H: ParseHandler>(buffer: R, handler: &mut H) {
    for event in EventReader::new(buffer) {
        match event {
            Ok(event_type) => {
                match event_type {
                    XmlEvent::StartElement {name, attributes, ..} => {
                        handler.enter(Node::new(name, attributes));
                    }
                    XmlEvent::EndElement {name} => {
                        handler.leave(name);
                    }
                    XmlEvent::Characters(chars) => {
                        handler.receive_chars(chars);
                    }
                    _ => {}
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
