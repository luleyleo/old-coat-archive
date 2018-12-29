use crate::*;

pub struct Linear;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Props {
    direction: Direction,
    spacing: Scalar,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            direction: Direction::Horizontal,
            spacing: 0.0,
        }
    }
}

impl PropsBuilder<Linear> {
    pub fn horizontal(mut self) -> Self {
        self.direction = Direction::Horizontal;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.direction = Direction::Vertical;
        self
    }

    pub fn spacing(mut self, spacing: Scalar) -> Self {
        self.spacing = spacing;
        self
    }
}

pub type State = Props;
pub type Msg = ();
pub type Event = ();

impl Component for Linear {
    type Props = Props;
    type State = State;
    type Msg = Msg;
    type Event = Event;

    fn init(props: &Props) -> State {
        *props
    }

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
        use self::Direction::*;
        let is_limited = match state.direction {
            Horizontal => constraints.max_width.is_some(),
            Vertical => constraints.max_height.is_some(),
        };

        let constraints = match state.direction {
            Horizontal => constraints.min_width(0.0),
            Vertical => constraints.min_height(0.0),
        };

        let mut length = 0.0;
        let mut thickness = 0.0;
        if !is_limited {
            for child in children {
                let size = ui.size(*child, constraints);
                let position = match state.direction {
                    Horizontal => Position::new(length, 0.0),
                    Vertical => Position::new(0.0, length),
                };
                ui.position(*child, position);
                let (distance, thickness2) = match state.direction {
                    Horizontal => (size.w, size.h),
                    Vertical => (size.h, size.w),
                };
                length += distance;
                if thickness2 > thickness {
                    thickness = thickness2;
                }
            }
        } else {
            let max_length = match state.direction {
                Horizontal => constraints.max_width.unwrap(),
                Vertical => constraints.max_height.unwrap(),
            };

            for child in children {
                let constraints = match state.direction {
                    Horizontal => constraints.max_width(max_length - length),
                    Vertical => constraints.max_height(max_length - length),
                };
                let size = ui.size(*child, constraints);

                let position = match state.direction {
                    Horizontal => Position::new(length, 0.0),
                    Vertical => Position::new(0.0, length),
                };
                ui.position(*child, position);

                let (distance, thickness2) = match state.direction {
                    Horizontal => (size.w, size.h),
                    Vertical => (size.h, size.w),
                };
                length += distance;

                if thickness2 > thickness {
                    thickness = thickness2;
                }

                if length >= max_length {
                    if length > max_length || child != children.last().unwrap() {
                        log::warn!(
                            "`Linear` layout {} has more children than it can fit",
                            ui.full_debug_name()
                        );
                    }
                    break;
                }
            }
        }

        match state.direction {
            Horizontal => Size::new(length, thickness),
            Vertical => Size::new(thickness, length),
        }
    }
}
