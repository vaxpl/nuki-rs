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

//! Boostrap and app lifecycle.

use crate::{Data, Env, Presenter};
use std::marker::PhantomData;

/// A function that modifies the initial environment.
type EnvSetupFn<T> = dyn FnOnce(&mut Env, &T);

// /// A function that setup the present context.
// type PresentSetupFn<T, U> = dyn FnOnce(&Env, &T) -> Box<dyn Presenter<U>>;

/// Handles initial setup of an application, and starts the runloop.
pub struct AppBuilder<T> {
    env_setup: Option<Box<EnvSetupFn<T>>>,
    // phantom: PhantomData<U>,
}

impl<T: Data> AppBuilder<T> {
    /// Construct an app launcher.
    pub fn new() -> Self {
        Self {
            env_setup: None,
            // phantom: Default::default(),
        }
    }

    /// Provide an optional closure that will be given mutable access to
    /// the environment and immutable access to the app state before launch.
    ///
    /// This can be used to set or override theme values.
    pub fn configure_env(mut self, f: impl Fn(&mut Env, &T) + 'static) -> Self {
        self.env_setup = Some(Box::new(f));
        self
    }

    // /// Provide an optional closure that will be setup the presenter.
    // pub fn configure_presenter(
    //     mut self,
    //     f: impl Fn(&Env, &T) -> Box<dyn Presenter<U>> + 'static,
    // ) -> Self {
    //     self.presenter_setup = Some(Box::new(f));
    //     self
    // }

    /// Build the state for render single frame.
    pub fn build(mut self, data: &T) -> AppState<T> {
        let mut env = Env::default();
        if let Some(f) = self.env_setup.take() {
            f(&mut env, data);
        }

        AppState::new(env)
    }
}

/// State shared by all widgets.
pub struct AppState<T> {
    env: Env,
    phantom: PhantomData<T>,
}

impl<T: Data> AppState<T> {
    pub fn new(env: Env) -> Self {
        Self {
            env,
            phantom: Default::default(),
        }
    }

    pub fn render_frame<P: Presenter>(&mut self, data: &mut T, present: &mut P) {
        println!("render_frame");
    }
}

/// Context used for `NullPresenter`.
#[derive(Copy, Clone, Debug, Default)]
pub struct NullContext;

/// A dummy presenter for test only.
#[derive(Copy, Clone, Debug, Default)]
pub struct NullPresenter {
    ctx: NullContext,
}

impl NullPresenter {
    /// Construct a dummy presenter.
    pub fn new() -> Self {
        Self {
            ctx: Default::default(),
        }
    }
}

impl Presenter for NullPresenter {
    // type Context = NullContext;

    // fn context(&self) -> &Self::Context {
    //     &self.ctx
    // }

    // fn context_mut(&mut self) -> &mut Self::Context {
    //     &mut self.ctx
    // }

    fn present(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Data, Lens};

    #[derive(Clone, Data, Lens)]
    struct MyState {
        enabled: bool,
        selected: i32,
    }

    #[test]
    fn test_app_launcher() {
        let mut data = MyState {
            enabled: false,
            selected: 0,
        };
        let mut present = NullPresenter::new();
        let mut app = AppBuilder::new().build(&data);
        app.render_frame(&mut data, &mut present);
    }
}
