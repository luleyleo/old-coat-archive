use crate::*;

pub struct Padding;

#[derive(Default, Clone, Copy, Debug)]
pub struct PaddingProps {
    top: Scalar,
    right: Scalar,
    bottom: Scalar,
    left: Scalar,
}

impl PropsBuilder<Padding> {
    pub fn all(self, value: Scalar) -> Self {
        self.top(value).right(value).bottom(value).left(value)
    }

    pub fn top(mut self, value: Scalar) -> Self {
        self.top = value;
        self
    }

    pub fn right(mut self, value: Scalar) -> Self {
        self.right = value;
        self
    }

    pub fn bottom(mut self, value: Scalar) -> Self {
        self.bottom = value;
        self
    }

    pub fn left(mut self, value: Scalar) -> Self {
        self.left = value;
        self
    }
}

impl Component for Padding {
    type Props = PaddingProps;
    type State = Self::Props;
    type Msg = ();
    type Event = ();

    fn init(props: &Self::Props) -> Self::State {
        *props
    }

    fn layout(
        state: &Self::State,
        children: &[Cid],
        constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size {
        if children.len() != 1 {
            log::error!(
                "`Padding` layout component {} must have exactly 1 child but it has {}",
                ui.full_debug_name(),
                children.len(),
            );
            if children.is_empty() {
                return Size::zero();
            }
        }

        let constraints = constraints.min(Size::zero());

        let width = constraints.max_width.map(|w| w - state.left - state.right);
        let height = constraints.max_height.map(|h| h - state.top - state.bottom);

        let mut constraints = constraints;
        constraints.max_width = width;
        constraints.max_height = height;

        log::trace!("Padding with {:?}", constraints);
        let size = ui.size(children[0], constraints);
        let position = Position::new(state.left, state.top);
        ui.position(children[0], position);

        let width = size.width + state.left + state.right;
        let height = size.height + state.top + state.bottom;

        Size::new(width, height)
    }
}
