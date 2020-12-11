// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::Application;
use crate::Box;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::HeaderBar;
use crate::LayoutManager;
use crate::Native;
use crate::Overflow;
use crate::ResponseType;
use crate::Root;
use crate::ShortcutManager;
use crate::Widget;
use crate::Window;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct Dialog(Object<ffi::GtkDialog, ffi::GtkDialogClass>) @extends Window, Widget, @implements Accessible, Buildable, ConstraintTarget, Native, Root, ShortcutManager;

    match fn {
        get_type => || ffi::gtk_dialog_get_type(),
    }
}

impl Dialog {
    #[doc(alias = "gtk_dialog_new")]
    pub fn new() -> Dialog {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_dialog_new()).unsafe_cast() }
    }

    //#[doc(alias = "gtk_dialog_new_with_buttons")]
    //pub fn with_buttons<P: IsA<Window>>(title: Option<&str>, parent: Option<&P>, flags: DialogFlags, first_button_text: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Dialog {
    //    unsafe { TODO: call ffi:gtk_dialog_new_with_buttons() }
    //}
}

impl Default for Dialog {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct DialogBuilder {
    use_header_bar: Option<i32>,
    application: Option<Application>,
    child: Option<Widget>,
    decorated: Option<bool>,
    default_height: Option<i32>,
    default_widget: Option<Widget>,
    default_width: Option<i32>,
    deletable: Option<bool>,
    destroy_with_parent: Option<bool>,
    display: Option<gdk::Display>,
    focus_visible: Option<bool>,
    focus_widget: Option<Widget>,
    fullscreened: Option<bool>,
    hide_on_close: Option<bool>,
    icon_name: Option<String>,
    maximized: Option<bool>,
    mnemonics_visible: Option<bool>,
    modal: Option<bool>,
    resizable: Option<bool>,
    startup_id: Option<String>,
    title: Option<String>,
    transient_for: Option<Window>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<AccessibleRole>,
}

impl DialogBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Dialog {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref use_header_bar) = self.use_header_bar {
            properties.push(("use-header-bar", use_header_bar));
        }
        if let Some(ref application) = self.application {
            properties.push(("application", application));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref decorated) = self.decorated {
            properties.push(("decorated", decorated));
        }
        if let Some(ref default_height) = self.default_height {
            properties.push(("default-height", default_height));
        }
        if let Some(ref default_widget) = self.default_widget {
            properties.push(("default-widget", default_widget));
        }
        if let Some(ref default_width) = self.default_width {
            properties.push(("default-width", default_width));
        }
        if let Some(ref deletable) = self.deletable {
            properties.push(("deletable", deletable));
        }
        if let Some(ref destroy_with_parent) = self.destroy_with_parent {
            properties.push(("destroy-with-parent", destroy_with_parent));
        }
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        if let Some(ref focus_visible) = self.focus_visible {
            properties.push(("focus-visible", focus_visible));
        }
        if let Some(ref focus_widget) = self.focus_widget {
            properties.push(("focus-widget", focus_widget));
        }
        if let Some(ref fullscreened) = self.fullscreened {
            properties.push(("fullscreened", fullscreened));
        }
        if let Some(ref hide_on_close) = self.hide_on_close {
            properties.push(("hide-on-close", hide_on_close));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref maximized) = self.maximized {
            properties.push(("maximized", maximized));
        }
        if let Some(ref mnemonics_visible) = self.mnemonics_visible {
            properties.push(("mnemonics-visible", mnemonics_visible));
        }
        if let Some(ref modal) = self.modal {
            properties.push(("modal", modal));
        }
        if let Some(ref resizable) = self.resizable {
            properties.push(("resizable", resizable));
        }
        if let Some(ref startup_id) = self.startup_id {
            properties.push(("startup-id", startup_id));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref transient_for) = self.transient_for {
            properties.push(("transient-for", transient_for));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        let ret = glib::Object::new(Dialog::static_type(), &properties)
            .expect("object new")
            .downcast::<Dialog>()
            .expect("downcast");
        ret
    }

    pub fn use_header_bar(mut self, use_header_bar: i32) -> Self {
        self.use_header_bar = Some(use_header_bar);
        self
    }

