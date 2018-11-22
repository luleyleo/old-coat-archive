use std::rc::Rc;
use std::ops;

pub struct Str(Rc<String>);

impl Str {
    pub fn new() -> Self {
        Str(Rc::new(String::new()))
    }

    pub fn is_unique(&self) -> bool {
        Rc::strong_count(&self.0) + Rc::weak_count(&self.0) == 1
    }
}

impl ops::Deref for Str {
    type Target = str;

    fn deref(&self) -> &str {
        self.0.as_ref()
    }
}

impl ops::DerefMut for Str {
    fn deref_mut(&mut self) -> &mut str {
        Rc::make_mut(&mut self.0).as_mut_str()
    }
}

impl std::convert::From<String> for Str {
    fn from(other: String) -> Self {
        Str(Rc::new(other))
    }
}

impl std::convert::From<&str> for Str {
    fn from(other: &str) -> Self {
        Str(Rc::new(other.to_string()))
    }
}
