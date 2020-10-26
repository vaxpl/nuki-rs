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

use super::prelude::*;

use std::fmt::Debug;
use std::num::NonZeroU64;
use std::ops::{Deref, DerefMut};

/// The trait implemented by all widgets.
///
/// All appearance and behavior for a widget is encapsulated in an
/// object that implements this trait.
///
pub trait Widget<T> {
    /// Handle a life cycle notification.
    ///
    /// This method is called to notify your widget of certain special events,
    /// (available in the [`LifeCycle`] enum) that are generally related to
    /// changes in the widget graph or in the state of your specific widget.
    ///
    /// A widget is not expected to mutate the application state in response
    /// to these events, but only to update its own internal state as required;
    /// if a widget needs to mutate data, it can submit a [`Command`] that will
    /// be executed at the next opportunity.
    ///    
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env);

    /// Paint the widget appearance.
    ///
    /// The [`PaintCtx`] derefs to something that implements the [`RenderContext`]
    /// trait, which exposes various methods that the widget can use to paint
    /// its appearance.
    ///
    /// Container widgets can paint a background before recursing to their
    /// children, or annotations (for example, scrollbars) by painting
    /// afterwards. In addition, they can apply masks and transforms on
    /// the render context, which is especially useful for scrolling.
    ///
    fn present(&mut self, ctx: &mut PresentCtx, data: &T, env: &Env);

    /// Get the identity of the widget; this is basically only implemented by
    /// `IdentityWrapper`. Widgets should not implement this on their own.
    fn id(&self) -> Option<WidgetId> {
        None
    }

    /// Get the (verbose) type name of the widget for debugging purposes.
    /// You should not override this method.
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl<T> Debug for dyn Widget<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Widget {{ id: {:?}, type_name: {:?} }}",
            self.id(),
            self.type_name()
        )
    }
}

impl<T> Widget<T> for Box<dyn Widget<T>> {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.deref_mut().lifecycle(ctx, event, data, env);
    }

    fn present(&mut self, ctx: &mut PresentCtx, data: &T, env: &Env) {
        self.deref_mut().present(ctx, data, env);
    }

    fn id(&self) -> Option<WidgetId> {
        self.deref().id()
    }

    fn type_name(&self) -> &'static str {
        self.deref().type_name()
    }
}

/// A unique identifier for a single [`Widget`].
///
/// `WidgetId`s are generated automatically for all widgets that participate
/// in layout. More specifically, each [`WidgetPod`] has a unique `WidgetId`.
///
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct WidgetId(NonZeroU64);

impl WidgetId {
    /// Allocate a new, unique `WidgetId`.
    ///
    /// All widgets are assigned ids automatically; you should only create
    /// an explicit id if you need to know it ahead of time, for instance
    /// if you want two sibling widgets to know each others' ids.
    ///
    /// You must ensure that a given `WidgetId` is only ever used for one
    /// widget at a time.
    pub fn next() -> WidgetId {
        use crate::Counter;
        static WIDGET_ID_COUNTER: Counter = Counter::new();
        WidgetId(WIDGET_ID_COUNTER.next_nonzero())
    }

    /// Create a reserved `WidgetId`, suitable for reuse.
    ///
    /// The caller is responsible for ensuring that this ID is in fact assigned
    /// to a single widget at any time, or your code may become haunted.
    ///
    /// The actual inner representation of the returned `WidgetId` will not
    /// be the same as the raw value that is passed in; it will be
    /// `u64::max_value() - raw`.
    #[allow(unsafe_code)]
    pub const fn reserved(raw: u16) -> WidgetId {
        let id = u64::max_value() - raw as u64;
        // safety: by construction this can never be zero.
        WidgetId(unsafe { std::num::NonZeroU64::new_unchecked(id) })
    }

    pub(crate) fn to_raw(self) -> u64 {
        self.0.into()
    }
}

/// A container for one widget in the hierarchy.
///
/// Generally, container widgets don't contain other widgets directly,
/// but rather contain a `WidgetPod`, which has additional state needed
/// for layout and for the widget to participate in event flow.
///
#[derive(Debug)]
pub struct WidgetPod<T, W> {
    state: WidgetState,
    old_data: Option<T>,
    inner: W,
}

