use std::borrow::Cow;

pub type FontSize = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontWeight {
    Thin,
    Ultralight,
    Light,
    Semilight,
    Book,
    Regular,
    Medium,
    Semibold,
    Bold,
    Ultrabold,
    Heavy,
    Ultraheavy,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Font {
    pub(crate) family: Cow<'static, str>,
    pub(crate) weight: FontWeight,
    pub(crate) italic: bool,
}

impl Font {
    pub fn from_family(family: impl Into<Cow<'static, str>>) -> Self {
        Font {
            family: family.into(),
            weight: FontWeight::Regular,
            italic: false,
        }
    }

    pub fn weight(&mut self, weight: FontWeight) {
        self.weight = weight;
    }

    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn italic(&mut self, italic: bool) {
        self.italic = italic;
    }

    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }
}

#[derive(Default)]
pub(crate) struct FontQueue(Vec<FontQueueAction>);

impl FontQueue {    
    pub fn add(&mut self, font: Font, data: impl Into<Vec<u8>>) {
        self.0.push(FontQueueAction::Add(font, data.into()));
    }

    pub fn remove(&mut self, font: Font) {
        self.0.push(FontQueueAction::Remove(font));
    }

    pub fn drain(&mut self) -> std::vec::Drain<FontQueueAction> {
        self.0.drain(..)
    }
}

pub(crate) enum FontQueueAction {
    Add(Font, Vec<u8>),
    Remove(Font),
}
