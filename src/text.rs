use crate::{Bounds, Position, Size};

#[derive(Default, Debug, Clone)]
pub struct Buffer {
    text: String,
    cursor: usize,
}

pub enum BufferUpdate {
    Insert(char),
    Delete(isize),
}

impl From<String> for Buffer {
    fn from(string: String) -> Self {
        let len = string.len();
        Buffer {
            text: string,
            cursor: len,
        }
    }
}

impl Buffer {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn unwrap(self) -> String {
        self.text
    }

    pub fn update(&mut self, update: BufferUpdate) {
        use BufferUpdate::*;
        match update {
            Insert(ch) => {
                self.text.insert(self.cursor, ch);
                self.cursor += 1;
            }
            Delete(direction) => {
                if direction == 0 { return }
                let len = self.text.len() as isize;
                let (start, end) = {
                    let cursor = self.cursor as isize;
                    let target = cursor + direction;
                    let raw = if direction < 0 {
                        (target, cursor)
                    } else {
                        (cursor, target)
                    };
                    assert!(raw.1 >= 0);
                    // Remember that `.min` and `.max` are weird
                    (
                        raw.0.max(0).min(len) as usize,
                        raw.1.max(0).min(len) as usize
                    )
                };
                self.text.replace_range(start..end, "");
                self.cursor = start;
            }
        }
    }
}

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
