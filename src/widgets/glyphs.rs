use crate::{
    Bounds, BoxConstraints, Cid, Component, Font, FontSize, LayoutGlyph, Renderer, Size,
    TextLayout, UiDerive, UiLayout,
};

pub struct Glyphs<'a> {
    size: FontSize,
    font: Option<Font>,
    layout: Option<&'a TextLayout>,
}

pub struct GlyphsState {
    size: FontSize,
    font: Option<Font>,
    layout: TextLayout,
}

impl Default for Glyphs<'_> {
    fn default() -> Self {
        Self {
            size: 12,
            font: None,
            layout: None,
        }
    }
}

impl<'a> Glyphs<'a> {
    pub fn text(mut self, text: &'a TextLayout) -> Self {
        self.layout = Some(text);
        self
    }

    pub fn size(mut self, size: FontSize) -> Self {
        self.size = size;
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }
}

impl Component for Glyphs<'_> {
    type State = GlyphsState;
    type Msg = ();
    type Event = ();

    fn init(props: &Self) -> Self::State {
        let layout = props.layout.cloned().unwrap_or_default();
        GlyphsState {
            size: props.size,
            font: props.font.clone(),
            layout,
        }
    }

    fn derive_state(props: &Self, state: &mut Self::State, _ui: &mut UiDerive<Self>) {
        if props.size != state.size {
            state.size = props.size;
        }
        if props.font != state.font {
            state.font = props.font.clone();
        }
        if props.layout != Some(&state.layout) {
            state.layout = props.layout.cloned().unwrap_or_default();
        }
    }

    fn layout(
        state: &Self::State,
        children: &[Cid],
        _constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size {
        if children.len() != 0 {
            let name = ui.full_debug_name();
            log::error!(
                "The primitive Component {} has content attached to it but it will be ignored",
                name
            );
        }
        state.layout.size
    }

    fn render(state: &Self::State, bounds: Bounds, renderer: &mut Renderer) {
        use webrender::api::{
            ColorF, FontInstanceFlags, FontRenderMode, GlyphInstance, GlyphOptions, LayoutPoint,
            LayoutPrimitiveInfo, SpecificDisplayItem, TextDisplayItem,
        };

        let default_font = renderer.font_manager.default_font().clone();
        let font = state.font.as_ref().unwrap_or(&default_font);

        let fm = &mut renderer.font_manager;
        let font_key = fm.instance(font, state.size, &renderer.api).unwrap();
        let mut dim = state.layout.size;

        let wr_glyph = |g: &LayoutGlyph| GlyphInstance {
            index: g.index,
            point: LayoutPoint::from_untyped(&(bounds.origin + g.bounds.origin.to_vector())),
        };
        let glyphs = state.layout.glyphs.iter().map(wr_glyph);

        if dim.width > bounds.size.width || dim.height > bounds.size.height {
            dim = bounds.size;
            // TODO: log with debug name of the component
            log::warn!("Text overflow while rendering Glyphs");
        }
        let pos = bounds.origin;
        let info = LayoutPrimitiveInfo::new(euclid::rect(pos.x, pos.y, dim.width, dim.height));

        let mut text_flags = FontInstanceFlags::empty();
        text_flags.set(FontInstanceFlags::SUBPIXEL_BGR, true);
        text_flags.set(FontInstanceFlags::LCD_VERTICAL, true);
        let text_options = GlyphOptions {
            render_mode: FontRenderMode::Subpixel,
            flags: text_flags,
        };

        let item = SpecificDisplayItem::Text(TextDisplayItem {
            color: ColorF::WHITE,
            font_key,
            glyph_options: Some(text_options),
        });

        // TODO: Will no longer work with newer webrender
        renderer.builder.push_item(&item, &info);
        // TODO: This is DANGEROUS! It should check for webrenders MAX_TEXT_RUN_LENGTH
        renderer.builder.push_iter(glyphs);
    }
}
