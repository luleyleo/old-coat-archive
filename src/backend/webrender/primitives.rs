use crate::backend::{PrimitiveRendererTrait};
use webrender::api::*;

use crate::{Component, Bounds, Rectangle, Renderer};

pub struct PrimitiveRenderer;
impl PrimitiveRendererTrait for PrimitiveRenderer {
    fn rectangle(state: &<Rectangle as Component>::State, bounds: Bounds, renderer: &mut Renderer) {
        let position = bounds.position;
        let size = bounds.size;
        let color = state.color;

        let info = LayoutPrimitiveInfo::new(LayoutRect::new(
            LayoutPoint::new(position.x, position.y),
            LayoutSize::new(size.w, size.h),
        ));
        renderer.push_rect(&info, ColorF::new(color.r, color.g, color.b, color.a));
    }
}
