//! # Video Player
//!
//! This sample demonstrates how to use GtkVideo to play videos

use gio::prelude::*;
use glib::subclass::prelude::*;
use glib::{clone, glib_object_subclass, glib_wrapper};
use gtk::{prelude::*, CompositeTemplate};

mod imp {
    use super::*;
    use glib::subclass;
    use gtk::subclass::prelude::*;
    use gtk::subclass::widget::*;

    #[derive(Debug, CompositeTemplate)]
    pub struct VideoPlayerWindow {
        #[template_child(id = "open_btn")]
        pub open_btn: TemplateChild<gtk::Button>,
        #[template_child(id = "video")]
        pub video: TemplateChild<gtk::Video>,
    }

    impl ObjectSubclass for VideoPlayerWindow {
        const NAME: &'static str = "VideoPlayerWindow";
        type Type = super::VideoPlayerWindow;
        type ParentType = gtk::ApplicationWindow;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib_object_subclass!();

        fn new() -> Self {
            Self {
                open_btn: TemplateChild::default(),
                video: TemplateChild::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            let template = include_bytes!("ui/video_player.ui");
            klass.set_template(template);
            Self::bind_template_children(klass);
        }
    }

    impl ObjectImpl for VideoPlayerWindow {
        fn constructed(&self, obj: &Self::Type) {
            obj.init_template();
            obj.init_actions();
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for VideoPlayerWindow {}
    impl WindowImpl for VideoPlayerWindow {}
    impl ApplicationWindowImpl for VideoPlayerWindow {}
}

glib_wrapper! {
    pub struct VideoPlayerWindow(ObjectSubclass<imp::VideoPlayerWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl VideoPlayerWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(Self::static_type(), &[("application", app)])
            .expect("Failed to create VideoPlayerWindow")
            .downcast::<VideoPlayerWindow>()
            .expect("Created object is of wrong type")
    }

    fn init_actions(&self) {
        let open = gio::SimpleAction::new("open", None);
        open.connect_activate(clone!(@weak self as win => move |_, _| {
            let dialog = gtk::FileChooserDialog::new(Some("Open Video"), Some(&win), gtk::FileChooserAction::Open, &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);
            dialog.set_modal(true);

            let videos_filter = gtk::FileFilter::new();
            videos_filter.add_mime_type("video/webm");
            videos_filter.set_name(Some("Video"));

            dialog.add_filter(&videos_filter);

            dialog.connect_response(move |d, response| {
                if response == gtk::ResponseType::Ok {
                    win.set_video(d.get_file().unwrap());
                }
                d.close();
            });

            dialog.show();
        }));
        self.add_action(&open);
    }

    fn set_video(&self, video: gio::File) {
        let self_ = imp::VideoPlayerWindow::from_instance(self);
        self_.video.get().set_file(Some(&video));
    }
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.video_player"),
        Default::default(),
    )
    .expect("Failed to initialize application");

    application.connect_activate(|app| {
        let win = VideoPlayerWindow::new(app);
        win.show();
    });

    application.run(&std::env::args().collect::<Vec<_>>());
}
