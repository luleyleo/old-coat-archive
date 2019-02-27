use crate::*;

/// A wrapper Component which provides functions to further constrain a Components size
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

impl Component for Constrained {
    type Props = Props;
    type State = State;
    type Msg = ();
    type Event = ();

    fn init(props: &Self::Props) -> Self::State {
        *props
    }

    fn derive_state(props: &Self::Props, state: &mut Self::State) {
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
            if children.is_empty() {
                return Size::zero();
            }
        }

        let mut constraints = constraints.min(Size::zero());

        if let Some(min_width) = state.min_width {
            constraints = constraints.min_width(min_width);
        }
        if let Some(min_height) = state.min_height {
            constraints = constraints.min_height(min_height);
        }
        if let Some(max_width) = state.max_width {
            if let Some(imposed_max_width) = constraints.max_width {
                if max_width < imposed_max_width {
                    constraints = constraints.max_width(max_width);
                } else {
                    log::warn!(
                        "Property ignored: `max_width` of `Constrained` layout {} is larger than the original constraint ({} > {})",
                        ui.full_debug_name(),
                        max_width, imposed_max_width
                    );
                }
            } else {
                constraints = constraints.max_width(max_width);
            }
        }
        if let Some(max_height) = state.max_height {
            if let Some(imposed_max_height) = constraints.max_height {
                if max_height < imposed_max_height {
                    constraints = constraints.max_height(max_height);
                } else {
                    log::warn!(
                        "Property ignored: `max_height` of `Constrained` layout {} is larger than the original constraint ({} > {})",
                        ui.full_debug_name(),
                        max_height, imposed_max_height
                    );
                }
            } else {
                constraints = constraints.max_height(max_height);
            }
        }

        log::trace!(
            "BoxConstraints for `Constrained` layout {} are {:?}",
            ui.full_debug_name(),
            constraints,
        );

        ui.size(children[0], constraints)
    }
}
