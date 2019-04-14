use crate::{Component, webrender::FontManager, Font, FontSize, Size, Position};

pub struct UiDerive<'a, C: Component> {
    events: &'a mut Vec<C::Event>,
    fonts: &'a mut FontManager,
    needs_update: bool,
}

impl<'a, C: Component> UiDerive<'a, C> {
    pub fn new(events: &'a mut Vec<C::Event>, fonts: &'a mut FontManager) -> Self {
        UiDerive {
            events,
            fonts,
            needs_update: false,
        }
    }

    pub fn emit(&mut self, event: C::Event) {
        self.events.push(event);
        self.needs_update = true;
    }

    pub fn dimensions(&self, text: &str, font: &Font, size: FontSize) -> Size {
        self.fonts.dimensions(text, font, size)
    }

    pub fn layout(
        &mut self,
        text: &str,
        font: &Font,
        size: FontSize,
        position: Position,
    ) -> &[webrender::api::GlyphInstance] {
        self.fonts.layout(text, font, size, position)
    }
    
    pub(crate) fn needs_update(&self) -> bool {
        self.needs_update
    }
}
