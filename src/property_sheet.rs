//! Property Sheet.
//!
#![allow(dead_code)]
use std::cell::{Cell, Ref, RefCell, RefMut, UnsafeCell};
use std::fmt::Debug;
use std::sync::Arc;

/// Property.
pub trait Property {
    /// Returns the `id` of the property.
    fn id(&self) -> usize {
        0
    }

    /// Change the `id` of the property.
    fn set_id(&self, _id: usize) {}

    /// Returns the name of the Property.
    fn name(&self) -> &'static str;

    /// Returns the Options of the Property.
    fn options(&self) -> &[&'static str] {
        &[]
    }

    /// Returns the Type of the Property Value.
    fn value_type(&self) -> ValueType {
        ValueType::Unknown
    }

    /// Returns which Type of Widget to render the Property.
    fn widget_type(&self) -> WidgetType {
        WidgetType::Unknown
    }

    /// Returns `true` if the property can marked with `selected`.
    fn is_selectable(&self) -> bool {
        self.widget_type() != WidgetType::Separator
    }

    /// Returns `true` if the property marked with `selected`.
    fn is_selected(&self) -> bool {
        false
    }

    /// Change the `selected` marker of the property.
    fn set_selected(&self, _selected: bool) {}

    /// Returns `true` if the property visibile.
    fn is_visible(&self) -> bool {
        true
    }

    /// Change the visibility of the property.
    fn set_visible(&self, _visible: bool) {}

    /// Change the visibility of the property to `true`.
    fn show(&self) {}

    /// Change the visibility of the property to `false`.
    fn hide(&self) {}

    /// Casting to PropertyAction.
    fn as_property_action(&self) -> Option<&PropertyAction> {
        None
    }

    /// Casting to PropertyBool.
    fn as_property_bool(&self) -> Option<&PropertyBool> {
        None
    }

    /// Casting to dyn PropertyNumber<f32>.
    fn as_property_f32<'l>(&self) -> Option<&(dyn PropertyNumber<f32> + 'l)> {
        None
    }

    /// Casting to dyn PropertyNumber<f64>.
    fn as_property_f64<'l>(&self) -> Option<&(dyn PropertyNumber<f64> + 'l)> {
        None
    }

    /// Casting to dyn PropertyNumber<i32>.
    fn as_property_i32<'l>(&self) -> Option<&(dyn PropertyNumber<i32> + 'l)> {
        None
    }

    /// Casting to dyn PropertyNumber<i64>.
    fn as_property_i64<'l>(&self) -> Option<&(dyn PropertyNumber<i64> + 'l)> {
        None
    }

    /// Casting to PropertyString.
    fn as_property_string(&self) -> Option<&PropertyString> {
        None
    }

    /// Returns the `checked` state if the property is type of `ValueType::Action`.
    fn is_action_checked(&self) -> Option<bool> {
        if let Some(p) = self.as_property_action() {
            Some(p.is_checked())
        } else {
            None
        }
    }

    /// Trigger the action if the property is type of `ValueType::Action`.
    fn trigger_action(&self, checked: bool) -> Option<bool> {
        if let Some(p) = self.as_property_action() {
            Some(p.trigger(checked))
        } else {
            None
        }
    }

    /// Returns the `bool` value if the property is type of `ValueType::Bool`.
    fn get_value_bool(&self) -> Option<bool> {
        if let Some(p) = self.as_property_bool() {
            Some(p.value())
        } else {
            None
        }
    }

    /// Change the `bool` value if the property is type of `ValueType::Bool`.
    fn set_value_bool(&self, value: bool) -> Option<bool> {
        if let Some(p) = self.as_property_bool() {
            Some(p.set_value(value))
        } else {
            None
        }
    }

    /// Returns the `f32` value if the property is type of `ValueType::F32`.
    fn get_value_f32(&self) -> Option<f32> {
        if let Some(p) = self.as_property_f32() {
            Some(p.value())
        } else {
            None
        }
    }

    /// Change the `f32` value if the property is type of `ValueType::F32`.
    fn set_value_f32(&self, value: f32) -> Option<f32> {
        if let Some(p) = self.as_property_f32() {
            Some(p.set_value(value))
        } else {
            None
        }
    }

    /// Returns the `f64` value if the property is type of `ValueType::F64`.
    fn get_value_f64(&self) -> Option<f64> {
        if let Some(p) = self.as_property_f64() {
            Some(p.value())
        } else {
            None
        }
    }

    /// Change the `f64` value if the property is type of `ValueType::F64`.
    fn set_value_f64(&self, value: f64) -> Option<f64> {
        if let Some(p) = self.as_property_f64() {
            Some(p.set_value(value))
        } else {
            None
        }
    }

    /// Returns the `i32` value if the property is type of `ValueType::I32`.
    fn get_value_i32(&self) -> Option<i32> {
        if let Some(p) = self.as_property_i32() {
            Some(p.value())
        } else {
            None
        }
    }

    /// Change the `i32` value if the property is type of `ValueType::I32`.
    fn set_value_i32(&self, value: i32) -> Option<i32> {
        if let Some(p) = self.as_property_i32() {
            Some(p.set_value(value))
        } else {
            None
        }
    }

    /// Returns the `i64` value if the property is type of `ValueType::I64`.
    fn get_value_i64(&self) -> Option<i64> {
        if let Some(p) = self.as_property_i64() {
            Some(p.value())
        } else {
            None
        }
    }

    /// Change the `i64` value if the property is type of `ValueType::I64`.
    fn set_value_i64(&self, value: i64) -> Option<i64> {
        if let Some(p) = self.as_property_i64() {
            Some(p.set_value(value))
        } else {
            None
        }
    }

    /// Returns the `&str` value if the property is type of `ValueType::String`.
    fn get_value_string(&self) -> Option<Ref<'_, str>> {
        if let Some(p) = self.as_property_string() {
            Some(p.value())
        } else {
            None
        }
    }

    fn set_value_string<'l>(&self, value: &'l str) -> Option<Ref<'_, str>> {
        if let Some(p) = self.as_property_string() {
            Some(p.set_value(value))
        } else {
            None
        }
    }
}

