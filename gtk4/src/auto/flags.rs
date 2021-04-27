// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

bitflags! {
    pub struct ApplicationInhibitFlags: u32 {
        const LOGOUT = 1;
        const SWITCH = 2;
        const SUSPEND = 4;
        const IDLE = 8;
    }
}

impl fmt::Display for ApplicationInhibitFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for ApplicationInhibitFlags {
    type GlibType = ffi::GtkApplicationInhibitFlags;

    fn into_glib(self) -> ffi::GtkApplicationInhibitFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkApplicationInhibitFlags> for ApplicationInhibitFlags {
    unsafe fn from_glib(value: ffi::GtkApplicationInhibitFlags) -> ApplicationInhibitFlags {
        skip_assert_initialized!();
        ApplicationInhibitFlags::from_bits_truncate(value)
    }
}

impl StaticType for ApplicationInhibitFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_application_inhibit_flags_get_type()) }
    }
}

impl glib::value::ValueType for ApplicationInhibitFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ApplicationInhibitFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for ApplicationInhibitFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<ApplicationInhibitFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct BuilderClosureFlags: u32 {
        const SWAPPED = 1;
    }
}

impl fmt::Display for BuilderClosureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for BuilderClosureFlags {
    type GlibType = ffi::GtkBuilderClosureFlags;

    fn into_glib(self) -> ffi::GtkBuilderClosureFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkBuilderClosureFlags> for BuilderClosureFlags {
    unsafe fn from_glib(value: ffi::GtkBuilderClosureFlags) -> BuilderClosureFlags {
        skip_assert_initialized!();
        BuilderClosureFlags::from_bits_truncate(value)
    }
}

impl StaticType for BuilderClosureFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_builder_closure_flags_get_type()) }
    }
}

impl glib::value::ValueType for BuilderClosureFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for BuilderClosureFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for BuilderClosureFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<BuilderClosureFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct CellRendererState: u32 {
        const SELECTED = 1;
        const PRELIT = 2;
        const INSENSITIVE = 4;
        const SORTED = 8;
        const FOCUSED = 16;
        const EXPANDABLE = 32;
        const EXPANDED = 64;
    }
}

impl fmt::Display for CellRendererState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for CellRendererState {
    type GlibType = ffi::GtkCellRendererState;

    fn into_glib(self) -> ffi::GtkCellRendererState {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkCellRendererState> for CellRendererState {
    unsafe fn from_glib(value: ffi::GtkCellRendererState) -> CellRendererState {
        skip_assert_initialized!();
        CellRendererState::from_bits_truncate(value)
    }
}

impl StaticType for CellRendererState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_cell_renderer_state_get_type()) }
    }
}

impl glib::value::ValueType for CellRendererState {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for CellRendererState {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for CellRendererState {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<CellRendererState>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct DebugFlags: u32 {
        const TEXT = 1;
        const TREE = 2;
        const KEYBINDINGS = 4;
        const MODULES = 8;
        const GEOMETRY = 16;
        const ICONTHEME = 32;
        const PRINTING = 64;
        const BUILDER = 128;
        const SIZE_REQUEST = 256;
        const NO_CSS_CACHE = 512;
        const INTERACTIVE = 1024;
        const TOUCHSCREEN = 2048;
        const ACTIONS = 4096;
        const LAYOUT = 8192;
        const SNAPSHOT = 16384;
        const CONSTRAINTS = 32768;
        const BUILDER_OBJECTS = 65536;
        const A11Y = 131072;
    }
}

impl fmt::Display for DebugFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for DebugFlags {
    type GlibType = ffi::GtkDebugFlags;

    fn into_glib(self) -> ffi::GtkDebugFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkDebugFlags> for DebugFlags {
    unsafe fn from_glib(value: ffi::GtkDebugFlags) -> DebugFlags {
        skip_assert_initialized!();
        DebugFlags::from_bits_truncate(value)
    }
}

impl StaticType for DebugFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_debug_flags_get_type()) }
    }
}

impl glib::value::ValueType for DebugFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for DebugFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for DebugFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<DebugFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct DialogFlags: u32 {
        const MODAL = 1;
        const DESTROY_WITH_PARENT = 2;
        const USE_HEADER_BAR = 4;
    }
}

impl fmt::Display for DialogFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for DialogFlags {
    type GlibType = ffi::GtkDialogFlags;

    fn into_glib(self) -> ffi::GtkDialogFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkDialogFlags> for DialogFlags {
    unsafe fn from_glib(value: ffi::GtkDialogFlags) -> DialogFlags {
        skip_assert_initialized!();
        DialogFlags::from_bits_truncate(value)
    }
}

impl StaticType for DialogFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_dialog_flags_get_type()) }
    }
}

impl glib::value::ValueType for DialogFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for DialogFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for DialogFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<DialogFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct EventControllerScrollFlags: u32 {
        const NONE = 0;
        const VERTICAL = 1;
        const HORIZONTAL = 2;
        const DISCRETE = 4;
        const KINETIC = 8;
        const BOTH_AXES = 3;
    }
}

