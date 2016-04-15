use regex::Regex;

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
