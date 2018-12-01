use crate::{Component, PropsBuilder, ViewArgs, UpdateArgs, Color};

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

    fn update(args: UpdateArgs<Self>) -> Option<Self::Event> { None }

    fn view(args: ViewArgs<Self>) {}
}