impl Debug for dyn Property + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("Property");
        ds.field("name", &self.name())
            .field("options", &self.options())
            .field("value_type", &self.value_type())
            .field("widget_type", &self.widget_type());
        match self.value_type() {
            ValueType::Action => {
                let p = self.as_property_action().unwrap();
                ds.field("checked", &p.is_checked());
            }
            ValueType::Bool => {
                let p = self.as_property_bool().unwrap();
                ds.field("def_val", &p.def_val());
            }
            ValueType::F32 => {
                let p = self.as_property_f32().unwrap();
                ds.field("range", &p.range())
                    .field("step", &p.step())
                    .field("def_val", &p.def_val());
            }
            ValueType::F64 => {
                let p = self.as_property_f64().unwrap();
                ds.field("range", &p.range())
                    .field("step", &p.step())
                    .field("def_val", &p.def_val());
            }
            ValueType::I32 => {
                let p = self.as_property_i32().unwrap();
                ds.field("range", &p.range())
                    .field("step", &p.step())
                    .field("def_val", &p.def_val());
            }
            ValueType::I64 => {
                let p = self.as_property_i64().unwrap();
                ds.field("range", &p.range())
                    .field("step", &p.step())
                    .field("def_val", &p.def_val());
            }
            ValueType::String => {
                let p = self.as_property_string().unwrap();
                ds.field("max_length", &p.max_length())
                    .field("def_val", &p.def_val());
            }
            _ => {}
        }
        ds.finish()
    }
}

/// Property Base Attributes.
#[derive(Clone, Debug, Default)]
pub struct PropertyBase {
    id: Cell<usize>,
    name: &'static str,
    options: Vec<&'static str>,
    value_type: ValueType,
    widget_type: WidgetType,
    selected: Cell<bool>,
    visible: Cell<bool>,
}

impl Property for PropertyBase {
    fn id(&self) -> usize {
        self.id.get()
    }

    fn set_id(&self, id: usize) {
        self.id.set(id)
    }

    fn name(&self) -> &'static str {
        &self.name
    }

    fn options(&self) -> &[&'static str] {
        &self.options
    }

    fn value_type(&self) -> ValueType {
        self.value_type
    }

    fn widget_type(&self) -> WidgetType {
        self.widget_type
    }

    fn is_selectable(&self) -> bool {
        self.widget_type != WidgetType::Separator
    }

    fn is_selected(&self) -> bool {
        self.selected.get()
    }

    fn set_selected(&self, selected: bool) {
        self.selected.set(selected)
    }

    fn is_visible(&self) -> bool {
        self.visible.get()
    }

    fn set_visible(&self, visible: bool) {
        self.visible.set(visible)
    }

    fn show(&self) {
        self.visible.set(true);
    }

    fn hide(&self) {
        self.visible.set(false);
    }
}

impl PropertyBase {
    pub fn new(
        name: &'static str,
        options: &[&'static str],
        value_type: ValueType,
        widget_type: WidgetType,
    ) -> Self {
        Self {
            id: Cell::new(0),
            name,
            options: options.to_vec(),
            value_type,
            widget_type,
            selected: Cell::new(false),
            visible: Cell::new(true),
        }
    }

    pub fn with_action_button(name: &'static str, options: &[&'static str]) -> Self {
        Self::new(name, options, ValueType::Action, WidgetType::Button)
    }

    pub fn with_action_check_box(name: &'static str) -> Self {
        Self::new(name, &[], ValueType::Action, WidgetType::CheckBox)
    }

    pub fn with_slider_f32(name: &'static str) -> Self {
        Self::new(name, &[], ValueType::F32, WidgetType::Slider)
    }

    pub fn with_slider_f64(name: &'static str) -> Self {
        Self::new(name, &[], ValueType::F64, WidgetType::Slider)
    }

    pub fn with_combo_box_i32(name: &'static str, options: &[&'static str]) -> Self {
        Self::new(name, options, ValueType::I32, WidgetType::ComboBox)
    }

    pub fn with_combo_box_i64(name: &'static str, options: &[&'static str]) -> Self {
        Self::new(name, options, ValueType::I64, WidgetType::ComboBox)
    }

    pub fn with_select_i32(name: &'static str, options: &[&'static str]) -> Self {
        Self::new(name, options, ValueType::I32, WidgetType::Select)
    }

    pub fn with_select_i64(name: &'static str, options: &[&'static str]) -> Self {
        Self::new(name, options, ValueType::I64, WidgetType::Select)
    }

    pub fn with_slider_i32(name: &'static str) -> Self {
        Self::new(name, &[], ValueType::I32, WidgetType::Slider)
    }

    pub fn with_slider_i64(name: &'static str) -> Self {
        Self::new(name, &[], ValueType::I64, WidgetType::Slider)
    }

    pub fn with_switch(name: &'static str) -> Self {
        Self::new(name, &[], ValueType::Bool, WidgetType::Switch)
    }

    pub fn with_text_box(name: &'static str) -> Self {
        Self::new(name, &[], ValueType::String, WidgetType::TextBox)
    }
}

