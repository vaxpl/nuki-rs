// Allows to use macros from nuki_derive in this crate
extern crate self as nuki;

use log::trace;

pub mod core;
pub mod draw;
pub mod nuklear;
pub mod widget;

pub use crate::core::{data, lens};
pub use crate::core::{
    AppBuilder, AppState, ArcStr, ContextState, Counter, Data, Env, Event, FocusChain, ForwardPool,
    ForwardPoolIter, ForwardPoolIterMut, Key, KeyLike, KeyOrValue, Lens, LensExt, LifeCycle,
    LifeCycleCtx, MissingKeyError, NullContext, NullPresenter, PoolObject, PoolObjectBase,
    PoolObjectTypeId, PresentCtx, ValueTypeError,
};
pub use crate::draw::Presenter;
pub use crate::nuklear::Color;
pub use crate::widget::{Widget, WidgetExt, WidgetId, WidgetPod, WidgetState};

pub use nuki_derive as derive;
