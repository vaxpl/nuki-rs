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

//! Convenience methods for widgets.

use super::{IdentityWrapper, LensWrap, Widget, WidgetId};
use crate::{Data, Lens};

/// A trait that provides extra methods for combining `Widget`s.
pub trait WidgetExt<T: Data>: Widget<T> + Sized + 'static {
    /// Wrap this widget in a [`LensWrap`] widget for the provided [`Lens`].
    ///
    ///
    /// [`LensWrap`]: struct.LensWrap.html
    /// [`Lens`]: trait.Lens.html
    fn lens<S: Data, L: Lens<S, T>>(self, lens: L) -> LensWrap<T, L, Self> {
        LensWrap::new(self, lens)
    }

    /// Assign the widget a specific [`WidgetId`].
    ///
    /// You must ensure that a given [`WidgetId`] is only ever used for
    /// a single widget at a time.
    ///
    /// An id _may_ be reused over time; for instance if you replace one
    /// widget with another, you may reuse the first widget's id.
    ///
    /// [`WidgetId`]: struct.WidgetId.html
    fn with_id(self, id: WidgetId) -> IdentityWrapper<Self> {
        IdentityWrapper::wrap(self, id)
    }

    /// Wrap this widget in a `Box`.
    fn boxed(self) -> Box<dyn Widget<T>> {
        Box::new(self)
    }
}

impl<T: Data, W: Widget<T> + 'static> WidgetExt<T> for W {}

#[cfg(test)]
mod tests {
    use super::*;
}
