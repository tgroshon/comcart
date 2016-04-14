#[derive(Debug)]
pub struct Summary {
    pub general: Option<General>,
    pub modules: Option<Vec<Module>>,
}

impl Summary {
    pub fn new(modules: Option<Vec<Module>>) -> Summary {
        Summary {
            general: None,
            modules: modules,
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
pub struct ModuleItem {
    pub title: String,
    pub item_type: String,
}

impl ModuleItem {
    pub fn new(title: String, i_type: String) -> ModuleItem {
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
    pub fn new(title: String, items: Vec<ModuleItem>) -> Module {
        Module {
            title: title,
            items: items,
        }
    }
}
