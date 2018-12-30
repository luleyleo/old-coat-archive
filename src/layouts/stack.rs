use crate::*;

pub struct Stack;

#[derive(Default, Clone, Copy)]
pub struct StackProps {
    //alignment: Alignment,
}

impl Component for Stack {
    type Props = StackProps;
    type State = StackProps;
    type Msg = ();
    type Event = ();

    fn init(props: &Self::Props) -> Self::State {
        *props
    }

    fn layout(
        _state: &Self::State,
        children: &[Cid],
        constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size {
        let constraints = constraints.min(Size::default());
        let mut largest = Size::default();

        for child in children.iter().cloned() {
            let Size { w, h } = ui.size(child, constraints);
            largest.w = w.max(largest.w);
            largest.h = h.max(largest.h);
        }

        for child in children.iter().cloned() {
            let size = ui.get_size(child);
            let x = (largest.w - size.w) / 2.0;
            let y = (largest.h - size.h) / 2.0;
            ui.position(child, Position::new(x, y));
        }

        largest
    }
}
