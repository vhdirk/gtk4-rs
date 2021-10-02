mod imp;

use gio::SimpleAction;
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{Application, Orientation};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create Window")
    }

    // ANCHOR: add_actions
    fn add_actions(&self) {
        // Get state
        let imp = imp::Window::from_instance(self);
        let label = imp.label.get();

        // Add stateful action "count" to `window` taking an integer as parameter
        let original_state = 0;
        let action_count = SimpleAction::new_stateful(
            "count",
            Some(&i32::static_variant_type()),
            &original_state.to_variant(),
        );

        action_count.connect_activate(clone!(@weak label => move |action, parameter| {
            // Get state
            let mut state = action
                .state()
                .expect("Could not get state.")
                .get::<i32>()
                .expect("The value needs to be of type `i32`.");

            // Get parameter
            let parameter = parameter
                .expect("Could not get parameter.")
                .get::<i32>()
                .expect("The value needs to be of type `i32`.");

            // Increase state by parameter and save state
            state += parameter;
            action.set_state(&state.to_variant());

            // Update label with new state
            label.set_label(&format!("Counter: {}", state));
        }));
        self.add_action(&action_count);

        // ANCHOR: action_quit
        // Add action "quit" to `window` taking no parameter
        let action_quit = SimpleAction::new("quit", None);

        action_quit.connect_activate(clone!(@weak self as window => move |_, _| {
            window.close();
        }));
        self.add_action(&action_quit);
        // ANCHOR_END: action_quit

        // ANCHOR: action_sensitive_button
        // Add stateful action "sensitive-button" to `window` taking no parameter
        let button = imp.button.get();
        let action_sensitive_button =
            SimpleAction::new_stateful("sensitive-button", None, &true.to_variant());

        action_sensitive_button.connect_activate(clone!(@weak button =>
            move |action, _| {
                // Get state
                let mut state = action
                    .state()
                    .expect("Could not get state.")
                    .get::<bool>()
                    .expect("The value needs to be of type `bool`.");

                // Toggle and save state
                state = !state;
                action.set_state(&state.to_variant());

                // Set the button's sensitivity in accordance with the current state
                button.set_sensitive(state);
        }));
        self.add_action(&action_sensitive_button);
        // ANCHOR_END: action_sensitive_button

        // ANCHOR: action_orientation

        // Add stateful action "orientation" to `window` taking a string as parameter
        let gtk_box = imp.gtk_box.get();
        let action_orientation = SimpleAction::new_stateful(
            "orientation",
            Some(&String::static_variant_type()),
            &"Vertical".to_variant(),
        );

        action_orientation.connect_activate(clone!(@weak gtk_box =>
            move |action, parameter| {
                // Get parameter
                let parameter = parameter
                    .expect("Could not get parameter.")
                    .get::<String>()
                    .expect("The value needs to be of type `String`.");

                let orientation = match parameter.as_str() {
                    "Horizontal" => Orientation::Horizontal,
                    "Vertical" => Orientation::Vertical,
                    _ => unreachable!()
                };

                // Set orientation and save state
                gtk_box.set_orientation(orientation);
                action.set_state(&parameter.to_variant());
        }));
        self.add_action(&action_orientation);
        //ANCHOR_END: action_orientation
    }
    // ANCHOR_END: add_actions
}
