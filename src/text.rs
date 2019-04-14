use crate::{Bounds, Position};
use webrender::api::GlyphInstance;

#[derive(Debug, Default, PartialEq)]
pub struct GlyphBounds(Vec<Bounds>);

impl GlyphBounds {
    pub fn new(data: Vec<Bounds>) -> Self {
        GlyphBounds(data)
    }

    pub fn move_by(&mut self, position: Position) {
        for glyph in &mut self.0 {
            glyph.origin += position.to_vector();
        }
    }

    pub fn index_at(&self, position: Position) -> Option<usize> {
        for (index, glyph) in self.0.iter().enumerate() {
            // TODO: This is rather restrictive as it requires hitting a glyph
            if glyph.contains(&position) {
                return Some(index);
            }
        }
        None
    }
}

impl From<&[GlyphInstance]> for GlyphBounds {
    fn from(glyphs: &[GlyphInstance]) -> Self {
        let data = Vec::with_capacity(glyphs.len());

        Self::new(data)
    }
}
