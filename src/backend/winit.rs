mod eventloop;
pub use self::eventloop::EventLoop;

mod window;
pub use self::window::{AppEvent, Window};

mod event_handler;
use self::event_handler::EventHandler;

pub(crate) static DEFAULT_FONT: &[u8] = include_bytes!("../../assets/fonts/OpenSans-Regular.ttf");
pub(crate) static DEFAULT_FONT_NAME: &str = "OpenSans";
