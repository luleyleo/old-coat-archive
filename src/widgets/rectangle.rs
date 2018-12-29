use crate::{
    Bounds, BoxConstraints, Cid, Color, Component, PropsBuilder, Renderer, Size, UiLayout, UiView,
};

pub struct Rectangle;

#[derive(Clone, Copy, PartialEq)]
pub struct Props {
    pub(crate) color: Color,
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

    fn init(props: &Self::Props) -> Self::State {
        *props
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
        crate::backend::PrimitiveRenderer::rectangle(state, bounds, renderer);
    }
}
