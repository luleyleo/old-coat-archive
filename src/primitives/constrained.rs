use crate::*;

pub struct Constrained;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Props {
    min_width: Option<Scalar>,
    min_height: Option<Scalar>,
    max_width: Option<Scalar>,
    max_height: Option<Scalar>,
}

impl PropsBuilder<Constrained> {
    pub fn min_width(mut self, min_width: Scalar) -> Self {
        self.min_width = Some(min_width);
        self
    }

    pub fn min_height(mut self, min_height: Scalar) -> Self {
        self.min_height = Some(min_height);
        self
    }

    pub fn max_width(mut self, max_width: Scalar) -> Self {
        self.max_width = Some(max_width);
        self
    }

    pub fn max_height(mut self, max_height: Scalar) -> Self {
        self.max_height = Some(max_height);
        self
    }
}

pub type State = Props;
pub type Msg = ();
pub type Event = ();

impl Component for Constrained {
    type Props = Props;
    type State = State;
    type Msg = Msg;
    type Event = Event;

    fn new() -> PropsBuilder<Self> {
        PropsBuilder::new(Props::default())
    }

    fn init_state(props: &Self::Props) -> Self::State {
        *props
    }

    fn view(_props: &Self::Props, _state: &Self::State, _ui: &mut UiView<Self>) {}

    fn derive_state(props: &Self::Props, mut state: Mut<Self::State>) {
        if *props != *state {
            *state = *props;
        }
    }

    fn layout(
        state: &Self::State,
        children: &[Cid],
        constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size {
        if children.len() != 1 {
            log::error!(
                "`Constrained` layout component {} must have exactly 1 child but it has {}",
                ui.full_debug_name(),
                children.len(),
            );
        }

        let mut constraints = constraints;

        if let Some(min_width) = state.min_width {
            constraints = constraints.min_width(min_width);
        }
        if let Some(min_height) = state.min_height {
            constraints = constraints.min_height(min_height);
        }
        if let Some(max_width) = state.max_width {
            constraints = constraints.max_width(max_width);
        }
        if let Some(max_height) = state.max_height {
            constraints = constraints.max_height(max_height);
        }

        log::trace!(
            "BoxConstraints for Constrained layout {} are {:?}",
            ui.full_debug_name(),
            constraints,
        );

        ui.size(children[0], constraints)
    }
}
