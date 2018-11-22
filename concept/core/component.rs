struct UpdateContext<'a, C: Component> {
    msg: C::Msg,
    props: C::Props,
    state: &'a mut C::State,
    dispatcher: Sender<C::Msg>,
}

pub trait Component: Sized {
    type Props: Sized + 'static;
    type State: Sized + 'static;
    type Msg: Sized + 'static;
    type Event: Sized + 'static;

    fn new<T: Component>() -> PropsBuilder<Self, T>;

    fn init_state(props: Self::Props) -> Self::State;

    fn update(msg: Self::Msg, props: Self::Props, state: &mut Self::State) -> Option<Event>;

    fn view(props: Self::Props, state: &Self::State, ui: &mut UI);

    fn layout(constraints: BoxConstraints, children: &[WidgetId], ctx: &mut LayoutContext) -> Size {
        if children.is_empty() {
            Size::default()
        } else {
            assert_eq!(children.len(), 1);
            let child = children[0];
            ctx.size(child, constraints)
        }
    }
}

pub struct AComponent<
    Props: Sized + 'static,
    State: Sized + 'static,
    Msg: Sized + 'static,
    Event: Sized + 'static,
> {
    init_state: fn(props: Self::Props) -> Self::State,
    update: fn(msg: Self::Msg, props: Self::Props, state: &mut Self::State) -> Option<Event>,
    view: fn(props: Self::Props, state: &Self::State, ui: &mut UI),
    layout: fn(constraints: BoxConstraints, children: &[WidgetId], ctx: &mut LayoutContext) -> Size,
}
