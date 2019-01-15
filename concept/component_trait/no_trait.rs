struct Props {
    step: usize,
}

struct State {
    count: usize,
}

enum Msg {
    Add(usize),
    Reset,
}

enum Event {
    EvenCount(usize),
}

fn counter() -> Builder</* Generics? */> {
    Builder::new()
        .props(Props { step: 1 })
        .state(State { count: 0 })
        .update(|msg, state, ui| match msg {
            Msg::Add(step) => {
                state.count += step;
                if state.count % 2 == 0 {
                    ui.fire(Event::EvenCount(state.count));
                }
            }
            Msg::Reset => state.count = 0,
        })
        .view(|props, state, ui| {
            linear().vertical().spacing(5.0).set(id!(), ui).add(|| {
                label().content(fromat!("{}", state.count)).set(id!(), ui);

                linear().horizontal().spacing(5.0).set(id!(), ui).add(|| {
                    button()
                        .label("+")
                        .on_action(|_| Msg::Add(props.step))
                        .set(id!(Increment), ui);
                    button()
                        .label("-")
                        .on_action(|_| Msg::Add(-props.step))
                        .set(id!(Decrement), ui);
                });

                button()
                    .label("0")
                    .on_action(|_| Msg::Reset)
                    .set(id!(Reset), ui);
            });
        })
}
