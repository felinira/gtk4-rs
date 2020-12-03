//! # Composite Template Example
//!
//! This sample demonstrates how to create a widget using GTK's composite templates.

use gio::prelude::*;
use glib::subclass::prelude::*;
use glib::{glib_object_subclass, glib_wrapper};
use gtk::gtk4_macros::*;
use gtk::prelude::*;

use gtk::subclass::widget::*;

#[template(file = "src/bin/composite_template.ui")]
mod imp {
    use super::*;
    use glib::subclass;
    use gtk::subclass::prelude::*;

    /// The private struct, which can hold widgets and other data.
    #[derive(Debug, CompositeTemplate)]
    pub struct ExApplicationWindow {
        #[template_widgets]
        pub widgets: ExApplicationWindowWidgets,
        custom_string_attr: String,
    }

    impl ObjectSubclass for ExApplicationWindow {
        const NAME: &'static str = Self::CONTEXT_NAME;
        type Type = super::ExApplicationWindow;
        type ParentType = <Self as CompositeTemplateContext>::ContextParentType;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib_object_subclass!();

        fn new() -> Self {
            Self {
                widgets: ExApplicationWindowWidgets::new(),
                custom_string_attr: "Custom string_attr".to_string(),
            }
        }

        // Within class_init() you must set the template
        // and bind it's children. The CompositeTemplate
        // derive macro provides a convenience function
        // bind_template_children() to bind all children
        // at once.
        fn class_init(klass: &mut Self::Class) {
            Self::bind_implicit_widgets(klass);
        }
    }

    impl ObjectImpl for ExApplicationWindow {
        fn constructed(&self, obj: &Self::Type) {
            obj.init_template();
            obj.init_label();
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for ExApplicationWindow {}
    impl WindowImpl for ExApplicationWindow {}
    impl ApplicationWindowImpl for ExApplicationWindow {}
}

glib_wrapper! {
    pub struct ExApplicationWindow(ObjectSubclass<imp::ExApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl ExApplicationWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(Self::static_type(), &[("application", app)])
            .expect("Failed to create ExApplicationWindow")
            .downcast::<ExApplicationWindow>()
            .expect("Created object is of wrong type")
    }

    pub fn init_label(&self) {
        // To access fields such as template children, you must get
        // the private struct.
        let self_ = imp::ExApplicationWindow::from_instance(self);
        self_
            .widgets
            .subtitle_label
            .get()
            .set_text("This is an example window made using composite templates");
    }
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.composite_template"),
        Default::default(),
    )
    .expect("Failed to initialize application");

    application.connect_activate(|app| {
        let win = ExApplicationWindow::new(app);
        win.show();
    });

    application.run(&std::env::args().collect::<Vec<_>>());
}
