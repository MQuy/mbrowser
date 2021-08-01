#[derive(Clone)]
pub enum Attachment {
    Scroll,
    Fixed,
    Local,
}

#[derive(Clone)]
pub struct BackgroundAttachment {
    attachments: Vec<Attachment>,
}
