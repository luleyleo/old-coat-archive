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

/// TODO: This should be replaced with a proper font resulution system
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
