use crate::{Component, webrender::FontManager, Font, FontSize, TextLayout};

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

    pub fn layout(
        &mut self,
        text: &str,
        font: Option<&Font>,
        size: FontSize,
    ) -> TextLayout {
        self.fonts.layout(text, font, size)
    }
}
