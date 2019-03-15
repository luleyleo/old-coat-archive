pub type Scalar = f32;

mod bounds;
pub use self::bounds::{Bounds, Position, Size};

mod builder;
pub use self::builder::{ContentBuilder, PropsBuilder, ReactivePropsBuilder};

mod component;
pub use self::component::Component;
use self::component::ComponentPointer;

mod constraints;
pub use self::constraints::BoxConstraints;

mod ids;
pub use self::ids::{Cid, Iid};

pub mod ui;
use self::ui::{full_debug_name_of, TypeIds, UiData, UiInputBase, UiRender};
pub use self::ui::{UiInput, UiLayout, UiUpdate, UiView};

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
pub use self::input::{Event, ModifiersState, MouseButton, TouchPhase};

mod font;
pub use self::font::{Font, FontSize, FontWeight};
