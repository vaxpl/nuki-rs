// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A label widget.

#![allow(unused_variables)]

use super::prelude::*;
use crate::{Color, KeyOrValue};

use std::borrow::Cow;
use std::fmt::{Debug, Display};

/// A label that displays static or dynamic text.
#[derive(Debug)]
pub struct Label<T: Debug> {
    text: LabelText<T>,
    text_color: KeyOrValue<Color>,
}

impl<T: Debug> Label<T> {
    /// Construct a new `Label` widget.
    ///
    /// ```
    /// use nuki::widget::Label;
    ///
    /// // Construct a new Label using static string.
    /// let _: Label<u32> = Label::new("Hello world");
    ///
    /// // Construct a new dynamic Label. Text will be updated when data changes.
    /// let _: Label<u32> = Label::new(|data: &u32, _env: &_| format!("Hello world: {}", data));
    /// ```
    pub fn new(text: impl Into<LabelText<T>>) -> Self {
        Self {
            text: text.into(),
            text_color: Color::rgb_f(0.0, 0.0, 0.0).into(),
        }
    }

    /// Construct a new dynamic label.
    ///
    /// The contents of this label are generated from the data using a closure.
    ///
    /// This is provided as a convenience; a closure can also be passed to [`new`],
    /// but due to limitations of the implementation of that method, the types in
    /// the closure need to be annotated, which is not true for this method.
    ///
    /// # Examples
    ///
    /// The following are equivalent.
    ///
    /// ```
    /// use nuki::Env;
    /// use nuki::widget::Label;
    /// let label1: Label<u32> = Label::new(|data: &u32, _: &Env| format!("total is {}", data));
    /// let label2: Label<u32> = Label::dynamic(|data, _| format!("total is {}", data));
    /// ```
    ///
    /// [`new`]: #method.new
    pub fn dynamic(text: impl Fn(&T, &Env) -> String + 'static) -> Self {
        let text: LabelText<T> = text.into();
        Label::new(text)
    }

    /// Construct a new fixed content label.
    ///
    /// # Examples
    ///
    /// The following are equivalent.
    ///
    /// ```
    /// use nuki::Env;
    /// use nuki::widget::Label;
    /// use std::borrow::Cow;
    /// let label1: Label<u32> = Label::new("The title of the <Label>");
    /// let label2: Label<u32> = Label::fixed("The title of the <Label>");
    /// let label3: Label<u32> = Label::fixed(format!("The title of the <{}>", "Label"));
    /// ```
    ///
    /// [`new`]: #method.new
    pub fn fixed<S>(text: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self {
            text: text.into().into(),
            text_color: Color::rgb_f(0.0, 0.0, 0.0).into(),
        }
    }

    /// Builder-style method for setting the text color.
    ///
    /// The argument can be either a `Color` or a [`Key<Color>`].
    ///
    /// [`Key<Color>`]: ../struct.Key.html
    pub fn with_text_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.set_text_color(color);
        self
    }

    /// Set the text color.
    ///
    /// The argument can be either a `Color` or a [`Key<Color>`].
    ///
    /// If you change this property, you are responsible for calling
    /// [`request_layout`] to ensure the label is updated.
    ///
    /// [`request_layout`]: ../struct.EventCtx.html#method.request_layout
    /// [`Key<Color>`]: ../struct.Key.html
    pub fn set_text_color(&mut self, color: impl Into<KeyOrValue<Color>>) {
        self.text_color = color.into();
    }
}

impl<T: Data + Debug> Widget<T> for Label<T> {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => ctx.set_has_focus(false),
            _ => {}
        }
    }

    fn present(&mut self, ctx: &mut PresentCtx, data: &T, env: &Env) {
        let text = self.text.resolve(data, env);
        println!("text={:?}", text);
    }
}

/// The text for a [`Label`].
#[derive(Debug)]
pub enum LabelText<T: Debug> {
    /// Fixed text.
    Fixed(Fixed),
    /// The provided closure is called on update, and its return
    /// value is used as the text for the label.
    Dynamic(Dynamic<T>),
}

impl<T: Debug> LabelText<T> {
    pub fn resolve(&mut self, data: &T, env: &Env) -> Cow<'static, str> {
        match self {
            LabelText::Fixed(s) => s.v.clone(),
            LabelText::Dynamic(s) => Cow::Owned((s.f)(data, env)),
        }
    }
}

impl<T: Debug> From<&'static str> for LabelText<T> {
    fn from(text: &'static str) -> Self {
        Self::Fixed(Fixed {
            v: Cow::Borrowed(text),
        })
    }
}

impl<T: Debug> From<Cow<'static, str>> for LabelText<T> {
    fn from(text: Cow<'static, str>) -> Self {
        Self::Fixed(Fixed { v: text })
    }
}

impl<T: Debug, F: Fn(&T, &Env) -> String + 'static> From<F> for LabelText<T> {
    fn from(f: F) -> Self {
        Self::Dynamic(Dynamic { f: Box::new(f) })
    }
}

/// Text that is computed dynamically.
pub struct Dynamic<T: Debug> {
    f: Box<dyn Fn(&T, &Env) -> String>,
}

impl<T: Debug> Debug for Dynamic<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dynamic {{ f: {:p} }}", self.f)
    }
}

/// Fixed text.
#[derive(Debug)]
pub struct Fixed {
    /// The text.
    v: Cow<'static, str>,
}
