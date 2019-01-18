use crate::{
    Bounds, BoxConstraints, Cid, Component, Font, FontSize, Position, PropsBuilder, Renderer, Size,
    UiLayout,
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

impl<'a> PropsBuilder<Text<'a>> {
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
    content: String,
    size: FontSize,
    font: Option<Font>,
}

impl<'a> Component for Text<'a> {
    type Props = Text<'a>;
    type State = TextState;
    type Msg = ();
    type Event = ();

    fn init(props: &Self::Props) -> Self::State {
        TextState {
            content: props.content.to_string(),
            size: props.size,
            font: props.font.clone(),
        }
    }

    fn layout(
        _state: &Self::State,
        children: &[Cid],
        constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size {
        if children.len() != 0 {
            let name = ui.full_debug_name();
            log::error!(
                "The primitive Component {} has content attached to it but it will be ignored",
                name
            );
        }

        if let Some(width) = constraints.max_width {
            if let Some(height) = constraints.max_height {
                return Size {
                    w: width,
                    h: height,
                };
            }
        }

        Size::default()
    }

    fn render(state: &Self::State, bounds: Bounds, renderer: &mut Renderer) {
        use webrender::api::{
            ColorF, FontInstanceFlags, FontRenderMode, GlyphOptions, LayoutPrimitiveInfo,
        };

        let default_font = renderer.font_manager.default_font().clone();
        let font = state.font.as_ref().unwrap_or(&default_font);

        let Position { x, y } = bounds.position;
        let Size { w, h } = bounds.size;
        let info = LayoutPrimitiveInfo::new(euclid::rect(x, y, w, h));

        let font_key = renderer
            .font_manager
            .instance(font, state.size, &renderer.api)
            .unwrap();
        let glyphs =
            renderer
                .font_manager
                .layout(&state.content, font, state.size, bounds.position);

        let mut text_flags = FontInstanceFlags::empty();
        text_flags.set(FontInstanceFlags::SUBPIXEL_BGR, true);
        text_flags.set(FontInstanceFlags::LCD_VERTICAL, true);
        let text_options = GlyphOptions {
            render_mode: FontRenderMode::Subpixel,
            flags: text_flags,
        };

        renderer
            .builder
            .push_text(&info, glyphs, font_key, ColorF::WHITE, Some(text_options));
    }
}
