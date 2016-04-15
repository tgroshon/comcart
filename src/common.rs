use regex::Regex;
use std::collections::HashMap;
use summarize::utils::{find_attr, Node};

#[derive(Debug)]
pub struct Manifest {
    pub general: General,
    pub modules: Vec<Module>,
    pub resources: HashMap<String, Node>,
}

#[derive(Debug)]
pub struct ManifestBuilder {
    pub general: GeneralBuilder,
    pub modules: Vec<ModuleBuilder>,
    pub resources: HashMap<String, Node>,
}

impl ManifestBuilder {
    pub fn new() -> ManifestBuilder {
        ManifestBuilder {
            general: GeneralBuilder::new(),
            modules: Vec::new(),
            resources: HashMap::new(),
        }
    }
    pub fn finalize(self) -> Manifest {
        let mut modules: Vec<Module> = Vec::new();
        // TODO: Figure out how to do this without scope hackery
        {
            let ref resources = self.resources;
            for builder in self.modules.into_iter() {
                modules.push(builder.finalize(resources));
            }
        }
        Manifest {
            general: self.general.finalize(),
            modules: modules,
            resources: self.resources,
        }
    }
}


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
    pub copyright: String,
}

#[derive(Debug)]
pub struct GeneralBuilder {
    title: String,
    description: String,
    copyright: String,
}

impl GeneralBuilder {
    pub fn new() -> GeneralBuilder {
        GeneralBuilder {
            title: "".to_string(),
            description: "".to_string(),
            copyright: "".to_string(),
        }
    }

    pub fn title(&mut self, title: String) -> &mut GeneralBuilder {
        self.title = title;
        self
    }

    pub fn copyright(&mut self, copyright: String) -> &mut GeneralBuilder {
        self.copyright = copyright;
        self
    }

    pub fn description(&mut self, desc: String) -> &mut GeneralBuilder {
        self.description = desc;
        self
    }

    pub fn finalize(self) -> General {
        General {
            title: self.title,
            description: self.description,
            copyright: self.copyright,
        }
    }
}

#[derive(Debug)]
pub struct ModuleItem {
    pub title: String,
    pub item_type: ItemType,
}

impl ModuleItem {
    pub fn new(title: String, i_type: String) -> ModuleItem {
        let item_type = typestr_to_enum(i_type.as_str());
        ModuleItem {
            title: title,
            item_type: item_type,
        }
    }
}

fn typestr_to_enum(i_type: &str) -> ItemType {
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

#[derive(Debug)]
pub enum ItemType {
    Assignment,
    Assessment,
    DiscussionTopic,
    WebContent,
    WebLink,
    NoType,
    Unknown { type_string: String },
}

#[derive(Debug)]
pub struct ModuleBuilder {
    pub title: String,
    pub items: Vec<ModuleItemBuilder>,
}

impl ModuleBuilder {
    pub fn new() -> ModuleBuilder {
        ModuleBuilder {
            title: "".to_string(),
            items: Vec::new(),
        }
    }

    pub fn title(&mut self, title: String) -> &mut ModuleBuilder {
        self.title = title;
        self
    }

    pub fn finalize(self, resources: &HashMap<String, Node>) -> Module {
        let items = self.items
            .into_iter()
            .filter_map(|s_item| {
                if s_item.identifier_ref.is_empty() {
                    None
                } else {
                    Some(s_item.finalize(resources))
                }
            })
            .collect::<Vec<ModuleItem>>();
        Module::new(self.title, items)
    }
}

#[derive(Debug)]
pub struct ModuleItemBuilder {
    pub title: String,
    pub identifier_ref: String,
}

impl ModuleItemBuilder {
    pub fn new() -> ModuleItemBuilder {
        ModuleItemBuilder {
            title: "".to_string(),
            identifier_ref: "".to_string(),
        }
    }

    pub fn title(&mut self, title: String) -> &mut ModuleItemBuilder {
        self.title = title;
        self
    }

    pub fn identifier_ref(&mut self, i_ref: Option<String>) -> &mut ModuleItemBuilder {
        if i_ref.is_none() {
            return self;
        }
        self.identifier_ref = i_ref.unwrap();
        self
    }

    pub fn finalize(self, resources: &HashMap<String, Node>) -> ModuleItem {
        let i_type = resources
            .get(self.identifier_ref.as_str())
            .and_then(|resource| find_attr("type", &resource.attributes))
            .unwrap_or("".to_string());
        ModuleItem::new(self.title, i_type)
    }
}
