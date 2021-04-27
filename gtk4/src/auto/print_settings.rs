// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::NumberUpLayout;
use crate::PageOrientation;
use crate::PageRange;
use crate::PageSet;
use crate::PaperSize;
use crate::PrintDuplex;
use crate::PrintPages;
use crate::PrintQuality;
use crate::Unit;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib::wrapper! {
    pub struct PrintSettings(Object<ffi::GtkPrintSettings>);

    match fn {
        type_ => || ffi::gtk_print_settings_get_type(),
    }
}

impl PrintSettings {
    #[doc(alias = "gtk_print_settings_new")]
    pub fn new() -> PrintSettings {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_print_settings_new()) }
    }

    #[doc(alias = "gtk_print_settings_new_from_file")]
    pub fn from_file<P: AsRef<std::path::Path>>(
        file_name: P,
    ) -> Result<PrintSettings, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_print_settings_new_from_file(
                file_name.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_print_settings_new_from_gvariant")]
    pub fn from_gvariant(variant: &glib::Variant) -> PrintSettings {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_print_settings_new_from_gvariant(
                variant.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_settings_new_from_key_file")]
    pub fn from_key_file(
        key_file: &glib::KeyFile,
        group_name: Option<&str>,
    ) -> Result<PrintSettings, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_print_settings_new_from_key_file(
                key_file.to_glib_none().0,
                group_name.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_print_settings_copy")]
    pub fn copy(&self) -> Option<PrintSettings> {
        unsafe { from_glib_full(ffi::gtk_print_settings_copy(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_settings_foreach")]
    pub fn foreach<P: FnMut(&str, &str)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&str, &str)>(
            key: *const libc::c_char,
            value: *const libc::c_char,
            user_data: glib::ffi::gpointer,
        ) {
            let key: Borrowed<glib::GString> = from_glib_borrow(key);
            let value: Borrowed<glib::GString> = from_glib_borrow(value);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(key.as_str(), value.as_str());
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::gtk_print_settings_foreach(
                self.to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    #[doc(alias = "gtk_print_settings_get")]
    pub fn get(&self, key: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_settings_get_bool")]
    pub fn bool(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_bool(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_settings_get_collate")]
    pub fn is_collate(&self) -> bool {
        unsafe { from_glib(ffi::gtk_print_settings_get_collate(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_settings_get_default_source")]
    pub fn default_source(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_default_source(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_settings_get_dither")]
    pub fn dither(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_print_settings_get_dither(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_settings_get_double")]
    pub fn double(&self, key: &str) -> f64 {
        unsafe { ffi::gtk_print_settings_get_double(self.to_glib_none().0, key.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_settings_get_double_with_default")]
    pub fn double_with_default(&self, key: &str, def: f64) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_double_with_default(
                self.to_glib_none().0,
                key.to_glib_none().0,
                def,
            )
        }
    }

    #[doc(alias = "gtk_print_settings_get_duplex")]
    pub fn duplex(&self) -> PrintDuplex {
        unsafe { from_glib(ffi::gtk_print_settings_get_duplex(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_settings_get_finishings")]
    pub fn finishings(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_finishings(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_settings_get_int")]
    pub fn int(&self, key: &str) -> i32 {
        unsafe { ffi::gtk_print_settings_get_int(self.to_glib_none().0, key.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_settings_get_int_with_default")]
    pub fn int_with_default(&self, key: &str, def: i32) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_int_with_default(
                self.to_glib_none().0,
                key.to_glib_none().0,
                def,
            )
        }
    }

    #[doc(alias = "gtk_print_settings_get_length")]
    pub fn length(&self, key: &str, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_length(
                self.to_glib_none().0,
                key.to_glib_none().0,
                unit.into_glib(),
            )
        }
    }

    #[doc(alias = "gtk_print_settings_get_media_type")]
    pub fn media_type(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_media_type(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_settings_get_n_copies")]
    pub fn n_copies(&self) -> i32 {
        unsafe { ffi::gtk_print_settings_get_n_copies(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_settings_get_number_up")]
    pub fn number_up(&self) -> i32 {
        unsafe { ffi::gtk_print_settings_get_number_up(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_settings_get_number_up_layout")]
    pub fn number_up_layout(&self) -> NumberUpLayout {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_number_up_layout(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_settings_get_orientation")]
    pub fn orientation(&self) -> PageOrientation {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_orientation(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_settings_get_output_bin")]
    pub fn output_bin(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_output_bin(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_settings_get_page_ranges")]
    pub fn page_ranges(&self) -> Vec<PageRange> {
        unsafe {
            let mut num_ranges = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                ffi::gtk_print_settings_get_page_ranges(
                    self.to_glib_none().0,
                    num_ranges.as_mut_ptr(),
                ),
                num_ranges.assume_init() as usize,
            );
            ret
        }
    }

    #[doc(alias = "gtk_print_settings_get_page_set")]
    pub fn page_set(&self) -> PageSet {
        unsafe { from_glib(ffi::gtk_print_settings_get_page_set(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_settings_get_paper_height")]
    pub fn paper_height(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_print_settings_get_paper_height(self.to_glib_none().0, unit.into_glib()) }
    }

    #[doc(alias = "gtk_print_settings_get_paper_size")]
    pub fn paper_size(&self) -> PaperSize {
        unsafe {
            from_glib_full(ffi::gtk_print_settings_get_paper_size(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_settings_get_paper_width")]
    pub fn paper_width(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_print_settings_get_paper_width(self.to_glib_none().0, unit.into_glib()) }
    }

    #[doc(alias = "gtk_print_settings_get_print_pages")]
    pub fn print_pages(&self) -> PrintPages {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_print_pages(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_settings_get_printer")]
    pub fn printer(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_print_settings_get_printer(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_settings_get_printer_lpi")]
    pub fn printer_lpi(&self) -> f64 {
        unsafe { ffi::gtk_print_settings_get_printer_lpi(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_settings_get_quality")]
    pub fn quality(&self) -> PrintQuality {
        unsafe { from_glib(ffi::gtk_print_settings_get_quality(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_settings_get_resolution")]
    pub fn resolution(&self) -> i32 {
        unsafe { ffi::gtk_print_settings_get_resolution(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_settings_get_resolution_x")]
    pub fn resolution_x(&self) -> i32 {
        unsafe { ffi::gtk_print_settings_get_resolution_x(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_settings_get_resolution_y")]
    pub fn resolution_y(&self) -> i32 {
        unsafe { ffi::gtk_print_settings_get_resolution_y(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_settings_get_reverse")]
    pub fn is_reverse(&self) -> bool {
        unsafe { from_glib(ffi::gtk_print_settings_get_reverse(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_settings_get_scale")]
    pub fn scale(&self) -> f64 {
        unsafe { ffi::gtk_print_settings_get_scale(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_settings_get_use_color")]
    pub fn uses_color(&self) -> bool {
        unsafe { from_glib(ffi::gtk_print_settings_get_use_color(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_settings_has_key")]
    pub fn has_key(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_settings_has_key(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_settings_load_file")]
    pub fn load_file<P: AsRef<std::path::Path>>(&self, file_name: P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_print_settings_load_file(
                self.to_glib_none().0,
                file_name.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_print_settings_load_key_file")]
    pub fn load_key_file(
        &self,
        key_file: &glib::KeyFile,
        group_name: Option<&str>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_print_settings_load_key_file(
                self.to_glib_none().0,
                key_file.to_glib_none().0,
                group_name.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_print_settings_set")]
    pub fn set(&self, key: &str, value: Option<&str>) {
        unsafe {
            ffi::gtk_print_settings_set(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_print_settings_set_bool")]
    pub fn set_bool(&self, key: &str, value: bool) {
        unsafe {
            ffi::gtk_print_settings_set_bool(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_print_settings_set_collate")]
    pub fn set_collate(&self, collate: bool) {
        unsafe {
            ffi::gtk_print_settings_set_collate(self.to_glib_none().0, collate.into_glib());
        }
    }

    #[doc(alias = "gtk_print_settings_set_default_source")]
    pub fn set_default_source(&self, default_source: &str) {
        unsafe {
            ffi::gtk_print_settings_set_default_source(
                self.to_glib_none().0,
                default_source.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_print_settings_set_dither")]
    pub fn set_dither(&self, dither: &str) {
        unsafe {
            ffi::gtk_print_settings_set_dither(self.to_glib_none().0, dither.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_print_settings_set_double")]
    pub fn set_double(&self, key: &str, value: f64) {
        unsafe {
            ffi::gtk_print_settings_set_double(self.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    #[doc(alias = "gtk_print_settings_set_duplex")]
    pub fn set_duplex(&self, duplex: PrintDuplex) {
        unsafe {
            ffi::gtk_print_settings_set_duplex(self.to_glib_none().0, duplex.into_glib());
        }
    }

    #[doc(alias = "gtk_print_settings_set_finishings")]
    pub fn set_finishings(&self, finishings: &str) {
        unsafe {
            ffi::gtk_print_settings_set_finishings(
                self.to_glib_none().0,
                finishings.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_print_settings_set_int")]
    pub fn set_int(&self, key: &str, value: i32) {
        unsafe {
            ffi::gtk_print_settings_set_int(self.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    #[doc(alias = "gtk_print_settings_set_length")]
    pub fn set_length(&self, key: &str, value: f64, unit: Unit) {
        unsafe {
            ffi::gtk_print_settings_set_length(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value,
                unit.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_print_settings_set_media_type")]
    pub fn set_media_type(&self, media_type: &str) {
        unsafe {
            ffi::gtk_print_settings_set_media_type(
                self.to_glib_none().0,
                media_type.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_print_settings_set_n_copies")]
    pub fn set_n_copies(&self, num_copies: i32) {
        unsafe {
            ffi::gtk_print_settings_set_n_copies(self.to_glib_none().0, num_copies);
        }
    }

    #[doc(alias = "gtk_print_settings_set_number_up")]
    pub fn set_number_up(&self, number_up: i32) {
        unsafe {
            ffi::gtk_print_settings_set_number_up(self.to_glib_none().0, number_up);
        }
    }

    #[doc(alias = "gtk_print_settings_set_number_up_layout")]
    pub fn set_number_up_layout(&self, number_up_layout: NumberUpLayout) {
        unsafe {
            ffi::gtk_print_settings_set_number_up_layout(
                self.to_glib_none().0,
                number_up_layout.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_print_settings_set_orientation")]
    pub fn set_orientation(&self, orientation: PageOrientation) {
        unsafe {
            ffi::gtk_print_settings_set_orientation(self.to_glib_none().0, orientation.into_glib());
        }
    }

    #[doc(alias = "gtk_print_settings_set_output_bin")]
    pub fn set_output_bin(&self, output_bin: &str) {
        unsafe {
            ffi::gtk_print_settings_set_output_bin(
                self.to_glib_none().0,
                output_bin.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_print_settings_set_page_set")]
    pub fn set_page_set(&self, page_set: PageSet) {
        unsafe {
            ffi::gtk_print_settings_set_page_set(self.to_glib_none().0, page_set.into_glib());
        }
    }

    #[doc(alias = "gtk_print_settings_set_paper_height")]
    pub fn set_paper_height(&self, height: f64, unit: Unit) {
        unsafe {
            ffi::gtk_print_settings_set_paper_height(
                self.to_glib_none().0,
                height,
                unit.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_print_settings_set_paper_size")]
    pub fn set_paper_size(&self, paper_size: &PaperSize) {
        unsafe {
            ffi::gtk_print_settings_set_paper_size(
                self.to_glib_none().0,
                mut_override(paper_size.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "gtk_print_settings_set_paper_width")]
    pub fn set_paper_width(&self, width: f64, unit: Unit) {
        unsafe {
            ffi::gtk_print_settings_set_paper_width(self.to_glib_none().0, width, unit.into_glib());
        }
    }

    #[doc(alias = "gtk_print_settings_set_print_pages")]
    pub fn set_print_pages(&self, pages: PrintPages) {
        unsafe {
            ffi::gtk_print_settings_set_print_pages(self.to_glib_none().0, pages.into_glib());
        }
    }

    #[doc(alias = "gtk_print_settings_set_printer")]
    pub fn set_printer(&self, printer: &str) {
        unsafe {
            ffi::gtk_print_settings_set_printer(self.to_glib_none().0, printer.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_print_settings_set_printer_lpi")]
    pub fn set_printer_lpi(&self, lpi: f64) {
        unsafe {
            ffi::gtk_print_settings_set_printer_lpi(self.to_glib_none().0, lpi);
        }
    }

    #[doc(alias = "gtk_print_settings_set_quality")]
    pub fn set_quality(&self, quality: PrintQuality) {
        unsafe {
            ffi::gtk_print_settings_set_quality(self.to_glib_none().0, quality.into_glib());
        }
    }

    #[doc(alias = "gtk_print_settings_set_resolution")]
    pub fn set_resolution(&self, resolution: i32) {
        unsafe {
            ffi::gtk_print_settings_set_resolution(self.to_glib_none().0, resolution);
        }
    }

    #[doc(alias = "gtk_print_settings_set_resolution_xy")]
    pub fn set_resolution_xy(&self, resolution_x: i32, resolution_y: i32) {
        unsafe {
            ffi::gtk_print_settings_set_resolution_xy(
                self.to_glib_none().0,
                resolution_x,
                resolution_y,
            );
        }
    }

    #[doc(alias = "gtk_print_settings_set_reverse")]
    pub fn set_reverse(&self, reverse: bool) {
        unsafe {
            ffi::gtk_print_settings_set_reverse(self.to_glib_none().0, reverse.into_glib());
        }
    }

    #[doc(alias = "gtk_print_settings_set_scale")]
    pub fn set_scale(&self, scale: f64) {
        unsafe {
            ffi::gtk_print_settings_set_scale(self.to_glib_none().0, scale);
        }
    }

    #[doc(alias = "gtk_print_settings_set_use_color")]
    pub fn set_use_color(&self, use_color: bool) {
        unsafe {
            ffi::gtk_print_settings_set_use_color(self.to_glib_none().0, use_color.into_glib());
        }
    }

    #[doc(alias = "gtk_print_settings_to_file")]
    pub fn to_file<P: AsRef<std::path::Path>>(&self, file_name: P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_print_settings_to_file(
                self.to_glib_none().0,
                file_name.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_print_settings_to_gvariant")]
    pub fn to_gvariant(&self) -> Option<glib::Variant> {
        unsafe { from_glib_none(ffi::gtk_print_settings_to_gvariant(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_settings_to_key_file")]
    pub fn to_key_file(&self, key_file: &glib::KeyFile, group_name: Option<&str>) {
        unsafe {
            ffi::gtk_print_settings_to_key_file(
                self.to_glib_none().0,
                key_file.to_glib_none().0,
                group_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_print_settings_unset")]
    pub fn unset(&self, key: &str) {
        unsafe {
            ffi::gtk_print_settings_unset(self.to_glib_none().0, key.to_glib_none().0);
        }
    }
}

impl Default for PrintSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PrintSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PrintSettings")
    }
}
