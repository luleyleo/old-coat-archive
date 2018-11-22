use crate::Size;

mod widget_data;
pub use self::widget_data::WidgetData;

mod widget_id;
pub use self::widget_id::WidgetId;

pub struct Ui {
    data: WidgetData,
    root: WidgetId,
    window_size: Size,
}