impl<T, W: Widget<T>> WidgetPod<T, W> {
    /// Create a new widget pod.
    ///
    /// In a widget hierarchy, each widget is wrapped in a `WidgetPod`
    /// so it can participate in layout and event flow. The process of
    /// adding a child widget to a container should call this method.
    pub fn new(inner: W) -> WidgetPod<T, W> {
        WidgetPod {
            state: WidgetState::new(),
            old_data: None,
            inner,
        }
    }

    /// Read-only access to state. We don't mark the field as `pub` because
    /// we want to control mutation.
    pub fn state(&self) -> &WidgetState {
        &self.state
    }

    /// Returns `true` if any descendant is active.
    pub fn has_active(&self) -> bool {
        self.state.has_active
    }

    /// Returns `true` if any descendant is hover.
    pub fn has_hover(&self) -> bool {
        self.state.has_hover
    }

    /// Returns `true` if any descendant is focus.
    pub fn has_focus(&self) -> bool {
        self.state.has_focus
    }

    /// Query the "active" state of the widget.
    pub fn is_actived(&self) -> bool {
        self.state.is_actived
    }

    /// Query the "hover" state of the widget.
    pub fn is_hovered(&self) -> bool {
        self.state.is_hovered
    }

    /// Query the "focus" state of the widget.
    pub fn is_focused(&self) -> bool {
        self.state.is_focused
    }

    /// Return a reference to the inner widget.
    pub fn widget(&self) -> &W {
        &self.inner
    }

    /// Return a mutable reference to the inner widget.
    pub fn widget_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    /// Get the identity of the widget.
    pub fn id(&self) -> WidgetId {
        self.state.id
    }
}

impl<T, W: Debug + Widget<T>> Widget<T> for WidgetPod<T, W> {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        let mut recurse = true;
        let recurse = match event {
            LifeCycle::WidgetAdded => true,
            _ => true,
        };

        if recurse {
            let mut child_ctx = LifeCycleCtx {
                state: ctx.state,
                widget_state: &mut self.state,
            };
            self.inner.lifecycle(&mut child_ctx, event, data, env);
        }

        match event {
            LifeCycle::WidgetAdded => {
                if self.state.has_focus {
                    ctx.add_focus_widget(self.id());
                }
            }
            _ => {}
        }
    }

    fn present(&mut self, ctx: &mut PresentCtx, data: &T, env: &Env) {
        self.inner.present(ctx, data, env);
    }
}

/// Generic state for all widgets in the hierarchy.
///
/// This struct contains the widget's layout rect, flags
/// indicating when the widget is active or focused, and other
/// state necessary for the widget to participate in event
/// flow.
///
#[derive(Copy, Clone, Debug)]
pub struct WidgetState {
    /// Identifier of the widget.
    pub id: WidgetId,
    /// True if the widget is activated.
    pub is_actived: bool,
    /// True if the widget is hovered.
    pub is_hovered: bool,
    /// True if the widget is focused.
    pub is_focused: bool,
    /// The widget could be activate or deactivate.
    pub has_active: bool,
    /// The widget could accept user inputs.
    pub has_focus: bool,
    /// The widget provide state on mouse is hovered over it.
    pub has_hover: bool,
}

impl WidgetState {
    /// Return a new state for widget.
    pub fn new() -> Self {
        Self {
            id: WidgetId::next(),
            is_actived: false,
            is_hovered: false,
            is_focused: false,
            has_active: true,
            has_focus: true,
            has_hover: false,
        }
    }

    // /// Returns a state for dummy widget.
    // pub fn dummy() -> Self {
    //     Self {
    //         id: WidgetId::dummy(),
    //         is_actived: false,
    //         is_hovered: false,
    //         is_focused: false,
    //         has_active: false,
    //         has_focus: false,
    //         has_hover: false,
    //     }
    // }
}

impl Default for WidgetState {
    fn default() -> Self {
        Self::new()
    }
}
