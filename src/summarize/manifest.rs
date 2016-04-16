use common::{ModuleBuilder, ModuleItemBuilder, Manifest, ManifestBuilder, Resource, ParseHandler};
use std::io::{BufReader};
use super::utils::{Node, handle_parse, MODULE_DEPTH, MODULE_ITEM_DEPTH};
use xml::name::{OwnedName};
use zip::read::{ZipFile};

struct ModuleIndexTracker {
    module_index: usize,
    module_item_index: usize,
}

impl ModuleIndexTracker {
    fn new() -> ModuleIndexTracker {
        ModuleIndexTracker {
            module_index: 0,
            module_item_index: 0,
        }
    }

    fn step(&mut self, depth: usize) {
        match depth {
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
}

struct ManifestHandler {
    builder: ManifestBuilder,
    index_tracker: ModuleIndexTracker,
    stack: Vec<Node>,
}

impl ManifestHandler {
    fn new() -> ManifestHandler {
        ManifestHandler {
            builder: ManifestBuilder::new(),
            index_tracker: ModuleIndexTracker::new(),
            stack: Vec::new(),
        }
    }

    fn new_module_builder(&mut self) {
        self.builder.modules.push(ModuleBuilder::new());
    }

    fn new_module_item_builder(&mut self, node: &Node) {
        if let Some(module) = self.builder.modules.get_mut(self.index_tracker.module_index) {
            module.items.push(ModuleItemBuilder::new(node.find("identifierref")));
        }
    }

    fn new_resource(&mut self, node: &Node) {
        let resource = Resource::new(node);
        self.builder.resources_map.insert(resource.identifier.clone(), resource);
    }

    fn add_module_title(&mut self, chars: String) {
        let module_index = self.index_tracker.module_index;
        if let Some(module) = self.builder.modules.get_mut(module_index) {
            module.title(chars);
        }
    }

    fn add_module_item_title(&mut self, chars: String) {
        let module_index = self.index_tracker.module_index;
        let module_item_index = self.index_tracker.module_item_index;
        if let Some(module) = self.builder.modules.get_mut(module_index) {
            if let Some(module_item) = module.items.get_mut(module_item_index) {
                module_item.title(chars);
            }
        }
    }
}

impl ParseHandler for ManifestHandler {
    fn enter(&mut self, node: Node) {
        self.stack.push(node.clone());
        let depth = self.stack.len();
        match node.name_str() {
            "item" => {
                if depth == MODULE_DEPTH {
                    self.new_module_builder();
                } else if depth == MODULE_ITEM_DEPTH {
                    self.new_module_item_builder(&node);
                }
            }
            "resource" => {
                if depth == 3 {
                    self.new_resource(&node);
                }
            }
            _ => {}
        }
    }

    fn leave(&mut self,  name: OwnedName) {
        self.stack.pop();
        if name.local_name.as_str() == "item" {
            self.index_tracker.step(self.stack.len());
        }
    }

    fn receive_chars(&mut self, chars: String) {
        let num_ancestors = self.stack.len();
        if num_ancestors < 2 {
            return;
        }

        if self.stack.last().unwrap().has_name("title")
            && self.stack.get(num_ancestors - 2).unwrap().has_name("item") {
            attach_titles(self, chars);
            return;
        }

        if num_ancestors < 4 {
            return;
        }

        if self.stack.get(num_ancestors - 4).unwrap().has_name("lom") {
            find_general_data(self, chars);
            return;
        }
    }
}

pub fn parse(manifest: ZipFile) -> Manifest {
    let buffer = BufReader::new(manifest);
    let mut handler = ManifestHandler::new();
    handle_parse(buffer, &mut handler);
    let manifest = handler.builder.finalize();
    manifest
}

fn attach_titles(handler: &mut ManifestHandler, chars: String) {
    let depth = handler.stack.len();
    if depth == MODULE_DEPTH + 1 {
        handler.add_module_title(chars)
    } else if depth == MODULE_ITEM_DEPTH + 1 {
        handler.add_module_item_title(chars)
    }
}

fn find_general_data(handler: &mut ManifestHandler, chars: String) {
    let num_ancestors = handler.stack.len();
    let ref current_tag = handler.stack.last().unwrap();
    let ref parent_tag = handler.stack.get(num_ancestors - 2).unwrap();
    let ref category = handler.stack.get(num_ancestors - 3).unwrap();

    if category.has_name("general") {
        if parent_tag.has_name("title") && current_tag.has_name("string") {
            handler.builder.general.title(chars);
        } else if parent_tag.has_name("description") && current_tag.has_name("string") {
            handler.builder.general.description(chars);
        }
    } else if category.has_name("rights") {
        if parent_tag.has_name("description") && current_tag.has_name("string") {
            handler.builder.general.copyright(chars);
        }
    }
}
