pub type Scalar = f32;

mod bounds;
pub use self::bounds::{Bounds, Position, Size};

mod builder;
pub use self::builder::ContentBuilder;

mod component;
use self::component::ComponentPointer;
pub use self::component::Component;

mod constraints;
pub use self::constraints::BoxConstraints;

mod ids;
pub use self::ids::{Cid, Iid};

pub mod ui;
use self::ui::{full_debug_name_of, TypeIds, UiData, UiInputBase, UiRender};
pub use self::ui::{UiInput, UiLayout, UiUpdate, UiView, UiDerive};

mod mutable;
pub use self::mutable::Mut;

pub mod backend;
pub use self::backend::*;

mod layouts;
pub use self::layouts::*;

mod widgets;
pub use self::widgets::*;

mod color;
pub use self::color::Color;

mod input;
use self::input::Input;
pub use self::input::{Event, ButtonState, ModifiersState, MouseButton, TouchPhase, VirtualKeyCode};

mod text;
pub use self::text::{TextLayout, LayoutGlyph};

mod font;
pub use self::font::{Font, FontSize, FontWeight};