macro_rules! wrap_property_base {
    () => {
        #[inline]
        fn id(&self) -> usize {
            self.base.id()
        }

        #[inline]
        fn set_id(&self, id: usize) {
            self.base.set_id(id)
        }

        #[inline]
        fn name(&self) -> &'static str {
            self.base.name()
        }

        #[inline]
        fn options(&self) -> &[&'static str] {
            self.base.options()
        }

        #[inline]
        fn value_type(&self) -> ValueType {
            self.base.value_type()
        }

        #[inline]
        fn widget_type(&self) -> WidgetType {
            self.base.widget_type()
        }

        #[inline]
        fn is_selected(&self) -> bool {
            self.base.is_selected()
        }

        #[inline]
        fn set_selected(&self, selected: bool) {
            self.base.set_selected(selected)
        }

        #[inline]
        fn is_visible(&self) -> bool {
            self.base.is_visible()
        }

        #[inline]
        fn set_visible(&self, visible: bool) {
            self.base.set_visible(visible)
        }

        #[inline]
        fn show(&self) {
            self.base.show()
        }

        #[inline]
        fn hide(&self) {
            self.base.hide()
        }
    };
}

type ActionCallback = dyn FnMut(&dyn Property, bool) -> bool + 'static;

/// Action Property.
pub struct PropertyAction {
    base: PropertyBase,
    checked: Cell<bool>,
    callback: Arc<RefCell<ActionCallback>>,
}

unsafe impl Send for PropertyAction {}
unsafe impl Sync for PropertyAction {}

impl Debug for PropertyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PropertyAction")
            .field("name", &self.name())
            .field("widget_type", &self.widget_type())
            .finish()
    }
}

impl Default for PropertyAction {
    fn default() -> Self {
        Self {
            base: PropertyBase::with_action_button("UnTitled", &["Click Me"]),
            checked: Cell::new(false),
            callback: Arc::new(RefCell::new(
                |_prop: &dyn Property, checked: bool| -> bool { checked },
            )),
        }
    }
}

impl Property for PropertyAction {
    wrap_property_base!();

    #[inline]
    fn as_property_action(&self) -> Option<&PropertyAction> {
        Some(self)
    }
}

impl PropertyAction {
    /// Create an Action Property with Push Button.
    #[inline]
    pub fn with_button<F>(name: &'static str, text: &'static str, callback: Arc<RefCell<F>>) -> Self
    where
        F: FnMut(&dyn Property, bool) -> bool + 'static,
    {
        Self {
            base: PropertyBase::with_action_button(name, &[text]),
            checked: Cell::new(false),
            callback,
        }
    }

    /// Create an Action Property with Check Box.
    #[inline]
    pub fn with_check_box<F>(name: &'static str, checked: bool, callback: Arc<RefCell<F>>) -> Self
    where
        F: FnMut(&dyn Property, bool) -> bool + 'static,
    {
        Self {
            base: PropertyBase::with_action_check_box(name),
            checked: Cell::new(checked),
            callback,
        }
    }

    /// Returns current check state.
    pub fn is_checked(&self) -> bool {
        self.checked.get()
    }

    /// Trigger the action callback and returning the final check state.
    pub fn trigger(&self, checked: bool) -> bool {
        let caller = &mut *self.callback.borrow_mut();
        let result = (caller)(self, checked);
        self.checked.set(result);
        result
    }
}

/// Bool Property.
pub struct PropertyBool {
    base: PropertyBase,
    def_val: bool,
    value: UnsafeCell<bool>,
}

unsafe impl Send for PropertyBool {}
unsafe impl Sync for PropertyBool {}

impl Debug for PropertyBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PropertyBool")
            .field("name", &self.name())
            .field("widget_type", &self.widget_type())
            .field("def_val", &self.def_val())
            .field("value", &self.value())
            .finish()
    }
}

impl Default for PropertyBool {
    fn default() -> Self {
        Self {
            base: PropertyBase::with_switch("UnTitled"),
            def_val: false,
            value: UnsafeCell::new(false),
        }
    }
}

impl Property for PropertyBool {
    wrap_property_base!();

    #[inline]
    fn as_property_bool(&self) -> Option<&PropertyBool> {
        Some(self)
    }
}

impl PropertyBool {
    #[inline]
    pub fn with_switch(name: &'static str, def_val: bool) -> Self {
        Self {
            base: PropertyBase::with_switch(name),
            def_val,
            value: UnsafeCell::new(def_val),
        }
    }

    #[inline]
    pub fn def_val(&self) -> bool {
        self.def_val
    }

    #[inline]
    pub fn value(&self) -> bool {
        unsafe { self.value.get().read() }
    }

    #[allow(clippy::mut_from_ref)]
    #[inline]
    pub fn value_mut(&self) -> &mut bool {
        unsafe { &mut (*self.value.get()) }
    }

    /// # Safety
    #[inline]
    pub unsafe fn value_mut_ptr(&self) -> *mut bool {
        self.value.get()
    }

    #[inline]
    pub fn set_value(&self, value: bool) -> bool {
        unsafe {
            self.value.get().write(value);
        }
        value
    }
}

