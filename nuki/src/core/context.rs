// Copyright 2020 The Druid Authors.
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

//! The context types that are passed into various widget methods.

use super::FocusChain;
use crate::{Presenter, WidgetId, WidgetState};

/// A macro for implementing methods on multiple contexts.
///
/// There are a lot of methods defined on multiple contexts; this lets us only
/// have to write them out once.
macro_rules! impl_context_method {
    ($ty:ty,  { $($method:item)+ } ) => {
        impl $ty { $($method)+ }
    };
    ( $ty:ty, $($more:ty),+, { $($method:item)+ } ) => {
        impl_context_method!($ty, { $($method)+ });
        impl_context_method!($($more),+, { $($method)+ });
    };
}

/// Static state that is shared between most contexts.
pub struct ContextState<'a> {
    /// A list to the focusable widgets.
    pub focus_chain: &'a mut FocusChain,
}

impl<'a> ContextState<'a> {
    // pub fn focus_chain(&self) -> Ref<'a, FocusChain> {
    //     Ref::new(self.focus_chain)
    // }

    // pub fn focus_chain_mut(&self) -> &'a mut FocusChain {
    //     self.focus_chain
    // }
}

/// A context passed to lifecycle methods of widgets.
pub struct LifeCycleCtx<'a, 'b> {
    /// A mutable reference to the state of shared between most contexts.
    pub state: &'a mut ContextState<'b>,
    /// A mutable reference to the state of the current widget.
    pub widget_state: &'a mut WidgetState,
}

impl<'a, 'b> LifeCycleCtx<'a, 'b> {
    /// Construct a new Context for `lifecycle`.
    pub fn new(state: &'a mut ContextState<'b>, widget_state: &'a mut WidgetState) -> Self {
        Self {
            state,
            widget_state,
        }
    }

    /// Change the `has_active` flag of the `widget_state`.
    pub fn set_has_active(&mut self, has: bool) {
        self.widget_state.has_active = has;
    }

    /// Change the `has_focus` flag of the `widget_state`.
    pub fn set_has_focus(&mut self, has: bool) {
        self.widget_state.has_focus = has;
    }

    /// Change the `has_hover` flag of the `widget_state`.
    pub fn set_has_hover(&mut self, has: bool) {
        self.widget_state.has_hover = has;
    }
}

/// A context passed to present methods of widgets.
pub struct PresentCtx<'a, 'b> {
    /// A mutable reference to the state of shared between most contexts.
    pub state: &'a mut ContextState<'b>,
    /// A reference to the state of the current widget.
    pub widget_state: &'a WidgetState,
    /// Presenter for Drawing.
    pub presenter: &'a mut dyn Presenter,
}

impl<'a, 'b> PresentCtx<'a, 'b> {
    pub fn new(
        state: &'a mut ContextState<'b>,
        widget_state: &'a WidgetState,
        presenter: &'a mut dyn Presenter,
    ) -> Self {
        Self {
            state,
            widget_state,
            presenter,
        }
    }
}

// methods on everyone
impl_context_method!(LifeCycleCtx<'_, '_>, PresentCtx<'_, '_>, {
    /// Add widget to the focus chain.
    pub fn add_focus_widget(&mut self, widget: WidgetId) {
        self.state.focus_chain.add_widget(widget);
    }

    /// Remove widget from the focus chain.
    pub fn remove_focus_widget(&mut self, widget: WidgetId) {
        self.state.focus_chain.remove_widget(widget);
    }

    /// Return the `WidgetId` of the current widget.
    pub fn widget_id(&self) -> WidgetId {
        self.widget_state.id
    }
});

// methods on everyone but layoutctx
impl_context_method!(LifeCycleCtx<'_, '_>, PresentCtx<'_, '_>, {
    /// Return true if the current widget can be activate or deactivate.
    pub fn has_active(&self) -> bool {
        self.widget_state.has_focus
    }

    /// Return true if the current widget can accept user inputs.
    pub fn has_focus(&self) -> bool {
        self.widget_state.has_focus
    }

    /// Return true if the current widget can track mouse hover it.
    pub fn has_hover(&self) -> bool {
        self.widget_state.has_focus
    }

    /// Return the active status of the current widget.
    pub fn is_actived(&self) -> bool {
        self.widget_state.is_actived
    }

    /// Return the focus status of the current widget.
    pub fn is_focused(&self) -> bool {
        self.widget_state.is_focused
    }

    /// Return the mouse hover status of the current widget.
    pub fn is_hovered(&self) -> bool {
        self.widget_state.is_hovered
    }
});
