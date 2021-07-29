#[derive(Clone)]
pub enum SpecifiedValue {
    Normal,
    Embed,
    Isolate,
    BidiOverride,
    IsolateOverride,
    Plaintext,
}