/// Numberic Property.
pub trait PropertyNumber<T>: Property {
    /// Returns the min/max range of the property value.
    fn range(&self) -> (T, T);

    /// Returns the increase/decrease step of the property value.
    fn step(&self) -> T;

    /// Increase the value of the property by step and return the new value.
    fn step_forward(&self) -> T;

    /// Decrease the value of the property by step and return the new value.
    fn step_backward(&self) -> T;

    /// Returns the default value of the property.
    fn def_val(&self) -> T;

    /// Returns the value of the property.
    fn value(&self) -> T;

    /// Returns the mutable reference of the property value.
    #[allow(clippy::mut_from_ref)]
    fn value_mut(&self) -> &mut T;

    /// Returns the mutable raw pointer of the property value.
    /// # Safety
    unsafe fn value_mut_ptr(&self) -> *mut T;

    /// Change the value of the property.
    fn set_value(&self, value: T) -> T;
}

impl Debug for dyn PropertyNumber<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PropertyF32")
            .field("name", &self.name())
            .field("widget_type", &self.widget_type())
            .field("range", &self.range())
            .field("step", &self.step())
            .field("def_val", &self.def_val())
            .field("value", &self.value())
            .finish()
    }
}

impl Debug for dyn PropertyNumber<f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PropertyF32")
            .field("name", &self.name())
            .field("widget_type", &self.widget_type())
            .field("range", &self.range())
            .field("step", &self.step())
            .field("def_val", &self.def_val())
            .field("value", &self.value())
            .finish()
    }
}

/// Float32 Property.
#[derive(Debug)]
pub struct PropertyF32 {
    base: PropertyBase,
    range: (f32, f32),
    step: f32,
    def_val: f32,
    value: UnsafeCell<f32>,
}

unsafe impl Send for PropertyF32 {}
unsafe impl Sync for PropertyF32 {}

impl Property for PropertyF32 {
    wrap_property_base!();

    #[inline]
    fn as_property_f32<'l>(&self) -> Option<&(dyn PropertyNumber<f32> + 'l)> {
        Some(self)
    }
}

impl PropertyNumber<f32> for PropertyF32 {
    #[inline]
    fn range(&self) -> (f32, f32) {
        self.range
    }

    #[inline]
    fn step(&self) -> f32 {
        self.step
    }

    #[inline]
    fn step_forward(&self) -> f32 {
        let clamped = (self.value() + self.step)
            .min(self.range.1)
            .max(self.range.0);
        unsafe {
            self.value.get().write(clamped);
        }
        clamped
    }

    #[inline]
    fn step_backward(&self) -> f32 {
        let clamped = (self.value() - self.step)
            .min(self.range.1)
            .max(self.range.0);
        unsafe {
            self.value.get().write(clamped);
        }
        clamped
    }

    #[inline]
    fn def_val(&self) -> f32 {
        self.def_val
    }

    #[inline]
    fn value(&self) -> f32 {
        unsafe { self.value.get().read() }
    }

    #[inline]
    fn value_mut(&self) -> &mut f32 {
        unsafe { &mut (*self.value.get()) }
    }

    /// # Safety
    #[inline]
    unsafe fn value_mut_ptr(&self) -> *mut f32 {
        self.value.get()
    }

    #[inline]
    fn set_value(&self, value: f32) -> f32 {
        let clamped = value.min(self.range.1).max(self.range.0);
        unsafe {
            self.value.get().write(clamped);
        }
        clamped
    }
}

impl PropertyF32 {
    pub fn with_slider(name: &'static str, range: (f32, f32), step: f32, def_val: f32) -> Self {
        Self {
            base: PropertyBase::with_slider_f32(name),
            range,
            step,
            def_val,
            value: UnsafeCell::new(def_val),
        }
    }
}

/// Float64 Property.
#[derive(Debug)]
pub struct PropertyF64 {
    base: PropertyBase,
    range: (f64, f64),
    step: f64,
    def_val: f64,
    value: UnsafeCell<f64>,
}

unsafe impl Send for PropertyF64 {}
unsafe impl Sync for PropertyF64 {}

impl Property for PropertyF64 {
    wrap_property_base!();

    #[inline]
    fn as_property_f64<'l>(&self) -> Option<&(dyn PropertyNumber<f64> + 'l)> {
        Some(self)
    }
}

impl PropertyNumber<f64> for PropertyF64 {
    #[inline]
    fn range(&self) -> (f64, f64) {
        self.range
    }

    #[inline]
    fn step(&self) -> f64 {
        self.step
    }

    #[inline]
    fn step_forward(&self) -> f64 {
        let clamped = (self.value() + self.step)
            .min(self.range.1)
            .max(self.range.0);
        unsafe {
            self.value.get().write(clamped);
        }
        clamped
    }

    #[inline]
    fn step_backward(&self) -> f64 {
        let clamped = (self.value() - self.step)
            .min(self.range.1)
            .max(self.range.0);
        unsafe {
            self.value.get().write(clamped);
        }
        clamped
    }

    #[inline]
    fn def_val(&self) -> f64 {
        self.def_val
    }

    #[inline]
    fn value(&self) -> f64 {
        unsafe { self.value.get().read() }
    }

    #[inline]
    fn value_mut(&self) -> &mut f64 {
        unsafe { &mut (*self.value.get()) }
    }

    /// # Safety
    #[inline]
    unsafe fn value_mut_ptr(&self) -> *mut f64 {
        self.value.get()
    }

