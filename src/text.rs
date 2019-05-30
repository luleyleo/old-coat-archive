use crate::{Bounds, Position, Size};

#[derive(Debug, Clone, PartialEq)]
pub struct TextLayout {
    pub size: Size,
    pub glyphs: Vec<LayoutGlyph>,
}

impl Default for TextLayout {
    fn default() -> Self {
        TextLayout {
            size: Size::zero(),
            glyphs: Vec::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutGlyph {
    pub index: u32,
    pub bounds: Bounds,
}

impl TextLayout {
    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }

    pub fn index_at(&self, position: Position) -> Option<usize> {
        for (index, glyph) in self.glyphs.iter().enumerate() {
            // TODO: This is rather restrictive as it requires hitting a glyph
            if glyph.bounds.contains(&position) {
                return Some(index);
            }
        }
        None
    }
}