impl fmt::Display for EventControllerScrollFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for EventControllerScrollFlags {
    type GlibType = ffi::GtkEventControllerScrollFlags;

    fn into_glib(self) -> ffi::GtkEventControllerScrollFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkEventControllerScrollFlags> for EventControllerScrollFlags {
    unsafe fn from_glib(value: ffi::GtkEventControllerScrollFlags) -> EventControllerScrollFlags {
        skip_assert_initialized!();
        EventControllerScrollFlags::from_bits_truncate(value)
    }
}

impl StaticType for EventControllerScrollFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_event_controller_scroll_flags_get_type()) }
    }
}

impl glib::value::ValueType for EventControllerScrollFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for EventControllerScrollFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for EventControllerScrollFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<EventControllerScrollFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct FontChooserLevel: u32 {
        const FAMILY = 0;
        const STYLE = 1;
        const SIZE = 2;
        const VARIATIONS = 4;
        const FEATURES = 8;
    }
}

impl fmt::Display for FontChooserLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for FontChooserLevel {
    type GlibType = ffi::GtkFontChooserLevel;

    fn into_glib(self) -> ffi::GtkFontChooserLevel {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkFontChooserLevel> for FontChooserLevel {
    unsafe fn from_glib(value: ffi::GtkFontChooserLevel) -> FontChooserLevel {
        skip_assert_initialized!();
        FontChooserLevel::from_bits_truncate(value)
    }
}

impl StaticType for FontChooserLevel {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_font_chooser_level_get_type()) }
    }
}

impl glib::value::ValueType for FontChooserLevel {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for FontChooserLevel {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for FontChooserLevel {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<FontChooserLevel>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct IconLookupFlags: u32 {
        const FORCE_REGULAR = 1;
        const FORCE_SYMBOLIC = 2;
        const PRELOAD = 4;
    }
}

impl fmt::Display for IconLookupFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for IconLookupFlags {
    type GlibType = ffi::GtkIconLookupFlags;

    fn into_glib(self) -> ffi::GtkIconLookupFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkIconLookupFlags> for IconLookupFlags {
    unsafe fn from_glib(value: ffi::GtkIconLookupFlags) -> IconLookupFlags {
        skip_assert_initialized!();
        IconLookupFlags::from_bits_truncate(value)
    }
}

impl StaticType for IconLookupFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_icon_lookup_flags_get_type()) }
    }
}

impl glib::value::ValueType for IconLookupFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for IconLookupFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for IconLookupFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<IconLookupFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct InputHints: u32 {
        const NONE = 0;
        const SPELLCHECK = 1;
        const NO_SPELLCHECK = 2;
        const WORD_COMPLETION = 4;
        const LOWERCASE = 8;
        const UPPERCASE_CHARS = 16;
        const UPPERCASE_WORDS = 32;
        const UPPERCASE_SENTENCES = 64;
        const INHIBIT_OSK = 128;
        const VERTICAL_WRITING = 256;
        const EMOJI = 512;
        const NO_EMOJI = 1024;
        const PRIVATE = 2048;
    }
}

impl fmt::Display for InputHints {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for InputHints {
    type GlibType = ffi::GtkInputHints;

    fn into_glib(self) -> ffi::GtkInputHints {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkInputHints> for InputHints {
    unsafe fn from_glib(value: ffi::GtkInputHints) -> InputHints {
        skip_assert_initialized!();
        InputHints::from_bits_truncate(value)
    }
}

impl StaticType for InputHints {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_input_hints_get_type()) }
    }
}

impl glib::value::ValueType for InputHints {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InputHints {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for InputHints {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<InputHints>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct PickFlags: u32 {
        const DEFAULT = 0;
        const INSENSITIVE = 1;
        const NON_TARGETABLE = 2;
    }
}

impl fmt::Display for PickFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for PickFlags {
    type GlibType = ffi::GtkPickFlags;

    fn into_glib(self) -> ffi::GtkPickFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkPickFlags> for PickFlags {
    unsafe fn from_glib(value: ffi::GtkPickFlags) -> PickFlags {
        skip_assert_initialized!();
        PickFlags::from_bits_truncate(value)
    }
}

impl StaticType for PickFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_pick_flags_get_type()) }
    }
}

impl glib::value::ValueType for PickFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PickFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for PickFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<PickFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct PopoverMenuFlags: u32 {
        const NESTED = 1;
    }
}

impl fmt::Display for PopoverMenuFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for PopoverMenuFlags {
    type GlibType = ffi::GtkPopoverMenuFlags;

