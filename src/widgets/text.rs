use crate::{
    Bounds, BoxConstraints, Cid, Component, Font, FontSize, Properties, Renderer, Size, UiLayout, TextLayout, UiDerive,
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

impl<'a> Properties for Text<'a> {
    type Component = Self;
}

pub struct TextState {
    content: String,
    size: FontSize,
    font: Option<Font>,
    layout: TextLayout,
}

pub type TextEvent = TextLayout;

impl<'a> Component for Text<'a> {
    type Props = Text<'a>;
    type State = TextState;
    type Msg = ();
    type Event = TextEvent;

    fn init(props: &Self::Props) -> Self::State {
        TextState {
            content: String::default(),
            size: props.size,
            font: props.font.clone(),
            layout: TextLayout::default(),
        }
    }

    fn derive_state(props: &Self::Props, state: &mut Self::State, ui: &mut UiDerive<Self>) {
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
        let size = ui.get_text_size(&state.content, state.font.as_ref(), state.size);

        size
    }

    fn render(state: &Self::State, bounds: Bounds, renderer: &mut Renderer) {
        use webrender::api::{
            ColorF, FontInstanceFlags, FontRenderMode, GlyphOptions, LayoutPrimitiveInfo, GlyphInstance, LayoutPoint,
        };

        let default_font = renderer.font_manager.default_font().clone();
        let font = state.font.as_ref().unwrap_or(&default_font);

        let fm = &mut renderer.font_manager;
        let font_key = fm.instance(font, state.size, &renderer.api).unwrap();
        let mut dim = fm.dimensions(&state.content, font, state.size);
        let glyphs: Vec<GlyphInstance> = state.layout.glyphs.iter().map(|g| GlyphInstance {
            index: g.index,
            point: LayoutPoint::from_untyped(&(bounds.origin + g.bounds.origin.to_vector())),
        }).collect(); // TODO: Avoid allocation at all costs! (pass an iterator to webrender?)

        // Check weather the text is larger than the bounds
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

        renderer
            .builder
            .push_text(&info, &glyphs, font_key, ColorF::WHITE, Some(text_options));
    }
}
