
struct ThemeProvider;

#[derive(Clone, Copy, Default)]
struct Theme {
    accent_color: SUPER_NICE_BLUE
}

impl Component for ThemeProvider {
    type State = Theme;

    ...
}

struct Button {
    ...
}

struct ButtonStyle {
    background: Color,
    foreground: Color,
    border_color: Color,
    border_width: Color,
    corner_radius: Scalar,
}

impl ButtonStyle {
    pub fn new(theme: Theme, state: &Button::State) -> Self {
        ButtonStyle {
            background: if state.hovered {
                theme.accent_color
            } else {
                theme.control_background
            },

            foreground: if state.hovered {
                theme.accent_foreground_color
            } else {
                theme.label_color
            },

            border_color: theme.border_color,
            border_width: 1.0,
            corner_radius: theme.border_radius,
        }
    }
}

impl Component for Button {
    ...

    #[qml]
    fn view(props: &Self, state: &Self::State, ui: &mut UiView<Self>) {
        let theme: Theme = ui.context().unwrap_or_default();
        let style = ButtonStyle::new(theme, state);

        Stack {
            Ractangle {
                color: style.background;
                radius: style.corner_radius;
            }
            Border {
                color: style.border_color;
                width: style.border_width;
            }
            Padding {
                all: 5.0;
                Label {
                    text: props.label;
                    color: style.foreground;
                }
            }
            TouchArea {
                @Moved(e): ButtonMsg::Hover(e.is_inside);
                @Touched(e): {
                    if e.count == 1 {
                        Some(ButtonMsg::Touched)
                    }
                    None
                };
            }
        }
    }
}
