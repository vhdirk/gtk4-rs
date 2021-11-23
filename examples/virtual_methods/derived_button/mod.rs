mod imp;

use gtk::{
    self, glib, glib::prelude::*, glib::subclass::prelude::*, prelude::*, subclass::prelude::*,
};

glib::wrapper! {
    pub struct DerivedButton(ObjectSubclass<imp::DerivedButton>)
        @extends gtk::Widget, gtk::Button, crate::base_button::BaseButton;
}

impl DerivedButton {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create DerivedButton")
    }
}

impl Default for DerivedButton {
    fn default() -> Self {
        let d = Self::new();
        let imp = imp::DerivedButton::from_instance(&d);

        imp.msg.set("DerivedButton Oncecell text".to_string());
        d
    }
}
