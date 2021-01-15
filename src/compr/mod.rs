//! Composited Presenters
//!
//! The composited presenters, currently supports:
//! * [`PropertySheet`] - A collection with variant of properties,
//!   Usually used to build some settings or preferences panels.
//!
//! # PropertySheet
//!
//! A collection with variant of properties.
//!
//! ```ignore
//! use nuki::compr::{Property, PropertySheet, PropertySheetInputCtrl, PropertySheetPresenter};
//!
//! // Setup
//! let mut ps = PropertySheet::new();
//! ps.slider_f32("Brightness", (-1.0, 1.0), 0.01, 0.0);
//! ps.slider_f32("Contrast", (0.0, 2.0), 0.01, 1.0);
//! ps.slider_f32("Hue", (-1.0, 1.0), 0.01, 0.0);
//! ps.slider_f32("Saturation", (0.0, 2.0), 0.01, 1.0);
//! ps.separator();
//! ps.switch("Auto Gain", false);
//! ps.switch("Auto Focus", true);
//! let exit_callback = Arc::new(RefCell::new(
//!     move |_prop: &dyn Property, checked: bool| -> bool {
//!         // Add your code here
//!         checked
//!     },
//! ));
//! ps.action_button("Exit", "...", Arc::clone(&exit_callback));
//!
//! // Rendering
//! if nk_ctx.begin(
//!     nuki::nk_string!("Hello, PropertySheet!"),
//!     nuki::Rect {
//!         x: 200f32,
//!         y: 200f32,
//!         w: 480f32,
//!         h: 480f32,
//!     },
//!     nuki::FlagsBuilder::panel().border().title().into(),
//! ) {
//!     PropertySheetInputCtrl::new().process(&nk_ctx, &mut ps);
//!     PropertySheetPresenter::new(32.0).present(&mut nk_ctx, &ps);
//! }
//! nk_ctx.end();
//! ```

mod property_sheet;
pub use property_sheet::*;
