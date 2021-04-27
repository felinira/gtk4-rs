// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct ColorChooser(Interface<ffi::GtkColorChooser, ffi::GtkColorChooserInterface>);

    match fn {
        type_ => || ffi::gtk_color_chooser_get_type(),
    }
}

pub const NONE_COLOR_CHOOSER: Option<&ColorChooser> = None;

pub trait ColorChooserExt: 'static {
    #[doc(alias = "gtk_color_chooser_get_rgba")]
    fn rgba(&self) -> gdk::RGBA;

    #[doc(alias = "gtk_color_chooser_get_use_alpha")]
    fn uses_alpha(&self) -> bool;

    #[doc(alias = "gtk_color_chooser_set_rgba")]
    fn set_rgba(&self, color: &gdk::RGBA);

    #[doc(alias = "gtk_color_chooser_set_use_alpha")]
    fn set_use_alpha(&self, use_alpha: bool);

    fn connect_color_activated<F: Fn(&Self, &gdk::RGBA) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ColorChooser>> ColorChooserExt for O {
    fn rgba(&self) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            ffi::gtk_color_chooser_get_rgba(
                self.as_ref().to_glib_none().0,
                color.to_glib_none_mut().0,
            );
            color
        }
    }

    fn uses_alpha(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_color_chooser_get_use_alpha(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_rgba(&self, color: &gdk::RGBA) {
        unsafe {
            ffi::gtk_color_chooser_set_rgba(self.as_ref().to_glib_none().0, color.to_glib_none().0);
        }
    }

    fn set_use_alpha(&self, use_alpha: bool) {
        unsafe {
            ffi::gtk_color_chooser_set_use_alpha(
                self.as_ref().to_glib_none().0,
                use_alpha.into_glib(),
            );
        }
    }

    fn connect_color_activated<F: Fn(&Self, &gdk::RGBA) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn color_activated_trampoline<P, F: Fn(&P, &gdk::RGBA) + 'static>(
            this: *mut ffi::GtkColorChooser,
            color: *mut gdk::ffi::GdkRGBA,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ColorChooser>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ColorChooser::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(color),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"color-activated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    color_activated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rgba_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkColorChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ColorChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&ColorChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::rgba\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_rgba_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_use_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_alpha_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkColorChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ColorChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&ColorChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-alpha\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_alpha_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ColorChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ColorChooser")
    }
}
