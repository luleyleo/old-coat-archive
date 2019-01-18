use crate::{bounds::Size, Scalar};

#[derive(Debug, Clone, Copy, Default)]
pub struct BoxConstraints {
    pub min_width: Scalar,
    pub min_height: Scalar,
    pub max_width: Option<Scalar>,
    pub max_height: Option<Scalar>,
}

impl BoxConstraints {
    pub fn new_tight(size: Size) -> Self {
        Self::default().min(size).max(size)
    }

    pub fn min(self, size: Size) -> Self {
        self.min_width(size.width).min_height(size.height)
    }

    pub fn max(self, size: Size) -> Self {
        self.max_width(size.width).max_height(size.height)
    }

    pub fn min_width(mut self, width: Scalar) -> Self {
        self.min_width = width;
        self
    }

    pub fn min_height(mut self, height: Scalar) -> Self {
        self.min_height = height;
        self
    }

    pub fn max_width(mut self, width: Scalar) -> Self {
        self.max_width = Some(width);
        self
    }

    pub fn max_height(mut self, height: Scalar) -> Self {
        self.max_height = Some(height);
        self
    }

    pub fn check_width(&self, width: Scalar) -> Scalar {
        if width < self.min_width {
            self.min_width
        } else if let Some(max_width) = self.max_width {
            if width > max_width {
                max_width
            } else {
                width
            }
        } else {
            width
        }
    }

    pub fn check_height(&self, height: Scalar) -> Scalar {
        if height < self.min_height {
            self.min_height
        } else if let Some(max_height) = self.max_height {
            if height > max_height {
                max_height
            } else {
                height
            }
        } else {
            height
        }
    }

    pub fn check_size(&self, size: Size) -> Size {
        Size::new(self.check_width(size.width), self.check_height(size.height))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_width() {
        let width = 10.0;

        let larger = BoxConstraints::new_tight(Size::new(20.0, 0.0));
        assert_eq!(larger.check_width(width), 20.0);

        let smaller = BoxConstraints::new_tight(Size::new(5.0, 0.0));
        assert_eq!(smaller.check_width(width), 5.0);
    }

    #[test]
    fn test_check_height() {
        let height = 10.0;

        let larger = BoxConstraints::new_tight(Size::new(0.0, 20.0));
        assert_eq!(larger.check_height(height), 20.0);

        let smaller = BoxConstraints::new_tight(Size::new(0.0, 5.0));
        assert_eq!(smaller.check_height(height), 5.0);
    }
}
