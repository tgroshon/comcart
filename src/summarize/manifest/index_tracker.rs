use summarize::utils::{MODULE_DEPTH, MODULE_ITEM_DEPTH};

pub struct ModuleIndexTracker {
    pub module_index: usize,
    pub module_item_index: usize,
}

impl ModuleIndexTracker {
    pub fn new() -> ModuleIndexTracker {
        ModuleIndexTracker {
            module_index: 0,
            module_item_index: 0,
        }
    }

    pub fn step(&mut self, depth: usize) {
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