    pub fn application<P: IsA<Application>>(mut self, application: &P) -> Self {
        self.application = Some(application.clone().upcast());
        self
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn decorated(mut self, decorated: bool) -> Self {
        self.decorated = Some(decorated);
        self
    }

    pub fn default_height(mut self, default_height: i32) -> Self {
        self.default_height = Some(default_height);
        self
    }

    pub fn default_widget<P: IsA<Widget>>(mut self, default_widget: &P) -> Self {
        self.default_widget = Some(default_widget.clone().upcast());
        self
    }

    pub fn default_width(mut self, default_width: i32) -> Self {
        self.default_width = Some(default_width);
        self
    }

    pub fn deletable(mut self, deletable: bool) -> Self {
        self.deletable = Some(deletable);
        self
    }

    pub fn destroy_with_parent(mut self, destroy_with_parent: bool) -> Self {
        self.destroy_with_parent = Some(destroy_with_parent);
        self
    }

    pub fn display(mut self, display: &gdk::Display) -> Self {
        self.display = Some(display.clone());
        self
    }

    pub fn focus_visible(mut self, focus_visible: bool) -> Self {
        self.focus_visible = Some(focus_visible);
        self
    }

    pub fn focus_widget<P: IsA<Widget>>(mut self, focus_widget: &P) -> Self {
        self.focus_widget = Some(focus_widget.clone().upcast());
        self
    }

    pub fn fullscreened(mut self, fullscreened: bool) -> Self {
        self.fullscreened = Some(fullscreened);
        self
    }

    pub fn hide_on_close(mut self, hide_on_close: bool) -> Self {
        self.hide_on_close = Some(hide_on_close);
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn maximized(mut self, maximized: bool) -> Self {
        self.maximized = Some(maximized);
        self
    }

    pub fn mnemonics_visible(mut self, mnemonics_visible: bool) -> Self {
        self.mnemonics_visible = Some(mnemonics_visible);
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn startup_id(mut self, startup_id: &str) -> Self {
        self.startup_id = Some(startup_id.to_string());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn transient_for<P: IsA<Window>>(mut self, transient_for: &P) -> Self {
        self.transient_for = Some(transient_for.clone().upcast());
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }
}

pub const NONE_DIALOG: Option<&Dialog> = None;

pub trait DialogExt: 'static {
    #[doc(alias = "gtk_dialog_add_action_widget")]
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P, response_id: ResponseType);

    #[doc(alias = "gtk_dialog_add_button")]
    fn add_button(&self, button_text: &str, response_id: ResponseType) -> Widget;

    //#[doc(alias = "gtk_dialog_add_buttons")]
    //fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[doc(alias = "gtk_dialog_get_content_area")]
    fn get_content_area(&self) -> Box;

    #[doc(alias = "gtk_dialog_get_header_bar")]
    fn get_header_bar(&self) -> HeaderBar;

    #[doc(alias = "gtk_dialog_get_widget_for_response")]
    fn get_widget_for_response(&self, response_id: ResponseType) -> Option<Widget>;

    #[doc(alias = "gtk_dialog_response")]
    fn response(&self, response_id: ResponseType);

    #[doc(alias = "gtk_dialog_set_default_response")]
    fn set_default_response(&self, response_id: ResponseType);

    #[doc(alias = "gtk_dialog_set_response_sensitive")]
    fn set_response_sensitive(&self, response_id: ResponseType, setting: bool);

    fn get_property_use_header_bar(&self) -> i32;

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_close(&self);

    fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Dialog>> DialogExt for O {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P, response_id: ResponseType) {
        unsafe {
            ffi::gtk_dialog_add_action_widget(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                response_id.to_glib(),
            );
        }
    }

    fn add_button(&self, button_text: &str, response_id: ResponseType) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_dialog_add_button(
                self.as_ref().to_glib_none().0,
                button_text.to_glib_none().0,
                response_id.to_glib(),
            ))
        }
    }

    //fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_dialog_add_buttons() }
    //}

    fn get_content_area(&self) -> Box {
        unsafe {
            from_glib_none(ffi::gtk_dialog_get_content_area(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_header_bar(&self) -> HeaderBar {
        unsafe {
            from_glib_none(ffi::gtk_dialog_get_header_bar(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_widget_for_response(&self, response_id: ResponseType) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_dialog_get_widget_for_response(
                self.as_ref().to_glib_none().0,
                response_id.to_glib(),
            ))
        }
    }

    fn response(&self, response_id: ResponseType) {
        unsafe {
            ffi::gtk_dialog_response(self.as_ref().to_glib_none().0, response_id.to_glib());
        }
    }

    fn set_default_response(&self, response_id: ResponseType) {
        unsafe {
            ffi::gtk_dialog_set_default_response(
                self.as_ref().to_glib_none().0,
                response_id.to_glib(),
            );
        }
    }

    fn set_response_sensitive(&self, response_id: ResponseType, setting: bool) {
        unsafe {
            ffi::gtk_dialog_set_response_sensitive(
                self.as_ref().to_glib_none().0,
                response_id.to_glib(),
                setting.to_glib(),
            );
        }
    }

    fn get_property_use_header_bar(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"use-header-bar\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `use-header-bar` getter")
                .unwrap()
        }
    }

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn close_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkDialog,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Dialog>,
        {
            let f: &F = &*(f as *const F);
            f(&Dialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    close_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_close(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("close", &[])
                .unwrap()
        };
    }

    fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn response_trampoline<P, F: Fn(&P, ResponseType) + 'static>(
            this: *mut ffi::GtkDialog,
            response_id: ffi::GtkResponseType,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Dialog>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Dialog::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(response_id),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"response\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    response_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Dialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Dialog")
    }
}
