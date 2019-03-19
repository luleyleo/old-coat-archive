use coat::*;

//! This concept demonstrates possible macros / styles to define the ui

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
    step: usize,
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

impl Component for Counter {
    type Props = Props;
    type State = State;
    type Msg = Msg;
    type Event = Event;

    fn new() -> PropsBuilder<Self> {
        PropsBuilder::new(
            Props { step: 1 }
        )
    }

    fn init(props: &Props) -> State {
        State {
            step: props.step,
            count: 0,
        }
    }

    fn derive_state(props: &Props, state: Mut<State>) {
        if state.step != props.step {
            state.step = props.step;
        }
    }

    fn update(msg: Msg, mut state: Mut<State>, ui: &mut UiUpdate) {
        match msg {
            Msg::Increment => state.count += state.step,
            Msg::Decrement => state.count -= state.step,
            Msg::Set(value) => state.count = value,
        }

        if state.count > 10 {
            ui.dispatch(Event::OverTen(state.count));
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
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UiView) {
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
    #[cfg(xml_extended)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UiView) {
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

    #[cfg(xml_extended_example)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UiView) {
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
     * This allows the use of `if`, `loop`/`while`/`for` and `match`.
     * In difference to the xml syntax this would feel just right and
     * would be rather straight forward to implement.
     */
    #[cfg(qml)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UiView) {
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
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UiView) {
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
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UiView) {
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
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UiView) {
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
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UiView) {
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

    /**
     * Instead of using a `parent` function, you can call `add` after
     * `set`ing a `Component`. All Components that are set afterwards
     * are considered children of the last Component that `add` was called on.
     * The only problem is that `add` has to borrow `Ui` which means it would
     * be unavailable in the closure. The two solutions to this are:
     * - Make the `Ui` an argument to the closure passed to `add`
     * - Provide `add` with an `Rc<Cell<?>>` to tell the `Ui` about the indentation
     */
    #[cfg(pure_human_macro_indented)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UiView) {
        ids!(ROOT, LABEL, BOX, PLUS, CLEAR, MINUS);

        VBox::new()
            .set(ROOT, ui)
            .add(||{
                Label::new()
                    .text(format!("{}", state.count))
                    .set(LABEL, ui);

                HBox::new()
                    .spacing(10.0)
                    .set(BOX, ui)
                    .add(||{
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
                    });
            });
    }

    /**
     * This style makes it really easy to split the `view` function
     */
    #[cfg(pure_human_macro_indented_split)]
    fn view(props: Props, state: &State, ctx: &Ctx, ui: &mut UiView) {
        ids!(ROOT, LABEL, BOX);

        VBox::new()
            .set(ROOT, ui)
            .add(||{
                Label::new()
                    .text(format!("{}", state.count))
                    .set(LABEL, ui);

                HBox::new()
                    .spacing(10.0)
                    .set(BOX, ui)
                    .add(|| buttons(state, ui));
            });
    }

    fn view(props: &Props, state: &State, ui: &mut UiView) {
        ui.add(VBox::new().spacing(10.0))
            .add(Label::new().text(format!("{}", state.count)))
            .and(HBox::new().spacing(10.0))
                .add(Button::new().label("+").clicked(|| Msg::Increment))
                .and(Button::new().label("C").clicked(|| Msg::Set(0)))
                .and(Button::new().label("+").clicked(|| Msg::Decrement));
        
         ui.add(VBox::new().spacing(10.0)).id(iid!())
            .add(Label::new().text(format!("{}", state.count)))
            .and(HBox::new().spacing(10.0))
                .add(Button::new().label("+"))
                    .on(|e| match e {
                        ButtonEvent::Action(_) => Some(Msg::Increment),
                        _ => None,
                    })
                .and(Button::new().label("C"))
                    .on(just!(ButtonEvent::Action(_) => Msg::Set(0)))
                .and(Button::new().label("+"))
                    .on(event!(ButtonEvent::Action(_) => Msg::Decrement));

        ui.add(TabView::new().tabs(&["Title 1", "Title 2"]))
            .add(Content1::new())
            .add(Content2::new());
        
        ui.add(TabView::new()
                .tabs(state.tabs.iter().map(|t| t.1))
                .closeable(true)
                .moveable(true))
            .on(|e| match e {
                TabViewEvent::Close(index) => Some(Msg::TabClose(index)),
                TabViewEvent::Move(index, new_index) => Some(Msg::TabMove(index, new_index)),
                _ => None,
            })
            .where(|ui| {
                for (index, _, content) in state.tabs {
                    ui.add(ContentView::new().data(content))
                        .indexed(index)
                        .close();
                }
                // or
                ui.add_many();
                for (index, _, content) in state.tabs {
                    ui.and(ContentView::new().data(content))
                        .indexed(index);
                }
            });
    }

    fn view(props: &Props, state: &State, ui: &mut UiView) {
        ui.add(VBox::new().spacing(10.0)).with(|| {
            ui.add(Label::new().text(state.label));
            ui.add(HBox::new().spacing(10.0)).with(|| {
                ui.add(Button::new().label("+"))
                    .on(event!(ButtonEvent::Action(_) => Msg::Increment));
                ui.add(Button::new().label("C"))
                    .on(event!(ButtonEvent::Action(_) => Msg::Set(0)));
                ui.add(Button::new().label("-"))
                    .on(event!(ButtonEvent::Action(_) => Msg::Decrement));
            });
        });
    }
}

#[cfg(pure_human_macro_extended_split)]
fn buttons(state: &State, ui: &mut UiView) {
    ids!(PLUS, CLEAR, MINUS);

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
