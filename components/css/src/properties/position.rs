#[derive(Clone)]
pub enum SpecifiedValue {
    Static,
    Absolute,
    Relative,
    Fixed,
    Sticky,
}
