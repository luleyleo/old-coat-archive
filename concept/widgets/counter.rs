struct Counter;

#[non_exhaustive]
struct Props {
    step: usize,
}

impl PropsBuilder<Counter> {
    pub fn step(mut self, value: usize) -> Self {
        self.step = value;
        self
    }
}

struct State {
    count: usize,
}

enum Msg {
    Increment,
    Decrement,
    Set(usize),
}

#[non_exhaustive]
enum Event {
    OverTen(usize),
}

type Ctx = ();

impl Component for Counter {
    type Props = Props;
    type State = State;
    type Msg = Msg;
    type Event = Event;
    type Ctx = Ctx;

    fn new<T: Component>() -> PropsBuilder<Self, T> {
        Props { step: 1 }.into()
    }

    fn init_state(props: Props) -> State {
        State { count: 0 }
    }

    fn update(msg: Msg, props: Props, state: &mut State, ctx: &Ctx) -> Option<Event> {
        match msg {
            Msg::Increment => state.count += props.step,
            Msg::Decrement => state.count -= props.step,
            Msg::Set(value) => state.count = value,
        }

        if state.count > 10 {
            Some(Event::OverTen(state.count))
        } else {
            None
        }
    }

    /**
     * Writing the ui using React style "rsx".
     * This should mostly expand to the "pure" variant.
     * When setting a property called `text` it will expand to
     * a builder call of its method `text` with the rhs as parameter.
     * Actions have to be annotated with `@` because they are part
     * of the `handle` builder function and not a function themselves.
     * To keep it pure there is a need for control flow components like `If`.
     * The problem with this is that I cannot think of a way to express loops like that.
     */
    #[cfg(xml)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UI) {
        xml! {
            <VBox>
                <Label text=format!("{}" state.count)/>
                <HBox spacing=10.0 even=true>
                    <Button label="+" @Action(_)=Msg::Increment/>
                    <If pred={state.count < 0}>
                        <Button label="C" @Action(_)=Msg::Set(0)/>
                    </If>
                    <Button label="-" @Action(_)=Msg::Decrement/>
                </HBox>
            </VBox>
        }
    }

    /**
     * Mostly the same as `xml` but allows a more React style by
     * mixing Rust and xml. So instead of requiring an `IF` component
     * you can just use a Rust `if` expression.
     * The problem is that I can not come up with an idea to make this work.
     * The `xml!` macro has to generate the ids and relationships of components
     * *statically* at compile time. This is not possible when there are two
     * nested `xml` macro calls.
     * One possible solution to id generation would be making them
     * dependent on the macros line using the `line!()` macro.
     * This way it would be only forbidden to use two macros in the
     * same line.
     * Then it would also be allowed to mix the different styles
     * in one `view` function, even so I doubt that makes any sense.
     * That aside it just looks ugly.
     */
    #[cfg(xml_mixed)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UI) {
        xml! {
            <VBox>
                <Label text=format!("{}" state.count)/>
                <HBox spacing=10.0>
                    <Button label="+" @Action(_)=Msg::Increment/>
                    { if state.count > 0 { xml! {
                        <Button label="C" @Action(_)=Msg::Set(0)/>
                    }}}
                    <Button label="-" @Action(_)=Msg::Decrement/>
                </HBox>
            </VBox>
        }
    }

    /**
     * Like the previous one, but with rust control flow build into the macro.
     * This allows the use of `if`, `loop`/`while`/`for` and `match`.
     * The problem I have with this is that it uses a fairly different
     * syntax to normal xml and in the **case** of match it is also different
     * to Rusts syntax by having `<case>` children for each branch.
     * The `<case>` tags are also not being highlighted.
     */
    #[cfg(xml_mixed_extended)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UI) {
        xml! {
            <VBox>
                <Label text=format!("{}" state.count)/>
                <HBox spacing=10.0 >
                    <Button label="+" @Action(_)=Msg::Increment/>
                    <if {state.count > 0}>
                        <Button label="C" @Action(_)=Msg::Set(0)/>
                    </if>
                    <Button label="-" @Action(_)=Msg::Decrement/>
                </HBox>
            </VBox>
        }
    }

    #[cfg(xml_mixed_extended_example)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UI) {
        xml! {
            <VBox>
                <Label text=format!("{}" state.count)/>
                <HBox spacing=10.0 >
                    <Button label="+" @Action(_)=Msg::Increment/>
                    <if {state.count > 0}>
                        <Button label="C" @Action(_)=Msg::Set(0)/>
                    </if>
                    <while {false}></while>
                    <for some in None></for>
                    <match x>
                        <case X::Y(z)>
                        </case>
                    </match>
                    <Button label="-" @Action(_)=Msg::Decrement/>
                </HBox>
            </VBox>
        }
    }

    /**
     * Inspired by qml, pretty straight forward.
     */
    #[cfg(qml)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UI) {
        qml! {
            VBox {
                Label {
                    text: format!("{}", state.count);
                }
                HBox {
                    spacing: 10.0;

                    Button {
                        label: "+";
                        @Action(_): Msg::Increment;
                    }

                    If {
                        pred: state.count > 0;

                        Button {
                            label: "C";
                            @Action(_): Msg::Set(0);
                        }
                    }

                    Button {
                        label: "-";
                        @Action(_): Msg::Decrement;
                    }
                }
            }
        }
    }

    /**
     * Like the previous one, but with rust control flow build into the macro.
     * This allows the use of `if`, `loop`/`while`/`for` and `match`.
     * In difference to `xml_mixed` this would feel just right and
     * would be rather straight forward to implement.
     */
    #[cfg(qml_mixed)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UI) {
        qml! {
            VBox {
                Label {
                    text: format!("{}", state.count);
                }
                HBox {
                    spacing: 10.0;

                    Button {
                        label: "+";
                        @Action(_): Msg::Increment;
                    }

                    if state.count > 0 {
                        Button {
                            label: "C";
                            @Action(_): Msg::Set(0);
                        }
                    }

                    while false {}
                    for thing in things {}
                    match x { .. }

                    Button {
                        label: "-";
                        @Action(_): Msg::Decrement;
                    }
                }
            }
        }
    }

    /**
     * This uses good old Rust builder pattern.
     * The Blocks are only for indentation and have zero impact once compiled.
     * The most annoying part is keeping track of the ids and calling
     * `parent` and `set` on *every* builder.
     * This would be roughly what the macros expand to.
     * A style close to this will be definitely supported.
     * One of the awesome things about this is that it comes with
     * the full power Rust has to offer.
     */
    #[cfg(pure)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut Ui) {
        VBox::new()
            .set(0, ui);
        {
            Label::new()
                .text(format!("{}", state.count))
                .parent(0)
                .set(1, ui);

            HBox::new()
                .spacing(10.0)
                .parent(1)
                .set(2, ui);
            {
                Button::new()
                    .label("+")
                    .handle(|t| match t {
                        Button::Event::Action(e) => Msg::Increment,
                        _ => (),
                    })
                    .parent(2)
                    .set(3, ui);

                if state.count > 0 {
                    Button::new()
                        .label("C")
                        .handle(|t| match t {
                            Button::Event::Action(e) => Msg::Set(0),
                            _ => (),
                        })
                        .parent(2)
                        .set(4, ui);
                }

                Button::new()
                    .label("-")
                    .handle(|t| match t {
                        Button::Event::Action(e) => Msg::Decrement,
                        _ => (),
                    })
                    .parent(2)
                    .set(5, ui);
            }
        }
    }

    /**
     * Same as before but with constants instead of raw numbers as ids.
     */
    #[cfg(pure_human)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut Ui) {
        const ROOT: usize = 0;
        const LABEL: usize = 1;
        const BOX: usize = 2;
        const PLUS: usize = 3;
        const CLEAR: usize = 4;
        const MINUS: usize = 5;

        VBox::new()
            .set(ROOT, ui);
        {
            Label::new()
                .text(format!("{}", state.count))
                .parent(ROOT)
                .set(LABEL, ui);

            HBox::new()
                .spacing(10.0)
                .parent(ROOT)
                .set(BOX, ui);
            {
                Button::new()
                    .label("+")
                    .handle(|t| match t {
                        Button::Event::Action(e) => Msg::Increment,
                        _ => (),
                    })
                    .parent(BOX)
                    .set(PLUS, ui);

                if state.count > 0 {
                    Button::new()
                        .label("C")
                        .handle(|t| match t {
                            Button::Event::Action(_) => Msg::Set(0),
                            _ => (),
                        })
                        .parent(BOX)
                        .set(CLEAR, ui);
                }

                Button::new()
                    .label("-")
                    .handle(|t| match t {
                        Button::Event::Action(e) => Msg::Decrement,
                        _ => (),
                    })
                    .parent(BOX)
                    .set(MINUS, ui);
            }
        }
    }

    /**
     * Some macro magic to automate declaring the id constants.
     */
    #[cfg(pure_human_macro)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut Ui) {
        ids!(ROOT, LABEL, BOX, PLUS, CLEAR, MINUS);

        VBox::new()
            .set(ROOT, ui);
        {
            Label::new()
                .text(format!("{}", state.count))
                .parent(ROOT)
                .set(LABEL, ui);

            HBox::new()
                .spacing(10.0)
                .parent(ROOT)
                .set(BOX, ui);
            {
                Button::new()
                    .label("+")
                    .handle(|t| match t {
                        Button::Event::Action(e) => Msg::Increment,
                        _ => (),
                    })
                    .parent(BOX)
                    .set(PLUS, ui);

                if state.count > 0 {
                    Button::new()
                        .label("C")
                        .handle(|t| match t {
                            Button::Event::Action(e) => Msg::Set(0),
                            _ => (),
                        })
                        .parent(BOX)
                        .set(CLEAR, ui);
                }

                Button::new()
                    .label("-")
                    .handle(|t| match t {
                        Button::Event::Action(e) => Msg::Decrement,
                        _ => (),
                    })
                    .parent(BOX)
                    .set(MINUS, ui);
            }
        }
    }

    /**
     * Some more macro magic.
     * Instead of calling `parent` on every builder to create a hierarchy
     * the `ids!` macro takes care of this.
     * The ids are declared in a nested fashion and the `child -> parent`
     * connections are defined before the components are set.
     * The disadvantage is that when using block indentation for visual
     * orientation we end up with two trees, for one the macro that actually
     * defines the hierarchy and then the visual indentation.
     * This could be a little bit awkward when those two get out of sync
     * but I doubt it would be much worse than for the previous ones.
     */
    #[cfg(pure_human_macro_extended)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut Ui) {
        ids!{ui,
            ROOT {
                LABEL,
                BOX {
                    PLUS,
                    CLEAR,
                    MINUS,
                }
            }
        }

        VBox::new()
            .set(ROOT, ui);
        {
            Label::new()
                .text(format!("{}", state.count))
                .set(LABEL, ui);

            HBox::new()
                .spacing(10.0)
                .set(BOX, ui);
            {
                Button::new()
                    .label("+")
                    .handle(|t| match t {
                        Button::Event::Action(e) => Msg::Increment,
                        _ => (),
                    })
                    .set(PLUS, ui);

                if state.count > 0 {
                    Button::new()
                        .label("C")
                        .handle(|t| match t {
                            Button::Event::Action(e) => Msg::Set(0),
                            _ => (),
                        })
                        .set(CLEAR, ui);
                }

                Button::new()
                    .label("-")
                    .handle(|t| match t {
                        Button::Event::Action(e) => Msg::Decrement,
                        _ => (),
                    })
                    .set(MINUS, ui);
            }
        }
    }

    #[cfg(pure_human_macro_extended_split)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut Ui) {
        ids!{ui,
            ROOT {
                LABEL,
            }
        }

        VBox::new()
            .set(ROOT, ui);
        {
            Label::new()
                .text(format!("{}", state.count))
                .set(LABEL, ui);

            buttons(ROOT, state, ui);
        }
    }
}

#[cfg(pure_human_macro_extended_split)]
fn buttons(parent: usize, state: &State, ui: &mut Ui) {
    ids! {ui,
        BOX {
            PLUS,
            CLEAR,
            MINUS,
        }
    }

    HBox::new()
        .spacing(10.0)
        .parent(parent)
        .set(BOX, ui);
    {
        Button::new()
            .label("+")
            .handle(|t| match t {
                Button::Event::Action(e) => Msg::Increment,
                _ => (),
            })
            .set(PLUS, ui);

        if state.count > 0 {
            Button::new()
                .label("C")
                .handle(|t| match t {
                    Button::Event::Action(e) => Msg::Set(0),
                    _ => (),
                })
                .set(CLEAR, ui);
        }

        Button::new()
            .label("-")
            .handle(|t| match t {
                Button::Event::Action(e) => Msg::Decrement,
                _ => (),
            })
            .set(MINUS, ui);
    }
}
