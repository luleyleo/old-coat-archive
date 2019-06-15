use crate::{webrender::FontManager, Font, FontSize, TextLayout};

pub struct UiDerive<'a> {
    fonts: &'a FontManager,
}

impl<'a> UiDerive<'a> {
    pub fn new(fonts: &'a FontManager) -> Self {
        UiDerive { fonts }
    }

    pub fn layout(&self, text: &str, font: Option<&Font>, size: FontSize, buffer: &mut TextLayout) {
        self.fonts.layout(text, font, size, buffer)
    }
}
