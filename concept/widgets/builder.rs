use crate::*;

pub trait ComponentBuilder {
    type Comp: Component;

    /// I think "Component::init" is better than this
    fn build(self, builder: &mut UiBuilder<Comp>) -> Comp::State;

    fn set(self, ui: &mut UiView<Self>, id: Iid) -> ContentBuilder {
        ui.add(self, id)
    }
}

pub struct Button;

#[derive(Default)]
pub struct ButtonProps {
    label: String,
    enabled: bool,
}

impl ComponentBuilder for ButtonProps {
    type Comp = Button;

    fn build(self) -> Self::Comp::State {
        ButtonState {
            ...
        }
    }
}

impl Component for Button {
    type Props = ButtonProps;
    type State = ButtonState;
    type Msg = ButtonMsg;
    type Event = ButtonMsg;

    fn new() -> Self::Props {
        Self::Props::default()
    }

    fn view(props, state, ui) {
        ui.add(VBox::new().spacing(10.0), iid!()).with(|| {
            ui.add(Button::new().label("First"), iid!())
                .on(ui, event!(Button::Event::Activated(_) => Msg::SomeEvent));
            
            ui.add(Border::new(), iid!()).with(|| {
                ui.add(Padding::new().all(5.0), iid!()).with(|| {
                    ui.add(VBox::new().spacing(5.0), iid!()).with (|| {
                        for item in state.items {
                            ui.add(Button::new(), iid!().key(item.id))
                                .on(ui, event!(blah))
                                .with(|ui| {
                                    ui.add(Text::new()
                                        .content(item.name)
                                        .color(Color::rgb(..))
                                        .size(16.0), iid!());

                                    Text::new()
                                        .content(item.name)
                                        .color(Color::rgb(..))
                                        .size(16.0)
                                        .set(ui, iid!());
                                });
                        }
                    });
                });
            });
        });
    }
}

//////////////////////////////////////

pub trait Component {
    type Props: Default + ComponentBuilder + Sized;
    type State: Sized + 'static;
    type Msg: Sized + 'static;
    type Event: Sized + 'static;
}

