// Copyright 2018 The Druid Authors.
// Copyright 2020 The Nuki Authors.
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

//! A widget that arranges its children in a one-dimensional array.

// use nuki_derive::{Data, Lens};
use super::prelude::*;

/// An axis in visual space.
///
/// Most often used by widgets to describe
/// the direction in which they grow as their number of children increases.
/// Has some methods for manipulating geometry with respect to the axis.
#[derive(Data, Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    /// The x axis
    Horizontal,
    /// The y axis
    Vertical,
}

/// Widget Wrapper.
#[derive(Debug)]
struct ChildWidget<T> {
    widget: WidgetPod<T, Box<dyn Widget<T>>>,
    params: FlexParams,
}

impl<T> ChildWidget<T> {
    fn new(child: impl Widget<T> + 'static, params: FlexParams) -> Self {
        ChildWidget {
            widget: WidgetPod::new(Box::new(child)),
            params,
        }
    }
}

impl<T: Data> Widget<T> for ChildWidget<T> {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.widget.lifecycle(ctx, event, data, env);
    }

    fn present(&mut self, ctx: &mut PresentCtx, data: &T, env: &Env) {
        self.widget.present(ctx, data, env);
    }
}

/// A container with either horizontal or vertical layout.
///
/// This widget is the foundation of most layouts, and is highly configurable.
///
pub struct Flex<T> {
    direction: Axis,
    children: Vec<ChildWidget<T>>,
}

impl<T: Data> Flex<T> {
    /// Create a new Flex oriented along the provided axis.
    pub fn for_axis(axis: Axis) -> Self {
        Flex {
            direction: axis,
            children: Vec::new(),
        }
    }

    /// Create a new horizontal stack.
    ///
    /// The child widgets are laid out horizontally, from left to right.
    ///
    pub fn row() -> Self {
        Self::for_axis(Axis::Horizontal)
    }

    /// Create a new vertical stack.
    ///
    /// The child widgets are laid out vertically, from top to bottom.
    pub fn column() -> Self {
        Self::for_axis(Axis::Vertical)
    }

    /// Builder-style variant of `add_child`.
    ///
    /// Convenient for assembling a group of widgets in a single expression.
    pub fn with_child(mut self, child: impl Widget<T> + 'static) -> Self {
        self.add_flex_child(child, 0.0);
        self
    }

    /// Builder-style method to add a flexible child to the container.
    ///
    /// This method is used when you need more control over the behaviour
    /// of the widget you are adding. In the general case, this likely
    /// means giving that child a 'flex factor', but it could also mean
    /// giving the child a custom [`CrossAxisAlignment`], or a combination
    /// of the two.
    ///
    /// This function takes a child widget and [`FlexParams`]; importantly
    /// you can pass in a float as your [`FlexParams`] in most cases.
    ///
    /// For the non-builder varient, see [`add_flex_child`].
    ///
    /// # Examples
    ///
    /// ```
    /// use nuki::widget::{Flex, FlexParams, Label, Slider};
    ///
    /// let my_row = Flex::<f32>::row()
    ///     .with_flex_child(Slider::new(), 1.0)
    ///     .with_flex_child(Slider::new(), FlexParams::new(1.0));
    /// ```
    ///
    /// [`FlexParams`]: struct.FlexParams.html
    /// [`add_flex_child`]: #method.add_flex_child
    pub fn with_flex_child(
        mut self,
        child: impl Widget<T> + 'static,
        params: impl Into<FlexParams>,
    ) -> Self {
        self.add_flex_child(child, params);
        self
    }

    /// Add a non-flex child widget.
    ///
    /// See also [`with_child`].
    ///
    /// [`with_child`]: #method.with_child
    pub fn add_child(&mut self, child: impl Widget<T> + 'static) {
        self.add_flex_child(child, 0.0);
    }

    /// Add a flexible child widget.
    ///
    /// This method is used when you need more control over the behaviour
    /// of the widget you are adding. In the general case, this likely
    /// means giving that child a 'flex factor', but it could also mean
    /// giving the child a custom [`CrossAxisAlignment`], or a combination
    /// of the two.
    ///
    /// This function takes a child widget and [`FlexParams`]; importantly
    /// you can pass in a float as your [`FlexParams`] in most cases.
    ///
    /// For the builder-style varient, see [`with_flex_child`].
    ///
    /// # Examples
    ///
    /// ```
    /// use nuki::widget::{Flex, FlexParams, Label, Slider};
    ///
    /// let mut my_row = Flex::<f32>::row();
    /// my_row.add_flex_child(Slider::new(), 1.0);
    /// my_row.add_flex_child(Slider::new(), FlexParams::new(1.0));
    /// ```
    ///
    /// [`FlexParams`]: struct.FlexParams.html
    /// [`with_flex_child`]: #method.with_flex_child
    pub fn add_flex_child(
        &mut self,
        child: impl Widget<T> + 'static,
        params: impl Into<FlexParams>,
    ) {
        let child = ChildWidget::new(child, params.into());
        self.children.push(child);
    }
}

impl<T: Data> Widget<T> for Flex<T> {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        for c in self.children.iter_mut() {
            c.lifecycle(ctx, event, data, env);
        }
    }

    fn present(&mut self, ctx: &mut PresentCtx, data: &T, env: &Env) {
        for c in self.children.iter_mut() {
            c.present(ctx, data, env);
        }
    }
}

/// Optional parameters for an item in a [`Flex`] container (row or column).
///
/// Generally, when you would like to add a flexible child to a container,
/// you can simply call [`with_flex_child`] or [`add_flex_child`], passing the
/// child and the desired flex factor as a `f64`, which has an impl of
/// `Into<FlexParams>`.
///
/// If you need to set additional paramaters, such as a custom [`CrossAxisAlignment`],
/// you can construct `FlexParams` directly. By default, the child has the
/// same `CrossAxisAlignment` as the container.
///
/// For an overview of the flex layout algorithm, see the [`Flex`] docs.
///
/// # Examples
/// ```
/// use nuki::widget::{FlexParams, Label};
///
/// let mut row = nuki::widget::Flex::<()>::row();
/// let child_1 = Label::new("I'm hungry");
/// let child_2 = Label::new("I'm scared");
/// // normally you just use a float:
/// row.add_flex_child(child_1, 1.0);
/// // you can construct FlexParams if needed:
/// let params = FlexParams::new(2.0);
/// row.add_flex_child(child_2, params);
/// ```
///
/// [`CrossAxisAlignment`]: enum.CrossAxisAlignment.html
/// [`Flex`]: struct.Flex.html
/// [`with_flex_child`]: struct.Flex.html#method.with_flex_child
/// [`add_flex_child`]: struct.Flex.html#method.add_flex_child
#[derive(Copy, Clone, Debug, Default)]
pub struct FlexParams {
    flex: f64,
}

impl FlexParams {
    /// Create custom `FlexParams` with a specific `flex_factor`.
    ///
    /// You likely only need to create these manually if you need to specify
    /// a custom alignment; if you only need to use a custom `flex_factor` you
    /// can pass an `f64` to any of the functions that take `FlexParams`.
    ///
    /// By default, the widget uses the alignment of its parent [`Flex`] container.
    ///
    ///
    /// [`Flex`]: struct.Flex.html
    pub fn new(flex: f64) -> Self {
        Self { flex }
    }
}

impl From<f64> for FlexParams {
    fn from(val: f64) -> Self {
        Self::new(val)
    }
}
