// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::DirectionType;
use crate::LayoutManager;
use crate::NotebookPage;
use crate::NotebookTab;
use crate::Overflow;
use crate::PackType;
use crate::PositionType;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct Notebook(Object<ffi::GtkNotebook>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_notebook_get_type(),
    }
}

impl Notebook {
    #[doc(alias = "gtk_notebook_new")]
    pub fn new() -> Notebook {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_notebook_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_notebook_detach_tab")]
    pub fn detach_tab<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_notebook_detach_tab(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_notebook_get_action_widget")]
    pub fn action_widget(&self, pack_type: PackType) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_action_widget(
                self.to_glib_none().0,
                pack_type.into_glib(),
            ))
        }
    }

    #[doc(alias = "gtk_notebook_get_group_name")]
    pub fn group_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_notebook_get_group_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_notebook_get_menu_label")]
    pub fn menu_label<P: IsA<Widget>>(&self, child: &P) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_menu_label(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_notebook_get_menu_label_text")]
    pub fn menu_label_text<P: IsA<Widget>>(&self, child: &P) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_menu_label_text(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_notebook_get_page")]
    pub fn page<P: IsA<Widget>>(&self, child: &P) -> Option<NotebookPage> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_page(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_notebook_get_pages")]
    pub fn pages(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_full(ffi::gtk_notebook_get_pages(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_notebook_get_scrollable")]
    pub fn is_scrollable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_notebook_get_scrollable(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_notebook_get_show_border")]
    pub fn shows_border(&self) -> bool {
        unsafe { from_glib(ffi::gtk_notebook_get_show_border(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_notebook_get_show_tabs")]
    pub fn shows_tabs(&self) -> bool {
        unsafe { from_glib(ffi::gtk_notebook_get_show_tabs(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_notebook_get_tab_detachable")]
    pub fn tab_is_detachable<P: IsA<Widget>>(&self, child: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_detachable(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_notebook_get_tab_label")]
    pub fn tab_label<P: IsA<Widget>>(&self, child: &P) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_tab_label(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_notebook_get_tab_label_text")]
    pub fn tab_label_text<P: IsA<Widget>>(&self, child: &P) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_tab_label_text(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_notebook_get_tab_pos")]
    pub fn tab_pos(&self) -> PositionType {
        unsafe { from_glib(ffi::gtk_notebook_get_tab_pos(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_notebook_get_tab_reorderable")]
    pub fn tab_is_reorderable<P: IsA<Widget>>(&self, child: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_reorderable(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_notebook_next_page")]
    pub fn next_page(&self) {
        unsafe {
            ffi::gtk_notebook_next_page(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_notebook_popup_disable")]
    pub fn popup_disable(&self) {
        unsafe {
            ffi::gtk_notebook_popup_disable(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_notebook_popup_enable")]
    pub fn popup_enable(&self) {
        unsafe {
            ffi::gtk_notebook_popup_enable(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_notebook_prev_page")]
    pub fn prev_page(&self) {
        unsafe {
            ffi::gtk_notebook_prev_page(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_notebook_set_action_widget")]
    pub fn set_action_widget<P: IsA<Widget>>(&self, widget: &P, pack_type: PackType) {
        unsafe {
            ffi::gtk_notebook_set_action_widget(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                pack_type.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_notebook_set_group_name")]
    pub fn set_group_name(&self, group_name: Option<&str>) {
        unsafe {
            ffi::gtk_notebook_set_group_name(self.to_glib_none().0, group_name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_notebook_set_menu_label")]
    pub fn set_menu_label<P: IsA<Widget>, Q: IsA<Widget>>(
        &self,
        child: &P,
        menu_label: Option<&Q>,
    ) {
        unsafe {
            ffi::gtk_notebook_set_menu_label(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                menu_label.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_notebook_set_menu_label_text")]
    pub fn set_menu_label_text<P: IsA<Widget>>(&self, child: &P, menu_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_menu_label_text(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                menu_text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_notebook_set_scrollable")]
    pub fn set_scrollable(&self, scrollable: bool) {
        unsafe {
            ffi::gtk_notebook_set_scrollable(self.to_glib_none().0, scrollable.into_glib());
        }
    }

    #[doc(alias = "gtk_notebook_set_show_border")]
    pub fn set_show_border(&self, show_border: bool) {
        unsafe {
            ffi::gtk_notebook_set_show_border(self.to_glib_none().0, show_border.into_glib());
        }
    }

    #[doc(alias = "gtk_notebook_set_show_tabs")]
    pub fn set_show_tabs(&self, show_tabs: bool) {
        unsafe {
            ffi::gtk_notebook_set_show_tabs(self.to_glib_none().0, show_tabs.into_glib());
        }
    }

    #[doc(alias = "gtk_notebook_set_tab_detachable")]
    pub fn set_tab_detachable<P: IsA<Widget>>(&self, child: &P, detachable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_detachable(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                detachable.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_notebook_set_tab_label")]
    pub fn set_tab_label<P: IsA<Widget>, Q: IsA<Widget>>(&self, child: &P, tab_label: Option<&Q>) {
        unsafe {
            ffi::gtk_notebook_set_tab_label(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_label.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_notebook_set_tab_label_text")]
    pub fn set_tab_label_text<P: IsA<Widget>>(&self, child: &P, tab_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_tab_label_text(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                tab_text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_notebook_set_tab_pos")]
    pub fn set_tab_pos(&self, pos: PositionType) {
        unsafe {
            ffi::gtk_notebook_set_tab_pos(self.to_glib_none().0, pos.into_glib());
        }
    }

    #[doc(alias = "gtk_notebook_set_tab_reorderable")]
    pub fn set_tab_reorderable<P: IsA<Widget>>(&self, child: &P, reorderable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_reorderable(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                reorderable.into_glib(),
            );
        }
    }

    #[doc(alias = "get_property_enable_popup")]
    pub fn enables_popup(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"enable-popup\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `enable-popup` getter")
        }
    }

    #[doc(alias = "set_property_enable_popup")]
    pub fn set_enable_popup(&self, enable_popup: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"enable-popup\0".as_ptr() as *const _,
                enable_popup.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "set_property_page")]
    pub fn set_page(&self, page: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"page\0".as_ptr() as *const _,
                page.to_value().to_glib_none().0,
            );
        }
    }

    pub fn connect_change_current_page<F: Fn(&Notebook, i32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn change_current_page_trampoline<
            F: Fn(&Notebook, i32) -> bool + 'static,
        >(
            this: *mut ffi::GtkNotebook,
            object: libc::c_int,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"change-current-page\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    change_current_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_change_current_page(&self, object: i32) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("change-current-page", &[&object])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_change_current_page`")
    }

    pub fn connect_create_window<F: Fn(&Notebook, &Widget) -> Notebook + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn create_window_trampoline<
            F: Fn(&Notebook, &Widget) -> Notebook + 'static,
        >(
            this: *mut ffi::GtkNotebook,
            page: *mut ffi::GtkWidget,
            f: glib::ffi::gpointer,
        ) -> *mut ffi::GtkNotebook {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(page)) /*Not checked*/
                .to_glib_none()
                .0
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"create-window\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    create_window_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_focus_tab<F: Fn(&Notebook, NotebookTab) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn focus_tab_trampoline<
            F: Fn(&Notebook, NotebookTab) -> bool + 'static,
        >(
            this: *mut ffi::GtkNotebook,
            object: ffi::GtkNotebookTab,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(object)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"focus-tab\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    focus_tab_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_focus_tab(&self, object: NotebookTab) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("focus-tab", &[&object])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_focus_tab`")
    }

    pub fn connect_move_focus_out<F: Fn(&Notebook, DirectionType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_focus_out_trampoline<
            F: Fn(&Notebook, DirectionType) + 'static,
        >(
            this: *mut ffi::GtkNotebook,
            object: ffi::GtkDirectionType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-focus-out\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_focus_out_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_move_focus_out(&self, object: DirectionType) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("move-focus-out", &[&object])
                .unwrap()
        };
    }

    pub fn connect_page_added<F: Fn(&Notebook, &Widget, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn page_added_trampoline<F: Fn(&Notebook, &Widget, u32) + 'static>(
            this: *mut ffi::GtkNotebook,
            child: *mut ffi::GtkWidget,
            page_num: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(child), page_num)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    page_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_page_removed<F: Fn(&Notebook, &Widget, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn page_removed_trampoline<F: Fn(&Notebook, &Widget, u32) + 'static>(
            this: *mut ffi::GtkNotebook,
            child: *mut ffi::GtkWidget,
            page_num: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(child), page_num)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    page_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_page_reordered<F: Fn(&Notebook, &Widget, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn page_reordered_trampoline<F: Fn(&Notebook, &Widget, u32) + 'static>(
            this: *mut ffi::GtkNotebook,
            child: *mut ffi::GtkWidget,
            page_num: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(child), page_num)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-reordered\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    page_reordered_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_reorder_tab<F: Fn(&Notebook, DirectionType, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn reorder_tab_trampoline<
            F: Fn(&Notebook, DirectionType, bool) -> bool + 'static,
        >(
            this: *mut ffi::GtkNotebook,
            object: ffi::GtkDirectionType,
            p0: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(object), from_glib(p0)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"reorder-tab\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    reorder_tab_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_reorder_tab(&self, object: DirectionType, p0: bool) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("reorder-tab", &[&object, &p0])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_reorder_tab`")
    }

    pub fn connect_select_page<F: Fn(&Notebook, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn select_page_trampoline<F: Fn(&Notebook, bool) -> bool + 'static>(
            this: *mut ffi::GtkNotebook,
            object: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(object)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"select-page\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    select_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_select_page(&self, object: bool) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("select-page", &[&object])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_select_page`")
    }

    pub fn connect_switch_page<F: Fn(&Notebook, &Widget, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn switch_page_trampoline<F: Fn(&Notebook, &Widget, u32) + 'static>(
            this: *mut ffi::GtkNotebook,
            page: *mut ffi::GtkWidget,
            page_num: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(page), page_num)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"switch-page\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    switch_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_enable_popup_notify<F: Fn(&Notebook) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_popup_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-popup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_popup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_group_name_notify<F: Fn(&Notebook) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_group_name_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::group-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_group_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_page_notify<F: Fn(&Notebook) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::page\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_pages_notify<F: Fn(&Notebook) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pages_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pages\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pages_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_scrollable_notify<F: Fn(&Notebook) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_scrollable_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scrollable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scrollable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_border_notify<F: Fn(&Notebook) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_border_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-border\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_border_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_tabs_notify<F: Fn(&Notebook) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_tabs_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-tabs\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_tabs_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_tab_pos_notify<F: Fn(&Notebook) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_pos_trampoline<F: Fn(&Notebook) + 'static>(
            this: *mut ffi::GtkNotebook,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tab-pos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tab_pos_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Notebook {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct NotebookBuilder {
    enable_popup: Option<bool>,
    group_name: Option<String>,
    page: Option<i32>,
    scrollable: Option<bool>,
    show_border: Option<bool>,
    show_tabs: Option<bool>,
    tab_pos: Option<PositionType>,
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

impl NotebookBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Notebook {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref enable_popup) = self.enable_popup {
            properties.push(("enable-popup", enable_popup));
        }
        if let Some(ref group_name) = self.group_name {
            properties.push(("group-name", group_name));
        }
        if let Some(ref page) = self.page {
            properties.push(("page", page));
        }
        if let Some(ref scrollable) = self.scrollable {
            properties.push(("scrollable", scrollable));
        }
        if let Some(ref show_border) = self.show_border {
            properties.push(("show-border", show_border));
        }
        if let Some(ref show_tabs) = self.show_tabs {
            properties.push(("show-tabs", show_tabs));
        }
        if let Some(ref tab_pos) = self.tab_pos {
            properties.push(("tab-pos", tab_pos));
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
        let ret = glib::Object::new::<Notebook>(&properties).expect("object new");
        ret
    }

    pub fn enable_popup(mut self, enable_popup: bool) -> Self {
        self.enable_popup = Some(enable_popup);
        self
    }

    pub fn group_name(mut self, group_name: &str) -> Self {
        self.group_name = Some(group_name.to_string());
        self
    }

    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = Some(scrollable);
        self
    }

    pub fn show_border(mut self, show_border: bool) -> Self {
        self.show_border = Some(show_border);
        self
    }

    pub fn show_tabs(mut self, show_tabs: bool) -> Self {
        self.show_tabs = Some(show_tabs);
        self
    }

    pub fn tab_pos(mut self, tab_pos: PositionType) -> Self {
        self.tab_pos = Some(tab_pos);
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

impl fmt::Display for Notebook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Notebook")
    }
}
