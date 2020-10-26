// Copyright 2018 The Druid Authors.
// Copyright 2018 The Nuki Authors.
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

//! Common widgets.

mod flex;
mod identity_wrapper;
mod label;
mod lens_wrap;
mod property;
mod slider;
mod textbox;
mod widget;
mod widget_ext;

pub use flex::{Axis, Flex, FlexParams};
pub use identity_wrapper::IdentityWrapper;
pub use label::Label;
pub use lens_wrap::LensWrap;
pub use property::Property;
pub use slider::Slider;
pub use textbox::TextBox;
pub use widget::{Widget, WidgetId, WidgetPod, WidgetState};
pub use widget_ext::WidgetExt;

pub mod prelude {
    pub use super::{IdentityWrapper, Widget, WidgetExt, WidgetId, WidgetPod};
    pub use crate::{Data, Env, Event, Lens, LifeCycle, LifeCycleCtx, PresentCtx};
}