    #[inline]
    fn set_value(&self, value: f64) -> f64 {
        let clamped = value.min(self.range.1).max(self.range.0);
        unsafe {
            self.value.get().write(clamped);
        }
        clamped
    }
}

impl PropertyF64 {
    pub fn with_slider(name: &'static str, range: (f64, f64), step: f64, def_val: f64) -> Self {
        Self {
            base: PropertyBase::with_slider_f64(name),
            range,
            step,
            def_val,
            value: UnsafeCell::new(def_val),
        }
    }
}

/// Integer32 Property.
#[derive(Debug)]
pub struct PropertyI32 {
    base: PropertyBase,
    range: (i32, i32),
    step: i32,
    def_val: i32,
    value: UnsafeCell<i32>,
}

unsafe impl Send for PropertyI32 {}
unsafe impl Sync for PropertyI32 {}

impl Property for PropertyI32 {
    wrap_property_base!();

    #[inline]
    fn as_property_i32<'l>(&self) -> Option<&(dyn PropertyNumber<i32> + 'l)> {
        Some(self)
    }
}

impl PropertyNumber<i32> for PropertyI32 {
    #[inline]
    fn range(&self) -> (i32, i32) {
        self.range
    }

    #[inline]
    fn step(&self) -> i32 {
        self.step
    }

    #[inline]
    fn step_forward(&self) -> i32 {
        let clamped = (self.value() + self.step)
            .min(self.range.1)
            .max(self.range.0);
        unsafe {
            self.value.get().write(clamped);
        }
        clamped
    }

    #[inline]
    fn step_backward(&self) -> i32 {
        let clamped = (self.value() - self.step)
            .min(self.range.1)
            .max(self.range.0);
        unsafe {
            self.value.get().write(clamped);
        }
        clamped
    }

    #[inline]
    fn def_val(&self) -> i32 {
        self.def_val
    }

    #[inline]
    fn value(&self) -> i32 {
        unsafe { self.value.get().read() }
    }

    #[inline]
    fn value_mut(&self) -> &mut i32 {
        unsafe { &mut (*self.value.get()) }
    }

    /// # Safety
    #[inline]
    unsafe fn value_mut_ptr(&self) -> *mut i32 {
        self.value.get()
    }

    #[inline]
    fn set_value(&self, value: i32) -> i32 {
        let clamped = value.min(self.range.1).max(self.range.0);
        unsafe {
            self.value.get().write(clamped);
        }
        clamped
    }
}

impl PropertyI32 {
    /// Create an new Integer32 Property with ComboBox rendering.
    /// # Panic
    /// The `options` must be not empty.
    pub fn with_combo_box(name: &'static str, options: &[&'static str], def_val: i32) -> Self {
        assert!(!options.is_empty());
        let range = (0, (options.len() - 1) as i32);
        Self {
            base: PropertyBase::with_combo_box_i32(name, options),
            range,
            step: 1,
            def_val,
            value: UnsafeCell::new(def_val),
        }
    }

    /// Create an new Integer32 Property with Select rendering.
    /// # Panic
    /// The `options` must be not empty.
    pub fn with_select(name: &'static str, options: &[&'static str], def_val: i32) -> Self {
        assert!(!options.is_empty());
        let range = (0, (options.len() - 1) as i32);
        Self {
            base: PropertyBase::with_select_i32(name, options),
            range,
            step: 1,
            def_val,
            value: UnsafeCell::new(def_val),
        }
    }

    /// Create an new Integer32 Property with Slider rendering.
    pub fn with_slider(name: &'static str, range: (i32, i32), step: i32, def_val: i32) -> Self {
        Self {
            base: PropertyBase::with_slider_i32(name),
            range,
            step,
            def_val,
            value: UnsafeCell::new(def_val),
        }
    }
}

/// Integer64 Property.
#[derive(Debug)]
pub struct PropertyI64 {
    base: PropertyBase,
    range: (i64, i64),
    step: i64,
    def_val: i64,
    value: UnsafeCell<i64>,
}

unsafe impl Send for PropertyI64 {}
unsafe impl Sync for PropertyI64 {}

impl Property for PropertyI64 {
    wrap_property_base!();

    #[inline]
    fn as_property_i64<'l>(&self) -> Option<&(dyn PropertyNumber<i64> + 'l)> {
        Some(self)
    }
}

impl PropertyNumber<i64> for PropertyI64 {
    #[inline]
    fn range(&self) -> (i64, i64) {
        self.range
    }

    #[inline]
    fn step(&self) -> i64 {
        self.step
    }

    #[inline]
    fn step_forward(&self) -> i64 {
        let clamped = (self.value() + self.step)
            .min(self.range.1)
            .max(self.range.0);
        unsafe {
            self.value.get().write(clamped);
        }
        clamped
    }

    #[inline]
    fn step_backward(&self) -> i64 {
        let clamped = (self.value() - self.step)
            .min(self.range.1)
            .max(self.range.0);
        unsafe {
            self.value.get().write(clamped);
        }
        clamped
    }

    #[inline]
    fn def_val(&self) -> i64 {
        self.def_val
    }

    #[inline]
    fn value(&self) -> i64 {
        unsafe { self.value.get().read() }
    }

    #[inline]
    fn value_mut(&self) -> &mut i64 {
        unsafe { &mut (*self.value.get()) }
    }

