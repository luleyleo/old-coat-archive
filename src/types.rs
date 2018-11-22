use smallvec::SmallVec;

pub type Scalar = f32;
pub type MsgVec<Msg> = SmallVec<[Msg; 3]>;
