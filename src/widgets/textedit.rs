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
        index: usize,
        text: String,
    },
    Deletion {
        range: Range,
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
                state.text = text;
            }
            TextEditMsg::Deletion { range } => {
                state.text.replace_range(range, "");
            }
        }
    }
}
