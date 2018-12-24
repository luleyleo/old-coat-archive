pub type Scalar = f32;

mod bounds;
pub use self::bounds::{Position, Size, Bounds};

mod builder;
pub use self::builder::{PropsBuilder, ReactivePropsBuilder, ContentBuilder};

mod component;
use self::component::ComponentPointer;
pub use self::component::Component;

mod constraints;
pub use self::constraints::BoxConstraints;

mod ui;
use self::ui::{UiData, UiRender, UiInputBase, full_debug_name_of};
pub use self::ui::{Cid, UiView, UiUpdate, UiLayout, UiInput};

mod mutable;
pub use self::mutable::Mut;

mod backend;
pub use self::backend::*;

mod layout;
pub use self::layout::{Linear, Constrained};

mod widget;
pub use self::widget::{Rectangle};

mod color;
pub use self::color::Color;

mod name;
pub use self::name::Named;

mod input;
pub use self::input::{Input, Event};
