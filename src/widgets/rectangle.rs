use crate::{Bounds, BoxConstraints, Cid, Color, Component, Renderer, Size, UiDerive, UiLayout};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub(crate) color: Color,
}

impl Rectangle {
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Component for Rectangle {
    type State = Rectangle;
    type Msg = ();
    type Event = ();

    fn init(props: &Self) -> Self::State {
        *props
    }

    fn derive_state(props: &Self, state: &mut Self::State, _ui: &mut UiDerive<Self>) {
        if props != state {
            *state = *props;
        }
    }

    fn layout(
        _state: &Self::State,
        children: &[Cid],
        constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size {
        if children.len() != 0 {
            let name = ui.full_debug_name();
            log::error!(
                "The primitive Component {} has content attached to it but it will be ignored",
                name
            );
        }

        Size::new(
            constraints.max_width.unwrap_or(constraints.min_width),
            constraints.max_height.unwrap_or(constraints.min_height),
        )
    }

    fn render(state: &Self::State, bounds: Bounds, renderer: &mut Renderer) {
        use webrender::api::*;

        let position = bounds.origin;
        let size = bounds.size;
        let color = state.color;

        let info = LayoutPrimitiveInfo::new(LayoutRect::new(
            LayoutPoint::new(position.x, position.y),
            LayoutSize::new(size.width, size.height),
        ));

        renderer
            .builder
            .push_rect(&info, ColorF::new(color.r, color.g, color.b, color.a));
    }
}