    /// # Safety
    #[inline]
    unsafe fn value_mut_ptr(&self) -> *mut i64 {
        self.value.get()
    }

    #[inline]
    fn set_value(&self, value: i64) -> i64 {
        let clamped = value.min(self.range.1).max(self.range.0);
        unsafe {
            self.value.get().write(clamped);
        }
        clamped
    }
}

impl PropertyI64 {
    /// Create an new Integer32 Property with ComboBox rendering.
    /// # Panic
    /// The `options` must be not empty.
    pub fn with_combo_box(name: &'static str, options: &[&'static str], def_val: i64) -> Self {
        assert!(!options.is_empty());
        let range = (0, (options.len() - 1) as i64);
        Self {
            base: PropertyBase::with_combo_box_i64(name, options),
            range,
            step: 1,
            def_val,
            value: UnsafeCell::new(def_val),
        }
    }

    /// Create an new Integer32 Property with Select rendering.
    /// # Panic
    /// The `options` must be not empty.
    pub fn with_select(name: &'static str, options: &[&'static str], def_val: i64) -> Self {
        assert!(!options.is_empty());
        let range = (0, (options.len() - 1) as i64);
        Self {
            base: PropertyBase::with_select_i32(name, options),
            range,
            step: 1,
            def_val,
            value: UnsafeCell::new(def_val),
        }
    }

    /// Create an new Integer32 Property with Slider rendering.
    pub fn with_slider(name: &'static str, range: (i64, i64), step: i64, def_val: i64) -> Self {
        Self {
            base: PropertyBase::with_slider_i64(name),
            range,
            step,
            def_val,
            value: UnsafeCell::new(def_val),
        }
    }
}

/// String Property.
pub struct PropertyString {
    base: PropertyBase,
    max_length: usize,
    def_val: String,
    value: RefCell<String>,
}

unsafe impl Send for PropertyString {}
unsafe impl Sync for PropertyString {}

impl Debug for PropertyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PropertyString")
            .field("name", &self.name())
            .field("widget_type", &self.widget_type())
            .field("max_length", &self.max_length())
            .field("def_val", &self.def_val())
            .field("value", &self.value())
            .finish()
    }
}

impl Default for PropertyString {
    fn default() -> Self {
        Self {
            base: PropertyBase::with_text_box("UnTitled"),
            max_length: 256,
            def_val: "".into(),
            value: RefCell::new(String::with_capacity(256)),
        }
    }
}

impl Property for PropertyString {
    wrap_property_base!();

    #[inline]
    fn as_property_string(&self) -> Option<&PropertyString> {
        Some(self)
    }
}

impl PropertyString {
    #[inline]
    pub fn with_text_box<S>(name: &'static str, max_length: usize, def_val: S) -> Self
    where
        S: Into<String>,
    {
        let def_val = def_val.into();
        let mut value = String::with_capacity(max_length);
        value.push_str(&def_val);
        Self {
            base: PropertyBase::with_text_box(name),
            max_length,
            def_val,
            value: RefCell::new(value),
        }
    }

    #[inline]
    pub fn max_length(&self) -> usize {
        self.max_length
    }

    #[inline]
    pub fn def_val(&self) -> &str {
        &self.def_val
    }

    #[inline]
    pub fn value(&self) -> Ref<'_, str> {
        Ref::<'_, String>::map(self.value.borrow(), String::as_str)
    }

    #[inline]
    pub fn value_mut(&self) -> RefMut<'_, String> {
        self.value.borrow_mut()
    }

    /// # Safety
    #[inline]
    pub unsafe fn value_ptr(&self) -> *const u8 {
        self.value.borrow().as_ptr()
    }

    /// # Safety
    #[inline]
    pub unsafe fn value_mut_ptr(&self) -> *mut u8 {
        self.value.borrow_mut().as_mut_ptr()
    }

    #[inline]
    pub fn set_value(&self, value: &str) -> Ref<'_, str> {
        {
            let mut s = self.value.borrow_mut();
            s.clear();
            s.push_str(value);
        }
        Ref::<'_, String>::map(self.value.borrow(), String::as_str)
    }
}

type PropertyItem = Arc<dyn Property + Send + Sync>;

/// Property Sheet.
#[derive(Default)]
pub struct PropertySheet {
    items: Vec<PropertyItem>,
}

impl Debug for PropertySheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.items.iter()).finish()
    }
}

impl PropertySheet {
    /// Create a new property sheet.
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    /// Create a new property sheet with items.
    pub fn with_items(items: Vec<PropertyItem>) -> Self {
        for (i, p) in items.iter().enumerate() {
            p.set_id(i);
        }
        Self { items }
    }

    /// Append a property to the sheet.
    pub fn append<T>(&mut self, item: T)
    where
        T: Property + Sync + Send + 'static,
    {
        item.set_id(self.items.len());
        self.items.push(Arc::new(item));
    }

    /// Inserts a property at position index within the sheet, shifting all properties after it to the right.
    pub fn insert<T>(&mut self, index: usize, item: T)
    where
        T: Property + Sync + Send + 'static,
    {
        item.set_id(index);
        for p in &self.items[index..] {
            p.set_id(p.id() + 1);
        }
        self.items.insert(index, Arc::new(item));
    }

    /// Removes and returns the property at position index within the sheet, shifting all properties after it to the left.
    pub fn remove(&mut self, index: usize) -> PropertyItem {
        for p in &self.items[index..] {
            p.set_id(p.id() - 1);
        }
        self.items.remove(index)
    }

