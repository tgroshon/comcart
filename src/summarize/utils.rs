use xml::name::{OwnedName};
use xml::attribute::{OwnedAttribute};

#[derive(Debug)]
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
}

pub fn find_attr(key: &str, attrs: &Vec<OwnedAttribute>) -> Option<String> {
    for attr in attrs {
        if attr.name.local_name.as_str() == key {
            return Some(attr.value.clone());
        }
    }
    None
}
