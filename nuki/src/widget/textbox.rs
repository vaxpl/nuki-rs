// Copyright 2018 The Druid Authors.
// Copyright 2020 The Druid Authors.
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

//! A textbox widget.

use super::prelude::*;

/// A widget that allows user text input.
#[derive(Debug, Clone)]
pub struct TextBox<T> {
    text: T,
}

impl<T: Default> TextBox<T> {
    /// Create a new TextBox widget
    pub fn new() -> Self {
        Self {
            text: Default::default(),
        }
    }
}

impl<T: Data> Widget<T> for TextBox<T> {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {}

    fn present(&mut self, ctx: &mut PresentCtx, data: &T, env: &Env) {}
}

impl<T: Default> Default for TextBox<T> {
    fn default() -> Self {
        TextBox::new()
    }
}
