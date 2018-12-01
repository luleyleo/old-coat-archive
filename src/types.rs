use smallvec::SmallVec;
use webrender::api::DisplayListBuilder;

pub type Scalar = f32;
pub type MsgVec<Msg> = SmallVec<[Msg; 3]>;
pub type Renderer = DisplayListBuilder;
