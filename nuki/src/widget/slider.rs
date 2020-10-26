#![allow(unused_variables)]

use super::prelude::*;

/// A widget that allows user text input.
#[derive(Debug, Default, Clone)]
pub struct Slider;

impl Slider {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget<f32> for Slider {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &f32, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => ctx.set_has_focus(false),
            _ => {}
        }
        println!("lifecycle(Widget<f32> for Slider)");
    }

    fn present(&mut self, ctx: &mut PresentCtx, data: &f32, env: &Env) {
        println!("Slider @ {:p} data={:?}", self, data);
    }
}

impl Widget<f64> for Slider {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &f64, env: &Env) {}

    fn present(&mut self, ctx: &mut PresentCtx, data: &f64, env: &Env) {
        todo!()
    }
}

impl Widget<i32> for Slider {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &i32, env: &Env) {
        println!("lifecycle(Widget<i32> for Slider)");
    }

    fn present(&mut self, ctx: &mut PresentCtx, data: &i32, env: &Env) {
        println!("Slider @ {:p} data={:?}", self, data);
    }
}

impl Widget<i64> for Slider {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &i64, env: &Env) {}

    fn present(&mut self, ctx: &mut PresentCtx, data: &i64, env: &Env) {
        todo!()
    }
}

impl Widget<u32> for Slider {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &u32, env: &Env) {}

    fn present(&mut self, ctx: &mut PresentCtx, data: &u32, env: &Env) {
        todo!()
    }
}

impl Widget<u64> for Slider {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &u64, env: &Env) {}

    fn present(&mut self, ctx: &mut PresentCtx, data: &u64, env: &Env) {
        todo!()
    }
}
