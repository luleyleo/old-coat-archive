use crate::Scalar;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Color {
    pub r: Scalar,
    pub g: Scalar,
    pub b: Scalar,
    pub a: Scalar,
}

impl Color {
    pub fn rgb(r: Scalar, g: Scalar, b: Scalar) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    pub fn rgba(r: Scalar, g: Scalar, b: Scalar, a: Scalar) -> Self {
        Color { r, g, b, a }
    }
}
