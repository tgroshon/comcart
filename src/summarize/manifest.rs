use std::str::FromStr;
use std::io::{BufReader};
use std::collections::HashMap;

use zip::read::{ZipFile};
use xml::reader::{EventReader, XmlEvent};
use xml::name::{OwnedName};

use super::utils::Node;
use super::utils::find_attr;
use common::{Module, ModuleItem};

const MODULE_DEPTH: i32 = 5;
const MODULE_ITEM_DEPTH: i32 = 6;

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

pub fn parse(manifest: ZipFile) -> Vec<Module> {
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
                    if depth == MODULE_DEPTH + 1 {
                        if let Some(module) = modules.get_mut(module_index) {
                            module.title = Some(chars);
                        }
                    } else if depth == MODULE_ITEM_DEPTH + 1 {
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
