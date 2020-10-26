use crate::widget::prelude::*;

/// Trait to provide present.
pub trait Presenter {
    // fn context(&self) -> &Self::Context;
    // fn context_mut(&mut self) -> &mut Self::Context;

    // fn draw_prop_slider_f32<T: Widget<U>, U>(&mut self, widget: T);

    fn draw_static_text()
    fn present(&mut self);
}
