// Copyright 2019 The Druid Authors.
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

//! Events.

/// An event, propagated downwards during event flow.
///
/// Events are things that happen that can change the state of widgets.
/// An important category is events plumbed from the platform windowing
/// system, which includes mouse and keyboard events, but also (in the
/// future) status changes such as window focus changes.
///
/// Events can also be higher level concepts indicating state changes
/// within the widget hierarchy, for example when a widget gains or loses
/// focus or "hot" (also known as hover) status.
///
/// Events are a key part of what is called "event flow", which is
/// basically the propagation of an event through the widget hierarchy
/// through the [`event`] widget method. A container widget will
/// generally pass the event to its children, mediated through the
/// [`WidgetPod`] container, which is where most of the event flow logic
/// is applied (especially the decision whether or not to propagate).
///
/// This enum is expected to grow considerably, as there are many, many
/// different kinds of events that are relevant in a GUI.
///
/// [`event`]: trait.Widget.html#tymethod.event
/// [`WidgetPod`]: struct.WidgetPod.html
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Dummy,
    Attach,
    Detach,
}

/// Application life cycle events.
#[derive(Clone, Copy, Debug)]
pub enum LifeCycle {
    /// Sent to a `Widget` when it is added to the widget tree. This should be
    /// the first message that each widget receives.
    ///
    /// Widgets should handle this event in order to do any initial setup.
    ///
    /// In addition to setup, this event is also used by the framework to
    /// track certain types of important widget state.
    ///
    /// ## Registering children
    ///
    /// Container widgets (widgets which use [`WidgetPod`] to manage children)
    /// must ensure that this event is forwarded to those children. The [`WidgetPod`]
    /// itself will handle registering those children with the system; this is
    /// required for things like correct routing of events.
    ///
    /// ## Participating in focus
    ///
    /// Widgets which wish to participate in automatic focus (using tab to change
    /// focus) must handle this event and call [`LifeCycleCtx::register_for_focus`].
    ///
    /// [`LifeCycleCtx::register_child`]: struct.LifeCycleCtx.html#method.register_child
    /// [`WidgetPod`]: struct.WidgetPod.html
    /// [`LifeCycleCtx::register_for_focus`]: struct.LifeCycleCtx.html#method.register_for_focus
    WidgetAdded,
    WidgetRemoved,
}
