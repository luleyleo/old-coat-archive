use crate::*;

#[derive(Default, Clone, Copy)]
pub struct Offset {
    x: Scalar,
    y: Scalar,
}

impl Offset {
    pub fn x(mut self, x: Scalar) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: Scalar) -> Self {
        self.y = y;
        self
    }
}

impl Component for Offset {
    type State = Self;
    type Msg = ();
    type Event = ();

    fn init(props: &Self) -> Self {
        *props
    }

    fn derive_state(props: &Self, state: &mut Self, _ui: &UiDerive) {
        *state = *props;
    }

    fn layout(
        state: &Self,
        children: &[Cid],
        constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size {
        let constraints = constraints.tighten(Size::new(state.x, state.y));
        let mut largest = Size::zero();
        for child in children {
            let size = ui.size(*child, constraints);
            largest = largest.max(size);
            ui.position(*child, Position::new(state.x, state.y));
        }
        return largest;
    }
}
