use nuki::nuklear as nk;
use nuki::widget::prelude::*;
use nuki::widget::{Flex, Slider, WidgetExt, WidgetState};
use nuki::{ContextState, Env, FocusChain, LifeCycle, LifeCycleCtx, Presenter};
use nuki_backend_gles as nkbe;
use nuki_derive::{Data, Lens};
use std::fs::File;
use std::io::Read;

pub struct GlesPresenter<'a> {
    allo: nk::Allocator,
    ctx: nk::Context,
    drawer: nkbe::Drawer<'a>,
    drawer_options: nkbe::DrawOptions,
}

impl<'a> GlesPresenter<'a> {
    pub fn new() -> Self {
        let allo = nk::Allocator::new_heap();

        // Create a drawer
        const MAX_VERTEX_MEMORY: usize = 512 * 1024;
        const MAX_ELEMENT_MEMORY: usize = 128 * 1024;
        let mut drawer =
            nkbe::Drawer::new(Clone::clone(&allo), MAX_VERTEX_MEMORY, MAX_ELEMENT_MEMORY);

        // Load font to buffer
        let mut file = File::open("/usr/share/fonts/TTZhongHeiJ.ttf").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        // Create a font atlas
        let mut atlas = nk::FontAtlas::new(&allo);

        // Add a font with config
        let mut cfg = nk::FontConfig::with_size(0.0);
        cfg.set_oversample_h(3);
        cfg.set_oversample_v(2);
        let ranges = simplify_glyph_ranges();
        cfg.set_glyph_range(&ranges[0..]);
        cfg.set_ttf_data_owned_by_atlas(false);
        cfg.set_ttf(&buffer);
        cfg.set_size(16f32);
        let font_regular = atlas.add_font_with_config(&cfg).unwrap();
        // Bake the font atlas texture
        let font_handle = drawer.bake_font_atlas(&mut atlas);
        // Cleanup the temporary resources in font atlas
        atlas.cleanup();
        // Drop the font buffer
        drop(buffer);

        let mut ctx = nk::Context::new(&allo, atlas.font(font_regular).unwrap().handle());

        let colors: [nk::Color; 28usize] = crate::Theme::Blue.into();
        let color_table = nk::ColorMap::from(colors);
        ctx.style_from_table(&color_table);
        ctx.style_mut()
            .window_mut()
            .set_scrollbar_size(nk::vec2(4.0, 4.0));

        let (w, h) = (1920, 1080);
        let drawer_options =
            nkbe::DrawOptions::new(w as usize, h as usize).with_scale_factor(1.0, 1.0);

        Self {
            allo,
            ctx,
            drawer,
            drawer_options,
        }
    }
}

impl<'a> Presenter for GlesPresenter<'a> {
    // type Context = nk::Context;

    // fn context(&self) -> &Self::Context {
    //     &self.ctx
    // }

    // fn context_mut(&mut self) -> &mut Self::Context {
    //     &mut self.ctx
    // }

    fn present(&mut self) {}
}

fn simplify_glyph_ranges() -> Vec<(u32, u32)> {
    let cjk = "这是中文字符";
    let mut ranges: Vec<(u32, u32)> = vec![];
    ranges.push((32, 255));
    for a in cjk.chars() {
        ranges.push((a as u32, a as u32));
    }
    ranges.push((0, 0));
    ranges
}
