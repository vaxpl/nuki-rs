use super::prelude::*;
use std::marker::PhantomData;

pub struct Property<T> {
    title: &'static str,
    child: WidgetPod<T, Box<dyn Widget<T>>>,
}

pub struct PropertyTitled<T> {
    title: &'static str,
    phantom: PhantomData<T>,
}

impl<T> PropertyTitled<T> {
    pub fn with(self, child: impl Widget<T> + 'static) -> Property<T> {
        Property {
            title: self.title,
            child: WidgetPod::new(Box::new(child)),
        }
    }
}

impl<T> Property<T> {
    pub fn new(title: &'static str) -> PropertyTitled<T> {
        PropertyTitled {
            title,
            phantom: Default::default(),
        }
    }
}

impl<T: Data> Widget<T> for Property<T> {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                // DONT ADD TO FOCUS_CHAIN
            }
            _ => self.child.lifecycle(ctx, event, data, env),
        }
        println!("lifecycle(Widget<T> for Property<T>)");
    }

    fn present(&mut self, ctx: &mut PresentCtx, data: &T, env: &Env) {
        self.child.present(ctx, data, env);
        println!("present(Widget<T> for Property<T>)");
    }
}
