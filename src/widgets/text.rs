use crate::{
    Bounds, BoxConstraints, Cid, Component, Font, FontSize, LayoutGlyph, Renderer, Size,
    TextLayout, UiDerive, UiLayout,
};

pub struct Text<'a> {
    content: &'a str,
    size: FontSize,
    font: Option<Font>,
}

impl<'a> Default for Text<'a> {
    fn default() -> Self {
        Self {
            content: "",
            size: 12,
            font: None,
        }
    }
}

impl<'a> Text<'a> {
    pub fn content(mut self, content: &'a str) -> Self {
        self.content = content;
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

pub struct TextState {
    pub(crate) content: String,
    pub(crate) size: FontSize,
    pub(crate) font: Option<Font>,
    pub(crate) layout: TextLayout,
}

pub type TextEvent = TextLayout;

impl<'a> Component for Text<'a> {
    type State = TextState;
    type Msg = ();
    type Event = TextEvent;

    fn init(props: &Self) -> Self::State {
        TextState {
            content: String::default(),
            size: props.size,
            font: props.font.clone(),
            layout: TextLayout::default(),
        }
    }

    fn derive_state(props: &Self, state: &mut Self::State, ui: &mut UiDerive<Self>) {
        let mut changed = false;
        if props.content != state.content {
            state.content.replace_range(.., props.content);
            changed = true;
        }
        if props.size != state.size {
            state.size = props.size;
            changed = true;
        }
        if props.font != state.font {
            state.font = props.font.clone();
            changed = true;
        }
        if changed {
            state.layout = ui.layout(props.content, props.font.as_ref(), props.size);
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

        // TODO: Some sort of ellipsis or so if the constraints are to small
        let size = state.layout.size;

        size
    }

    fn render(state: &Self::State, bounds: Bounds, renderer: &mut Renderer) {
        render_text(state, bounds, renderer);
    }
}

pub fn render_text(state: &TextState, bounds: Bounds, renderer: &mut Renderer) {
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
        log::warn!("Text overflow while rendering \"{}\"", state.content);
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
