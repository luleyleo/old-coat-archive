use crate::{Component, PropsBuilder, ViewArgs, UpdateArgs, Color, Renderer, Bounds};

pub struct Rectangle;

#[derive(Clone, Copy, PartialEq)]
pub struct Props {
    color: Color,
}

impl Component for Rectangle {
    type Props = Props;
    type State = Props;
    type Msg = ();
    type Event = ();

    fn new<T: Component>() -> PropsBuilder<Self, T> {
        PropsBuilder::new(Props {
            color: Color::default()
        })
    }

    fn init_state(props: &Self::Props) -> Self::State {
        *props
    }

    fn update(_: UpdateArgs<Self>) -> Option<Self::Event> { None }

    fn view(_: ViewArgs<Self>) {}

    fn render(state: &Self::State, bounds: Bounds, renderer: &mut Renderer) {
        use webrender::api::*;

        let position = bounds.position;
        let size = bounds.size;
        let color = state.color;

        let info = LayoutPrimitiveInfo::new(LayoutRect::new(
            LayoutPoint::new(position.x, position.y),
            LayoutSize::new(size.w, size.h)
        ));
        renderer.push_rect(&info, ColorF::new(color.r, color.g, color.b, color.a));
    }
}