    fn into_glib(self) -> ffi::GtkPopoverMenuFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkPopoverMenuFlags> for PopoverMenuFlags {
    unsafe fn from_glib(value: ffi::GtkPopoverMenuFlags) -> PopoverMenuFlags {
        skip_assert_initialized!();
        PopoverMenuFlags::from_bits_truncate(value)
    }
}

impl StaticType for PopoverMenuFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_popover_menu_flags_get_type()) }
    }
}

impl glib::value::ValueType for PopoverMenuFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PopoverMenuFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for PopoverMenuFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<PopoverMenuFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct ShortcutActionFlags: u32 {
        const EXCLUSIVE = 1;
    }
}

impl fmt::Display for ShortcutActionFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for ShortcutActionFlags {
    type GlibType = ffi::GtkShortcutActionFlags;

    fn into_glib(self) -> ffi::GtkShortcutActionFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkShortcutActionFlags> for ShortcutActionFlags {
    unsafe fn from_glib(value: ffi::GtkShortcutActionFlags) -> ShortcutActionFlags {
        skip_assert_initialized!();
        ShortcutActionFlags::from_bits_truncate(value)
    }
}

impl StaticType for ShortcutActionFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_shortcut_action_flags_get_type()) }
    }
}

impl glib::value::ValueType for ShortcutActionFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ShortcutActionFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for ShortcutActionFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<ShortcutActionFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct StateFlags: u32 {
        const NORMAL = 0;
        const ACTIVE = 1;
        const PRELIGHT = 2;
        const SELECTED = 4;
        const INSENSITIVE = 8;
        const INCONSISTENT = 16;
        const FOCUSED = 32;
        const BACKDROP = 64;
        const DIR_LTR = 128;
        const DIR_RTL = 256;
        const LINK = 512;
        const VISITED = 1024;
        const CHECKED = 2048;
        const DROP_ACTIVE = 4096;
        const FOCUS_VISIBLE = 8192;
        const FOCUS_WITHIN = 16384;
    }
}

impl fmt::Display for StateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for StateFlags {
    type GlibType = ffi::GtkStateFlags;

    fn into_glib(self) -> ffi::GtkStateFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkStateFlags> for StateFlags {
    unsafe fn from_glib(value: ffi::GtkStateFlags) -> StateFlags {
        skip_assert_initialized!();
        StateFlags::from_bits_truncate(value)
    }
}

impl StaticType for StateFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_state_flags_get_type()) }
    }
}

impl glib::value::ValueType for StateFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for StateFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for StateFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<StateFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct StyleContextPrintFlags: u32 {
        const NONE = 0;
        const RECURSE = 1;
        const SHOW_STYLE = 2;
        const SHOW_CHANGE = 4;
    }
}

impl fmt::Display for StyleContextPrintFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for StyleContextPrintFlags {
    type GlibType = ffi::GtkStyleContextPrintFlags;

    fn into_glib(self) -> ffi::GtkStyleContextPrintFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkStyleContextPrintFlags> for StyleContextPrintFlags {
    unsafe fn from_glib(value: ffi::GtkStyleContextPrintFlags) -> StyleContextPrintFlags {
        skip_assert_initialized!();
        StyleContextPrintFlags::from_bits_truncate(value)
    }
}

impl StaticType for StyleContextPrintFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_style_context_print_flags_get_type()) }
    }
}

impl glib::value::ValueType for StyleContextPrintFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for StyleContextPrintFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for StyleContextPrintFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<StyleContextPrintFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct TextSearchFlags: u32 {
        const VISIBLE_ONLY = 1;
        const TEXT_ONLY = 2;
        const CASE_INSENSITIVE = 4;
    }
}

impl fmt::Display for TextSearchFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for TextSearchFlags {
    type GlibType = ffi::GtkTextSearchFlags;

    fn into_glib(self) -> ffi::GtkTextSearchFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkTextSearchFlags> for TextSearchFlags {
    unsafe fn from_glib(value: ffi::GtkTextSearchFlags) -> TextSearchFlags {
        skip_assert_initialized!();
        TextSearchFlags::from_bits_truncate(value)
    }
}

impl StaticType for TextSearchFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_text_search_flags_get_type()) }
    }
}

impl glib::value::ValueType for TextSearchFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for TextSearchFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for TextSearchFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<TextSearchFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct TreeModelFlags: u32 {
        const ITERS_PERSIST = 1;
        const LIST_ONLY = 2;
    }
}

impl fmt::Display for TreeModelFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for TreeModelFlags {
    type GlibType = ffi::GtkTreeModelFlags;

    fn into_glib(self) -> ffi::GtkTreeModelFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkTreeModelFlags> for TreeModelFlags {
    unsafe fn from_glib(value: ffi::GtkTreeModelFlags) -> TreeModelFlags {
        skip_assert_initialized!();
        TreeModelFlags::from_bits_truncate(value)
    }
}

impl StaticType for TreeModelFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_tree_model_flags_get_type()) }
    }
}

impl glib::value::ValueType for TreeModelFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for TreeModelFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for TreeModelFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<TreeModelFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
