//! Provides `struct`s and `trait`s that are the same for any back end
//! as well as back end specific code.
//!
//! Independent of the back end it should export `Renderer`

use crate::{Bounds, Component};

mod webrender;
pub mod winit;

use crate::Rectangle;
trait PrimitiveRendererTrait {
    fn rectangle(state: &<Rectangle as Component>::State, bounds: Bounds, renderer: &mut Renderer);
}

/// Contains all data and functions required to render something with the chosen back end
pub type Renderer = self::webrender::Renderer;

type UsedPrimitiveRenderer = self::webrender::PrimitiveRenderer;

/// Provides the functionality of `PrimitiveRendererTrait` without using the trait
pub(crate) struct PrimitiveRenderer;
impl PrimitiveRenderer {
    pub fn rectangle(
        state: &<Rectangle as Component>::State,
        bounds: Bounds,
        renderer: &mut Renderer,
    ) {
        UsedPrimitiveRenderer::rectangle(state, bounds, renderer);
    }
}
