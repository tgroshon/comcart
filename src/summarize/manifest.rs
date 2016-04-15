use common::{ModuleBuilder, ModuleItemBuilder, Manifest, ManifestBuilder};
use std::io::{BufReader};
use super::utils::Node;
use super::utils::find_attr;
use xml::reader::{EventReader, XmlEvent};
use xml::name::{OwnedName};
use zip::read::{ZipFile};

const MODULE_DEPTH: usize = 5;
const MODULE_ITEM_DEPTH: usize = 6;

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

pub fn parse(manifest: ZipFile) -> Manifest {
    let mut tracker = StackTracker::new();
    let mut data = ManifestBuilder::new();
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
    let manifest = data.finalize();
    println!("{:?}", manifest.general);
    manifest
}

fn save_manifest_data(tracker: &StackTracker, data: &mut ManifestBuilder, node: Node) {
    match node.name.local_name.as_str() {
        "item" => {
            if tracker.depth == MODULE_DEPTH {
                data.modules.push(ModuleBuilder::new());
            } else if tracker.depth == MODULE_ITEM_DEPTH {
                if let Some(module) = data.modules.get_mut(tracker.module_index) {
                    let mut module_item_builder = ModuleItemBuilder::new();
                    module_item_builder.identifier_ref(find_attr("identifierref", &node.attributes));
                    module.items.push(module_item_builder);
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

fn attach_titles(tracker: &StackTracker, data: &mut ManifestBuilder, chars: String) {
    if tracker.depth == MODULE_DEPTH + 1 {
        if let Some(module) = data.modules.get_mut(tracker.module_index) {
            module.title(chars);
        }
    } else if tracker.depth == MODULE_ITEM_DEPTH + 1 {
        if let Some(module) = data.modules.get_mut(tracker.module_index) {
            if let Some(module_item) = module.items.get_mut(tracker.module_item_index) {
                module_item.title(chars);
            }
        }
    }
}

fn find_general_data(category: &OwnedName, current_tag: &OwnedName, parent_tag: &OwnedName, data: &mut ManifestBuilder, chars: String) {
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

fn collect_characters(tracker: &StackTracker, data: &mut ManifestBuilder, chars: String) {
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