    /// Returns true if the vector contains no properties.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the number of properties in the sheet, also referred to as its 'length'.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns the property reference at index in the sheet.
    pub fn get(&self, index: usize) -> Option<&PropertyItem> {
        self.items.get(index)
    }

    /// Returns the mutable property reference at index in the sheet.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut PropertyItem> {
        self.items.get_mut(index)
    }

    /// Returns an iterator over the slice.
    pub fn iter(&self) -> std::slice::Iter<'_, PropertyItem> {
        self.items.iter()
    }

    /// Returns an iterator that allows modifying each value.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, PropertyItem> {
        self.items.iter_mut()
    }

    /// Returns an item reference that match to the `name`.
    pub fn find(&self, name: &'static str) -> Option<&PropertyItem> {
        for p in self.items.iter() {
            if p.name() == name {
                return Some(p);
            }
        }
        None
    }

    /// Returns an mutable item reference that match to the `name`.
    pub fn find_mut(&mut self, name: &'static str) -> Option<&mut PropertyItem> {
        for p in self.items.iter_mut() {
            if p.name() == name {
                return Some(p);
            }
        }
        None
    }

    /// Mark all item listed in `ids` as `selected`.
    pub fn select_items(&mut self, ids: &[usize]) {
        for p in self.items.iter_mut() {
            p.set_selected(ids.contains(&p.id()));
        }
    }

    /// Mark `prev` item as `selected`.
    pub fn select_prev(&mut self) {
        for i in self.selected_items() {
            if i > 0 && i < self.len() {
                self.select_items(&[i - 1]);
            }
            break;
        }
    }

    /// Mark `next` item as `selected`, wrap to `last` item when current at `first` item.
    pub fn select_prev_wrapped(&mut self) {
        for i in self.selected_items() {
            if i > 0 && i < (self.len() - 1) {
                self.select_items(&[i + 1]);
            } else {
                self.select_items(&[self.len() - 1]);
            }
            break;
        }
    }

    /// Mark next item as `selected`.
    pub fn select_next(&mut self) {
        for i in self.selected_items() {
            if i < (self.len() - 1) {
                self.select_items(&[i + 1]);
            }
            break;
        }
    }

    /// Mark next item as `selected`, wrap to `first` item when current at `last` item.
    pub fn select_next_wrapped(&mut self) {
        for i in self.selected_items() {
            if i < (self.len() - 1) {
                self.select_items(&[i + 1]);
            } else {
                self.select_items(&[0]);
            }
            break;
        }
    }

    /// Returns all `selected` items.
    pub fn selected_items(&self) -> Vec<usize> {
        let mut sels: Vec<usize> = vec![];
        for p in self.items.iter().filter(|x| x.is_selected()) {
            sels.push(p.id())
        }
        sels
    }

    /// Add a Action Button to the sheet.
    pub fn action_button<F>(&mut self, name: &'static str, text: &'static str, f: Arc<RefCell<F>>)
    where
        F: FnMut(&dyn Property, bool) -> bool + 'static,
    {
        let p = PropertyAction::with_button(name, text, f);
        p.set_id(self.items.len());
        self.items.push(Arc::new(p));
    }

    /// Add a Action CheckBox to the sheet.
    pub fn action_check_box<F>(&mut self, name: &'static str, checked: bool, f: Arc<RefCell<F>>)
    where
        F: FnMut(&dyn Property, bool) -> bool + 'static,
    {
        let p = PropertyAction::with_check_box(name, checked, f);
        p.set_id(self.items.len());
        self.items.push(Arc::new(p));
    }

    /// Add a Float32 Slider to the sheet.
    pub fn slider_f32(&mut self, name: &'static str, range: (f32, f32), step: f32, def_val: f32) {
        let p = PropertyF32::with_slider(name, range, step, def_val);
        p.set_id(self.items.len());
        self.items.push(Arc::new(p));
    }

    /// Add a Float64 Slider to the sheet.
    pub fn slider_f64(&mut self, name: &'static str, range: (f64, f64), step: f64, def_val: f64) {
        let p = PropertyF64::with_slider(name, range, step, def_val);
        p.set_id(self.items.len());
        self.items.push(Arc::new(p));
    }

    /// Add an Integer32 ComboBox to the sheet.
    pub fn combo_box_i32(&mut self, name: &'static str, options: &[&'static str], def_val: i32) {
        let p = PropertyI32::with_combo_box(name, options, def_val);
        p.set_id(self.items.len());
        self.items.push(Arc::new(p));
    }

    /// Add an Integer32 Select to the sheet.
    pub fn select_i32(&mut self, name: &'static str, options: &[&'static str], def_val: i32) {
        let p = PropertyI32::with_select(name, options, def_val);
        p.set_id(self.items.len());
        self.items.push(Arc::new(p));
    }

    /// Add an Integer32 Slider to the sheet.
    pub fn slider_i32(&mut self, name: &'static str, range: (i32, i32), step: i32, def_val: i32) {
        let p = PropertyI32::with_slider(name, range, step, def_val);
        p.set_id(self.items.len());
        self.items.push(Arc::new(p));
    }

    /// Add a Boolean Switch to the sheet.
    pub fn switch(&mut self, name: &'static str, def_val: bool) {
        let p = PropertyBool::with_switch(name, def_val);
        p.set_id(self.items.len());
        self.items.push(Arc::new(p))
    }

    /// Add a String Text Box to the sheet.
    pub fn text_box(&mut self, name: &'static str, max_length: usize, def_val: &'static str) {
        let p = PropertyString::with_text_box(name, max_length, def_val);
        p.set_id(self.items.len());
        self.items.push(Arc::new(p))
    }
}

