use crate::{
    Bounds, BoxConstraints, Cid, Color, Component, Mut, PropsBuilder, Renderer, Size, UiLayout,
    UiUpdate, UiView,
};

pub struct Rectangle;

#[derive(Clone, Copy, PartialEq)]
pub struct Props {
    color: Color,
}

impl PropsBuilder<Rectangle> {
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Component for Rectangle {
    type Props = Props;
    type State = Props;
    type Msg = ();
    type Event = ();

    fn new() -> PropsBuilder<Self> {
        PropsBuilder::new(Props {
            color: Color::default(),
        })
    }

    fn init_state(props: &Self::Props) -> Self::State {
        *props
    }

    fn update(_: Self::Msg, _: Mut<Self::State>, _: &mut UiUpdate) -> Option<Self::Event> {
        None
    }

    fn view(_: &Self::Props, _: &Self::State, _: &mut UiView<Self>) {}

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

        if let Some(width) = constraints.max_width {
            if let Some(height) = constraints.max_height {
                return Size {
                    w: width,
                    h: height,
                };
            }
        }

        Size::default()
    }

    fn render(state: &Self::State, bounds: Bounds, renderer: &mut Renderer) {
        use webrender::api::*;

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
