use std::io::{BufReader, Read, Seek, Result};
use std::collections::HashMap;
use std::str::FromStr;

use xml::reader::{EventReader, XmlEvent};
use xml::name::{OwnedName};
use xml::attribute::{OwnedAttribute};

use zip::{ZipArchive};
use zip::read::{ZipFile};

const MODULE_DEPTH: i32 = 5;
const MODULE_TITLE_DEPTH: i32 = 6;
const MODULE_ITEM_DEPTH: i32 = 6;
const MODULE_ITEM_TITLE_DEPTH: i32 = 7;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct General {
    pub title: String,
    pub description: String,
    pub keyword: String,
}

#[derive(Debug)]
pub struct SparseModuleItem {
    pub title: Option<String>,
    pub identifier_ref: Option<String>,
}

impl SparseModuleItem {
    fn new(identifier_ref: Option<String>) -> SparseModuleItem {
        SparseModuleItem {
            title: None,
            identifier_ref: identifier_ref,
        }
    }

    fn to_module_item(self, resources: &HashMap<String, Node>) -> ModuleItem {
        let title = self.title.unwrap_or("".to_string());
        // TODO lookup type from string
        let i_type = resources
            .get(self.identifier_ref.unwrap().as_str())
            .and_then(|resource| find_attr("type", &resource.attributes))
            .unwrap_or("".to_string());
        ModuleItem::new(title, i_type)
    }
}

#[derive(Debug)]
pub struct SparseModule {
    pub title: Option<String>,
    pub items: Vec<SparseModuleItem>,
}

impl SparseModule {
    fn new() -> SparseModule {
        SparseModule {
            title: None,
            items: Vec::new(),
        }
    }

    fn to_module(self, resources: &HashMap<String, Node>) -> Module {
        let title = self.title.unwrap_or("".to_string());
        let items = self.items
            .into_iter()
            .filter(|s_item| s_item.identifier_ref.is_some())
            .map(|s_item| s_item.to_module_item(resources))
            .collect::<Vec<ModuleItem>>();
        Module::new(title, items)
    }
}

#[derive(Debug)]
pub struct ModuleItem {
    pub title: String,
    pub item_type: String,
}

impl ModuleItem {
    fn new(title: String, i_type: String) -> ModuleItem {
        ModuleItem {
            title: title,
            item_type: i_type,
        }
    }
}

#[derive(Debug)]
pub struct Module {
    pub title: String,
    pub items: Vec<ModuleItem>,
}

impl Module {
    fn new (title: String, items: Vec<ModuleItem>) -> Module {
        Module {
            title: title,
            items: items,
        }
    }
}

pub struct Summary {
    pub general: Option<General>,
    pub modules: Option<Vec<Module>>,
}

impl Summary {
    fn new() -> Summary {
        Summary {
            general: None,
            modules: None,
        }
    }
}

fn find_attr(key: &str, attrs: &Vec<OwnedAttribute>) -> Option<String> {
    for attr in attrs {
        if attr.name.local_name.as_str() == key {
            return Some(attr.value.clone());
        }
    }
    None
}

fn collect_manifest(manifest: ZipFile) -> Vec<Module> {
    let mut modules: Vec<SparseModule> = Vec::new();
    let mut module_index = 0;
    let mut module_item_index = 0;
    let mut resources: HashMap<String, Node> = HashMap::new();

    let buffer = BufReader::new(manifest);
    let parser = EventReader::new(buffer);

    let mut depth = 0;
    let mut current_tag = OwnedName::from_str("unmatchable:start").unwrap();

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement {name, attributes, ..}) => {
                depth += 1;
                current_tag = name.clone();
                match name.local_name.as_str() {
                    "item" => {
                        if depth == MODULE_DEPTH {
                            modules.push(SparseModule::new());
                        } else if depth == MODULE_ITEM_DEPTH {
                            if let Some(module) = modules.get_mut(module_index) {
                                let identifier_ref = find_attr("identifierref", &attributes);
                                module.items.push(SparseModuleItem::new(identifier_ref));
                            }
                        }
                    }
                    "resource" => {
                        if depth == 3 {
                            if let Some(identifier) = find_attr("identifier", &attributes){
                                resources.insert(identifier, Node::new(name, attributes));
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(XmlEvent::EndElement {name}) => {
                if name.local_name.as_str() == "item" {
                    if depth == MODULE_DEPTH {
                        module_index += 1;
                        module_item_index = 0;
                    } else if depth == MODULE_ITEM_DEPTH {
                        module_item_index += 1;
                    }
                }
                depth -= 1;
            }
            Ok(XmlEvent::Characters(chars)) => {
                if current_tag.local_name.as_str() == "title" {
                    if depth == MODULE_TITLE_DEPTH {
                        if let Some(module) = modules.get_mut(module_index) {
                            module.title = Some(chars);
                        }
                    } else if depth == MODULE_ITEM_TITLE_DEPTH {
                        if let Some(module) = modules.get_mut(module_index) {
                            if let Some(module_item) = module.items.get_mut(module_item_index) {
                                module_item.title = Some(chars);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    modules.into_iter().map(|sparse_mod| sparse_mod.to_module(&resources)).collect::<Vec<Module>>()
}

pub fn summarize<R: Read + Seek>(mut archive: ZipArchive<R>) -> Result<Summary> {
    let manifest = try!(archive.by_name("imsmanifest.xml"));
    let modules = collect_manifest(manifest);
    println!("{:?}", modules);
    let summary = Summary::new();
    Ok(summary)
}
