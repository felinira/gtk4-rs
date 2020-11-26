use crate::EntryBuffer;
use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, GString, Object};

pub trait EntryBufferImpl: EntryBufferImplExt + ObjectImpl {
    fn delete_text(&self, entry_buffer: &Self::Type, position: u32, n_chars: u32) -> u32 {
        self.parent_delete_text(entry_buffer, position, n_chars)
    }

    fn deleted_text(&self, entry_buffer: &Self::Type, position: u32, n_chars: u32) {
        self.parent_deleted_text(entry_buffer, position, n_chars)
    }

    fn get_length(&self, entry_buffer: &Self::Type) -> u32 {
        self.parent_get_length(entry_buffer)
    }

    fn get_text(&self, entry_buffer: &Self::Type) -> GString {
        self.parent_get_text(entry_buffer)
    }
    fn insert_text(&self, entry_buffer: &Self::Type, position: u32, chars: &str) -> u32 {
        self.parent_insert_text(entry_buffer, position, chars)
    }

    fn inserted_text(&self, entry_buffer: &Self::Type, position: u32, chars: &str) {
        self.parent_inserted_text(entry_buffer, position, chars)
    }
}

pub trait EntryBufferImplExt: ObjectSubclass {
    fn parent_delete_text(&self, entry_buffer: &Self::Type, position: u32, n_chars: u32) -> u32;
    fn parent_deleted_text(&self, entry_buffer: &Self::Type, position: u32, n_chars: u32);
    fn parent_get_length(&self, entry_buffer: &Self::Type) -> u32;
    fn parent_get_text(&self, entry_buffer: &Self::Type) -> GString;
    fn parent_insert_text(&self, entry_buffer: &Self::Type, position: u32, chars: &str) -> u32;
    fn parent_inserted_text(&self, entry_buffer: &Self::Type, position: u32, chars: &str);
}

impl<T: EntryBufferImpl> EntryBufferImplExt for T {
    fn parent_delete_text(&self, entry_buffer: &Self::Type, position: u32, n_chars: u32) -> u32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkEntryBufferClass;
            let f = (*parent_class)
                .delete_text
                .expect("No parent class impl for \"delete_text\"");
            f(
                entry_buffer
                    .unsafe_cast_ref::<EntryBuffer>()
                    .to_glib_none()
                    .0,
                position,
                n_chars,
            )
        }
    }

    fn parent_deleted_text(&self, entry_buffer: &Self::Type, position: u32, n_chars: u32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkEntryBufferClass;
            if let Some(f) = (*parent_class).deleted_text {
                f(
                    entry_buffer
                        .unsafe_cast_ref::<EntryBuffer>()
                        .to_glib_none()
                        .0,
                    position,
                    n_chars,
                )
            }
        }
    }

    fn parent_get_length(&self, entry_buffer: &Self::Type) -> u32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkEntryBufferClass;
            let f = (*parent_class)
                .get_length
                .expect("No parent class impl for \"get_length\"");
            f(entry_buffer
                .unsafe_cast_ref::<EntryBuffer>()
                .to_glib_none()
                .0)
        }
    }

    fn parent_get_text(&self, entry_buffer: &Self::Type) -> GString {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkEntryBufferClass;
            let f = (*parent_class)
                .get_text
                .expect("No parent class impl for \"get_text\"");
            let mut n_bytes = 0;
            let res = f(
                entry_buffer
                    .unsafe_cast_ref::<EntryBuffer>()
                    .to_glib_none()
                    .0,
                &mut n_bytes,
            );
            FromGlibContainer::from_glib_none_num(res, n_bytes as usize)
        }
    }

    fn parent_insert_text(&self, entry_buffer: &Self::Type, position: u32, chars: &str) -> u32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkEntryBufferClass;
            let f = (*parent_class)
                .insert_text
                .expect("No parent class impl for \"insert_text\"");

            f(
                entry_buffer
                    .unsafe_cast_ref::<EntryBuffer>()
                    .to_glib_none()
                    .0,
                position,
                chars.to_glib_none().0,
                chars.len() as u32,
            )
        }
    }

    fn parent_inserted_text(&self, entry_buffer: &Self::Type, position: u32, chars: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkEntryBufferClass;
            if let Some(f) = (*parent_class).inserted_text {
                f(
                    entry_buffer
                        .unsafe_cast_ref::<EntryBuffer>()
                        .to_glib_none()
                        .0,
                    position,
                    chars.to_glib_none().0,
                    chars.len() as u32,
                )
            }
        }
    }
}

unsafe impl<T: EntryBufferImpl> IsSubclassable<T> for EntryBuffer {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.delete_text = Some(entry_buffer_delete_text::<T>);
        klass.deleted_text = Some(entry_buffer_deleted_text::<T>);
        klass.get_length = Some(entry_buffer_get_length::<T>);
        klass.get_text = Some(entry_buffer_get_text::<T>);
        klass.insert_text = Some(entry_buffer_insert_text::<T>);
        klass.inserted_text = Some(entry_buffer_inserted_text::<T>);
    }
}

unsafe extern "C" fn entry_buffer_delete_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    position: u32,
    n_chars: u32,
) -> u32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<EntryBuffer> = from_glib_borrow(ptr);

    imp.delete_text(wrap.unsafe_cast_ref(), position, n_chars)
}

unsafe extern "C" fn entry_buffer_deleted_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    position: u32,
    n_chars: u32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<EntryBuffer> = from_glib_borrow(ptr);

    imp.deleted_text(wrap.unsafe_cast_ref(), position, n_chars)
}

unsafe extern "C" fn entry_buffer_get_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    _n_bytes: *mut usize,
) -> *const libc::c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<EntryBuffer> = from_glib_borrow(ptr);

    imp.get_text(wrap.unsafe_cast_ref()).to_glib_full()
}

unsafe extern "C" fn entry_buffer_get_length<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
) -> u32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<EntryBuffer> = from_glib_borrow(ptr);

    imp.get_length(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn entry_buffer_insert_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    position: u32,
    charsptr: *const libc::c_char,
    n_chars: u32,
) -> u32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<EntryBuffer> = from_glib_borrow(ptr);
    let chars: Borrowed<GString> = from_glib_borrow(charsptr);

    imp.insert_text(
        wrap.unsafe_cast_ref(),
        position,
        &chars[0..(n_chars as usize)],
    )
}

unsafe extern "C" fn entry_buffer_inserted_text<T: EntryBufferImpl>(
    ptr: *mut ffi::GtkEntryBuffer,
    position: u32,
    charsptr: *const libc::c_char,
    length: u32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<EntryBuffer> = from_glib_borrow(ptr);
    let chars: Borrowed<GString> = from_glib_borrow(charsptr);

    imp.inserted_text(
        wrap.unsafe_cast_ref(),
        position,
        &chars[0..(length as usize)],
    )
}
