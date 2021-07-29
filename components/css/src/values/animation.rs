use crate::values::number::{Integer, Number};

#[derive(Clone)]
#[repr(u8)]
pub enum TimingKeyword {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    StepStart,
    StepEnd,
}

#[derive(Clone)]
#[repr(u8)]
pub enum StepPosition {
    JumpStart,
    JumpEnd,
    JumpNone,
    JumpBoth,
    Start,
    End,
}

#[derive(Clone)]
#[repr(u8, C)]
pub enum TimingFunction {
    Keyword(TimingKeyword),
    CubicBezier {
        x1: Number,
        y1: Number,
        x2: Number,
        y2: Number,
    },
    Steps(Integer, StepPosition),
}
