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

//! The fundamental nuki types.

mod app;
mod context;
mod counter;
pub mod data;
mod env;
mod event;
mod focus;
pub mod lens;
mod pool;
mod text;

pub use app::{AppBuilder, AppState, NullContext, NullPresenter};
pub use context::{ContextState, LifeCycleCtx, PresentCtx};
pub use counter::Counter;
pub use data::Data;
pub use env::{Env, Key, KeyLike, KeyOrValue, MissingKeyError, ValueTypeError};
pub use event::{Event, LifeCycle};
pub use focus::FocusChain;
pub use lens::{Lens, LensExt};
pub use pool::{
    ForwardPool, ForwardPoolIter, ForwardPoolIterMut, PoolObject, PoolObjectBase, PoolObjectTypeId,
};
pub use text::ArcStr;