/// The Type of the Property Value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ValueType {
    Unknown,
    Action,
    Bool,
    F32,
    F64,
    I32,
    I64,
    String,
}

impl Default for ValueType {
    fn default() -> Self {
        ValueType::Unknown
    }
}

/// The Type of the Widget to rendering the Property Value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WidgetType {
    Unknown,
    Button,
    CheckBox,
    ComboBox,
    Select,
    Slider,
    Switch,
    TextBox,
}

impl Default for WidgetType {
    fn default() -> Self {
        WidgetType::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;
    use std::sync::{Mutex, RwLock};

    #[test]
    fn test_property_sheet() {
        let ps = Arc::new(RwLock::new(PropertySheet::new()));
        let cloned = Arc::clone(&ps);
        let triggerd = Arc::new(RefCell::new(
            move |prop: &dyn Property, checked: bool| -> bool {
                assert!(cloned.read().unwrap().len() > 0);
                assert!(prop.name().len() > 0);
                checked
            },
        ));
        if let Ok(ref mut ps) = ps.write() {
            ps.action_button("Foo", "Click Me", Arc::clone(&triggerd));
            ps.action_button("Bar", "Click Me", Arc::clone(&triggerd));
            ps.combo_box_i32("ComboBox", &["A", "B", "C"], 0);
            ps.slider_f32("Float1", (-1.0, 1.0), 0.01, 0.0);
            ps.slider_f32("Float10", (-10.0, 10.0), 0.1, 0.0);
            ps.slider_f32("Float100", (-100.0, 100.0), 1.0, 0.0);
            ps.slider_f64("Float1000", (-1000.0, 1000.0), 10.0, 0.0);
            ps.switch("Switch", false);
            ps.text_box("TextBox", 128, "Okay");
        }
        for p in ps.read().unwrap().iter() {
            assert!(p.name().len() > 0);
            assert!(p.value_type() != ValueType::Unknown);
            assert!(p.widget_type() != WidgetType::Unknown);
            assert!(p.is_visible() == true);
        }
        for p in ps
            .read()
            .unwrap()
            .iter()
            .filter(|x| x.value_type() == ValueType::Bool)
        {
            assert!(p.name().len() > 0);
            assert!(p.value_type() == ValueType::Bool);
        }
        for p in ps
            .read()
            .unwrap()
            .iter()
            .filter(|x| x.value_type() == ValueType::F32)
        {
            assert!(p.name().len() > 0);
            assert!(p.value_type() == ValueType::F32);
        }
        let cloned = Arc::clone(&ps);
        let th = std::thread::spawn(move || {
            if let Ok(ref mut ps) = cloned.read() {
                if let Some(p) = ps.get(0) {
                    let fp = p.as_property_action();
                    fp.unwrap().trigger(true);
                }
                if let Some(p) = ps.get(1) {
                    let fp = p.as_property_action();
                    fp.unwrap().trigger(true);
                }
                if let Some(p) = ps.get(3) {
                    let fp = p.as_property_f32();
                    fp.unwrap().set_value(0.123456);
                    assert_eq!(fp.unwrap().value(), 0.123456);
                }
                if let Some(p) = ps.get(7) {
                    let fp = p.as_property_bool();
                    fp.unwrap().set_value(true);
                    assert_eq!(fp.unwrap().value(), true);
                }
                if let Some(p) = ps.get(8) {
                    let fp = p.as_property_string();
                    fp.unwrap().set_value("Failure");
                    fp.unwrap().value_mut().push('!');
                    assert_eq!(fp.unwrap().value().deref(), "Failure!");
                }
            }
        });

        assert_eq!(th.join().is_ok(), true);

        if let Ok(ref ps) = ps.read() {
            assert_eq!(ps.find("Foo").is_some(), true);
            assert_eq!(ps.find("Bar").is_some(), true);
            assert_eq!(ps.find("Switch").is_some(), true);
            assert_eq!(ps.find("TextBox").is_some(), true);
            assert_eq!(ps.find("UnExists").is_none(), true);
        }

        assert_eq!(ps.write().unwrap().find_mut("Foo").is_some(), true);
        assert_eq!(ps.write().unwrap().find_mut("Bar").is_some(), true);
        assert_eq!(ps.write().unwrap().find_mut("Switch").is_some(), true);
        assert_eq!(ps.write().unwrap().find_mut("TextBox").is_some(), true);
        assert_eq!(ps.write().unwrap().find_mut("UnExists").is_none(), true);

        assert_eq!(
            ps.read().unwrap().find("Foo").unwrap().is_action_checked(),
            Some(true)
        );
        assert_eq!(
            ps.read()
                .unwrap()
                .find("Foo")
                .unwrap()
                .trigger_action(false),
            Some(false)
        );
        assert_eq!(
            ps.read().unwrap().find("Float1").unwrap().get_value_f32(),
            Some(0.123456)
        );
        assert_eq!(
            ps.read().unwrap().find("ComboBox").unwrap().get_value_i32(),
            Some(0)
        );
        assert_eq!(
            ps.read()
                .unwrap()
                .find("TextBox")
                .unwrap()
                .get_value_string()
                .unwrap()
                .deref(),
            "Failure!"
        );
    }
}
