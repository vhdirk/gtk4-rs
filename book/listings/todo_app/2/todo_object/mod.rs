mod imp;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;
use serde::{Deserialize, Serialize};

glib::wrapper! {
    pub struct TodoObject(ObjectSubclass<imp::TodoObject>);
}

impl TodoObject {
    pub fn new(completed: bool, content: String) -> Self {
        Object::new(&[("completed", &completed), ("content", &content)])
            .expect("Failed to create `TodoObject`.")
    }

    pub fn is_completed(&self) -> bool {
        let imp = imp::TodoObject::from_instance(self);
        imp.data.borrow().completed
    }

    pub fn todo_data(&self) -> TodoData {
        let imp = imp::TodoObject::from_instance(self);
        imp.data.borrow().clone()
    }
}

// ANCHOR: serialize
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct TodoData {
    pub completed: bool,
    pub content: String,
}
// ANCHOR_END: serialize
