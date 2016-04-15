use std::io::{BufReader};
use std::collections::HashMap;

use zip::read::{ZipFile};
use xml::reader::{EventReader, XmlEvent};
use xml::name::{OwnedName};

use super::utils::Node;
use super::utils::find_attr;
use common::{Module, ModuleItem, GeneralBuilder};

const MODULE_DEPTH: usize = 5;
const MODULE_ITEM_DEPTH: usize = 6;

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
            .filter_map(|s_item| {
                if s_item.identifier_ref.is_some() {
                    Some(s_item.to_module_item(resources))
                } else {
                    None
                }
            })
            .collect::<Vec<ModuleItem>>();
        Module::new(title, items)
    }
}

struct StackTracker {
    ancestors: Vec<OwnedName>,
    depth: usize,
    module_index: usize,
    module_item_index: usize,
}

impl StackTracker {
    fn new() -> StackTracker {
        StackTracker {
            ancestors: Vec::new(),
            depth: 0,
            module_index: 0,
            module_item_index: 0,
        }
    }

    fn enter(&mut self, name: &OwnedName) {
        self.depth += 1;
        self.ancestors.push(name.clone());
    }

    fn leave(&mut self, name: &OwnedName) {
        if name.local_name.as_str() == "item" {
            match self.depth {
                MODULE_DEPTH => {
                    self.module_index += 1;
                    self.module_item_index = 0;
                }
                MODULE_ITEM_DEPTH => {
                    self.module_item_index += 1;
                }
                _ => {}
            }
        }
        self.ancestors.pop();
        self.depth -= 1;
    }
}


struct ManifestData {
    modules: Vec<SparseModule>,
    resources: HashMap<String, Node>,
    general: GeneralBuilder,
}

impl ManifestData {
    fn new() -> ManifestData {
        ManifestData {
            general: GeneralBuilder::new(),
            modules: Vec::new(),
            resources: HashMap::new(),
        }
    }
}

pub fn parse(manifest: ZipFile) -> Vec<Module> {
    let mut tracker = StackTracker::new();
    let mut data = ManifestData::new();
    let buffer = BufReader::new(manifest);

    for event in EventReader::new(buffer) {
        match event {
            Ok(event_type) => {
                match event_type {
                    XmlEvent::StartElement {name, attributes, ..} => {
                        tracker.enter(&name);
                        save_manifest_data(&tracker, &mut data, Node::new(name, attributes));
                    }
                    XmlEvent::EndElement {name} => {
                        tracker.leave(&name)
                    }
                    XmlEvent::Characters(chars) => {
                        collect_characters(&tracker, &mut data, chars);
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
    let sparse_modules = data.modules;
    let resources = data.resources;
    println!("{:?}", data.general.finalize());
    sparse_modules.into_iter().map(|sparse_mod| sparse_mod.to_module(&resources)).collect::<Vec<Module>>()
}

fn save_manifest_data(tracker: &StackTracker, data: &mut ManifestData, node: Node) {
    match node.name.local_name.as_str() {
        "item" => {
            if tracker.depth == MODULE_DEPTH {
                data.modules.push(SparseModule::new());
            } else if tracker.depth == MODULE_ITEM_DEPTH {
                if let Some(module) = data.modules.get_mut(tracker.module_index) {
                    let identifier_ref = find_attr("identifierref", &node.attributes);
                    module.items.push(SparseModuleItem::new(identifier_ref));
                }
            }
        }
        "resource" => {
            if tracker.depth == 3 {
                if let Some(identifier) = find_attr("identifier", &node.attributes){
                    data.resources.insert(identifier, node);
                }
            }
        }
        _ => {}
    }
}

fn attach_titles(tracker: &StackTracker, data: &mut ManifestData, chars: String) {
    if tracker.depth == MODULE_DEPTH + 1 {
        if let Some(module) = data.modules.get_mut(tracker.module_index) {
            module.title = Some(chars);
        }
    } else if tracker.depth == MODULE_ITEM_DEPTH + 1 {
        if let Some(module) = data.modules.get_mut(tracker.module_index) {
            if let Some(module_item) = module.items.get_mut(tracker.module_item_index) {
                module_item.title = Some(chars);
            }
        }
    }
}

fn find_general_data(category: &OwnedName, current_tag: &OwnedName, parent_tag: &OwnedName, data: &mut ManifestData, chars: String) {
    if category.local_name.as_str() == "general" {
        if parent_tag.local_name.as_str() == "title" && current_tag.local_name.as_str() == "string" {
            data.general.title(chars);
        } else if parent_tag.local_name.as_str() == "description" && current_tag.local_name.as_str() == "string" {
            data.general.description(chars);
        }
    } else if category.local_name.as_str() == "rights" {
        if parent_tag.local_name.as_str() == "description" && current_tag.local_name.as_str() == "string" {
            data.general.copyright(chars);
        }
    }
}

fn collect_characters(tracker: &StackTracker, data: &mut ManifestData, chars: String) {
    let num_ancestors = tracker.ancestors.len();
    if num_ancestors < 2 {
        return;
    }

    let current_tag = tracker.ancestors.last().unwrap();
    let parent_tag = tracker.ancestors.get(num_ancestors - 2).unwrap();

    if current_tag.local_name.as_str() == "title" && parent_tag.local_name.as_str() == "item" {
        attach_titles(tracker, data, chars);
        return;
    }

    if num_ancestors < 4 {
        return;
    }

    let lom = tracker.ancestors.get(num_ancestors - 4).unwrap();
    if lom.local_name.as_str() == "lom" {
        let category = tracker.ancestors.get(num_ancestors - 3).unwrap();
        find_general_data(category, current_tag, parent_tag, data, chars);
        return;
    }
}
