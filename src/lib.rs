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

mod ui;
use self::ui::{full_debug_name_of, UiData, UiInputBase, UiRender};
pub use self::ui::{UiInput, UiLayout, UiUpdate, UiView};

mod mutable;
pub use self::mutable::Mut;

mod backend;
pub use self::backend::*;

mod layout;
pub use self::layout::{Constrained, Linear};

mod widget;
pub use self::widget::Rectangle;

mod color;
pub use self::color::Color;

mod input;
pub use self::input::{Event, Input};
