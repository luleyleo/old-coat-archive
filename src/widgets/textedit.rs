use crate::*;
type Range = std::ops::Range<usize>;

pub type TextEdit<'a> = TextEditProps<'a>;

#[derive(Default)]
pub struct TextEditProps<'a> {
    text: &'a str,
}

pub struct TextEditState {
    text: String,
    cursor: Range,
}

pub enum TextEditEvent {
    Insertion {
        range: Range,
        text: String,
    },
    Deletion {
        range: Range,
    }
}

impl TextEditEvent {
    pub fn apply(self, target: &mut String) {
        use TextEditEvent::*;
        match self {
            Insertion { range, text } => {
                target.replace_range(range, &text);
            }
            Deletion { range } => {
                target.replace_range(range, "");
            }
        }
    }
}

pub enum TextEditMsg {
    Insertion {
        text: String
    },
    Deletion {
        range: Range
    },
    Selection {
        range: Range
    }
}

impl<'a> Properties for TextEditProps<'a> {
    type Component = TextEdit<'a>;
}

impl<'a> Component for TextEdit<'a> {
    type Props = TextEditProps<'a>;
    type State = TextEditState;
    type Msg = TextEditMsg;
    type Event = TextEditEvent;

    fn init(props: &Self::Props) -> Self::State {
        TextEditState {
            text: props.text.to_string(),
            cursor: 0..0,
        }
    }

    fn update(msg: Self::Msg, mut state: Mut<Self::State>, ui: &mut UiUpdate) {
        match msg {
            TextEditMsg::Selection { range } => {
                state.cursor = range;
            }
            TextEditMsg::Insertion { text } => {
                let cursor = state.cursor.clone();
                state.text.replace_range(cursor.clone(), &text);
                ui.emit(TextEditEvent::Insertion {
                    range: cursor,
                    text: text,
                });
            }
            TextEditMsg::Deletion { range } => {
                state.text.replace_range(range.clone(), "");
                ui.emit(TextEditEvent::Deletion { range });
            }
        }
    }

    fn view(props: &Self::Props, state: &Self::State, ui: &mut UiView<Self>) {
        TouchArea::new()
            .set(iid!(), ui);
    }

    fn render(state: &Self::State, bounds: Bounds, renderer: &mut Renderer) {
        // Render some text
    }
}
