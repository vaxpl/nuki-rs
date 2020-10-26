use nuki::nuklear as nk;
use nuki::widget::prelude::*;
use nuki::widget::{Flex, Property, Slider, WidgetExt, WidgetState};
use nuki::{
    AppBuilder, AppState, ContextState, Env, FocusChain, LifeCycle, LifeCycleCtx, NullPresenter,
    Presenter,
};
use nuki_derive::{Data, Lens};
use nuki_prefab::GlesPresenter;

#[derive(Clone, Debug, Default, Data, Lens)]
struct MyState {
    x: f32,
    y: f32,
    slot: i32,
}

fn main() {
    let mut data = MyState {
        x: 123.0,
        y: 456.0,
        slot: 999,
    };

    // let mut presenter = GlesPresenter::new();
    // let mut app: AppState<_> = AppBuilder::new().build(&data);
    // app.render_frame(&mut data, &mut presenter);

    let mut env = Env::new();
    let mut focus_chain = FocusChain::new();
    let mut ctx_state = ContextState {
        focus_chain: &mut focus_chain,
    };
    let mut widget_state = WidgetState::new();

    let mut row = Flex::<MyState>::row();
    // let mut col = Flex::<MyState>::column();
    row.add_child(Property::new("X Offset").with(Slider::new().lens(MyState::x)));
    row.add_child(Property::new("Y Offset").with(Slider::new().lens(MyState::y)));
    row.add_child(Property::new("Slot").with(Slider::new().lens(MyState::slot)));
    // col.add_child(Slider::new().lens(MyState::x));
    // col.add_child(Slider::new().lens(MyState::y));
    // col.add_child(Slider::new().lens(MyState::slot));
    // row.add_child(col);

    {
        let mut ctx = LifeCycleCtx::new(&mut ctx_state, &mut widget_state);
        let event = LifeCycle::WidgetAdded;
        row.lifecycle(&mut ctx, &event, &data, &env);
    }

    {
        let mut presenter = NullPresenter::new();
        let mut ctx = PresentCtx::new(&mut ctx_state, &mut widget_state, &mut presenter);
        row.present(&mut ctx, &data, &env);
    }
}
