use Color;
use Theme;

pub struct MyCoolTheme {
    accent: Color
}

impl MyCoolTheme {
    pub fn new() -> Self {
        MyCoolTheme {
            accent: Color::rgb(0.5, 0.5, 0.8)
        }
    }

    pub fn build(self) -> Theme {
        let t = Theme::new();

        t.style(Button)
            .set(|style| {
                style.background = WHITE;
                style.foreground = black;
                style.border = GRAY;
                style.border_width = 1.0;
                style.radius = 3.0;
            });

        t.style(Button)
            .where(|state| state.hovered)
            .set(|style| style.background = self.accent);

        t.style(Button)
            .inside(Toolbar)
            .set(|style| style.border_width = 0.0)

        return t;
    }
}

pub trait Component: Sized {
    type Style: Sized + 'static;
    type State: Sized + 'static;
    type Msg: Sized + 'static;
    type Event: Sized + 'static;

    fn view(props: &Self, state: &Self::State, style: &Self::Style, ui: &mut UiView<Self>) {};

    ...
}
