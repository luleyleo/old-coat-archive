pub type FontSize = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontId(pub usize);

#[derive(Default)]
pub(crate) struct FontQueue {
    queue: Vec<FontQueueAction>,
    next_font_id: usize,
}

impl FontQueue {
    pub fn add(&mut self, data: impl Into<Vec<u8>>) -> FontId {
        let fid = FontId(self.next_font_id);
        self.queue.push(FontQueueAction::Add(fid, data.into()));
        self.next_font_id += 1;
        return fid;
    }

    pub fn remove(&mut self, fid: FontId) {
        self.queue.push(FontQueueAction::Remove(fid));
    }

    pub fn drain(&mut self) -> std::vec::Drain<FontQueueAction> {
        self.queue.drain(..)
    }
}

pub(crate) enum FontQueueAction {
    Add(FontId, Vec<u8>),
    Remove(FontId),
}
