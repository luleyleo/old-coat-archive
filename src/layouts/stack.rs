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
        let constraints = constraints.min(Size::zero());
        let mut largest = Size::zero();

        for child in children.iter().cloned() {
            let size = ui.size(child, constraints);
            largest = largest.max(size);
        }

        for child in children.iter().cloned() {
            let size = ui.get_size(child);
            let x = (largest.width - size.width) / 2.0;
            let y = (largest.height - size.height) / 2.0;
            ui.position(child, Position::new(x, y));
        }

        largest
    }
}
