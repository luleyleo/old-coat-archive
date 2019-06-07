use crate::{ButtonState, KeyboardEvent, MouseEvent, VirtualKeyCode};

#[derive(Default, Debug, Clone)]
pub struct Buffer {
    text: String,
    cursor: usize,
}

#[derive(Debug)]
pub enum BufferUpdate {
    Insert(char),
    Keyboard(KeyboardEvent),
    Mouse { event: MouseEvent, position: usize },
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

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn unwrap(self) -> String {
        self.text
    }

    pub fn event_filter(event: &KeyboardEvent) -> bool {
        false
            || event.keycode == Some(VirtualKeyCode::Backspace)
            || event.keycode == Some(VirtualKeyCode::Delete)
            || event.keycode == Some(VirtualKeyCode::Left)
            || event.keycode == Some(VirtualKeyCode::Right)
    }

    pub fn update(&mut self, update: BufferUpdate) {
        match update {
            BufferUpdate::Insert(ch) => {
                self.text.insert(self.cursor, ch);
                self.cursor += 1;
            }
            BufferUpdate::Keyboard(event) => {
                if event.state == ButtonState::Pressed {
                    let cursor = self.cursor as isize;
                    match event.keycode {
                        Some(VirtualKeyCode::Backspace) => self.delete(-1),
                        Some(VirtualKeyCode::Delete) => self.delete(1),
                        Some(VirtualKeyCode::Left) => self.move_cursor(cursor - 1),
                        Some(VirtualKeyCode::Right) => self.move_cursor(cursor + 1),
                        _ => (),
                    }
                }
            }
            BufferUpdate::Mouse { event, position } => match event.state {
                ButtonState::Pressed => {
                    self.move_cursor(position as isize);
                }
                ButtonState::Released => {
                    // TODO: Make the cursor a `Range`
                }
            },
        }
    }

    pub fn move_cursor(&mut self, position: isize) {
        self.cursor = position.max(0).min(self.text.len() as isize) as usize;
    }

    pub fn delete(&mut self, direction: isize) {
        if direction == 0 {
            return;
        }
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
                raw.1.max(0).min(len) as usize,
            )
        };
        self.text.replace_range(start..end, "");
        self.cursor = start;
    }
}
