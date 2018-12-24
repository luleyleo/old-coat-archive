//! Provides `struct`s and `trait`s that are the same for any back end
//! as well as back end specific code.
//!
//! Independent of the use back end it should export:
//! - `struct Renderer`: This provides everything necessary to render anything
//! - `struct PrimitiveRenderer`: Provides all functions of the `PrimitiveRendererTrait`

use crate::{Bounds, Component};

mod webrender;
mod winit;

pub use self::webrender::Renderer;

use crate::Rectangle;
trait PrimitiveRendererTrait {
    fn rectangle(state: &<Rectangle as Component>::State, bounds: Bounds, renderer: &mut Renderer);
}

use self::webrender::PrimitiveRenderer as WrPrimitiveRenderer;
pub struct PrimitiveRenderer;
impl PrimitiveRenderer {
    pub fn rectangle(
        state: &<Rectangle as Component>::State,
        bounds: Bounds,
        renderer: &mut Renderer,
    ) {
        WrPrimitiveRenderer::rectangle(state, bounds, renderer);
    }
}

pub use self::winit::{AppEvent, AppProps, Window};
