use glib::{ParamFlags, ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

use std::cell::RefCell;

// Object holding the state
#[derive(Default)]
pub struct CustomButton {
    number: RefCell<i32>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

// ANCHOR: object_impl
// Trait shared by all GObjects
impl ObjectImpl for CustomButton {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.set_label(&self.number.borrow().to_string());
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpec::int(
                // Name
                "number",
                // Nickname
                "number",
                // Short description
                "number",
                // Minimum value
                i32::MIN,
                // Maximum value
                i32::MAX,
                // Default value
                0,
                // The property can be read and written to
                ParamFlags::READWRITE,
            )]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.get_name() {
            "number" => {
                let input_number = value.get().unwrap().unwrap();
                self.number.replace(input_number);
            }
            _ => unimplemented!(),
        }
    }

    fn get_property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.get_name() {
            "number" => self.number.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}
// ANCHOR_END: object_impl

// Trait shared by all widgets
impl WidgetImpl for CustomButton {}

// Trait shared by all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self, button: &Self::Type) {
        let incremented_number = self.number.borrow().clone() + 1;
        button.set_property("number", &incremented_number).unwrap();
        button.set_label(&self.number.borrow().to_string())
    }
}