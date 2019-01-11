use crate::{Bounds, Component, FontSize, PropsBuilder, Renderer, Position, Size, FontId, Cid, BoxConstraints, UiLayout};

pub struct Text<'a> {
    content: &'a str,
    size: FontSize,
}

impl<'a> Default for Text<'a> {
    fn default() -> Self {
        Self {
            content: "",
            size: 12,
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
}

pub struct TextState {
    content: String,
    size: FontSize,
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
        use webrender::api::*;
        let Position {x, y} = bounds.position;
        let Size {w, h} = bounds.size;
        let info = LayoutPrimitiveInfo::new(euclid::rect(x, y, w, h));
        let font_key = renderer.font_manager.load_instance(FontId(0), state.size, &renderer.api).unwrap();
        let glyphs = layout(&state.content, state.size, bounds.position, renderer.font_manager.load_rusttype(FontId(0)));
        let mut text_flags = FontInstanceFlags::empty();
        text_flags.set(FontInstanceFlags::SUBPIXEL_BGR, true);
        text_flags.set(FontInstanceFlags::LCD_VERTICAL, true);
        let text_options = GlyphOptions {
            render_mode: FontRenderMode::Subpixel,
            flags: text_flags,
        };
        renderer.builder.push_text(
            &info,
            glyphs.as_slice(),
            font_key,
            ColorF::WHITE,
            Some(text_options),
        );
    }
}

pub fn layout<'a>(
    text: &str,
    size: FontSize,
    position: Position,
    font: &'a rusttype::Font<'static>,
) -> Vec<webrender::api::GlyphInstance> {
    use webrender::api::{GlyphInstance, LayoutPoint};
    let size = size as f32;
    let scale = rusttype::Scale {
        // TODO: Fix glyph overlapping without additional x-scaling
        // The current value roughly fits OpenSans
        x: size * 1.2,
        y: size,
    };
    let point = rusttype::Point { x: position.x, y: position.y };
    font.layout(text, scale, point)
        .map(|glyph| {
            let index = glyph.id().0;
            let pos = glyph.position();
            let point = LayoutPoint::new(pos.x, pos.y + size);
            GlyphInstance { index, point }
        })
        .collect()
}
