use std::ops::Range;
use crate::{Bounds, Size, Scalar};

#[derive(Debug, Clone, PartialEq)]
pub struct TextLayout {
    pub glyphs: Vec<LayoutGlyph>,
    pub lines: Vec<LayoutLine>,
}

impl Default for TextLayout {
    fn default() -> Self {
        TextLayout {
            glyphs: Vec::default(),
            lines: Vec::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutLine {
    pub y_offset: Scalar,
    pub size: Size,
    pub glyphs: Range<usize>,
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

    pub fn clear(&mut self) {
        self.glyphs.clear();
        self.lines.clear();
    }

    pub fn size(&self) -> Size {
        if let Some(last) = self.lines.last() {
            let mut size = last.size;
            size.height += last.y_offset;
            size
        } else {
            Size::zero()
        }
    }
}
