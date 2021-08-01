use common::url::BrowserUrl;

use crate::values::number::Number;

#[derive(Clone)]
#[repr(u8)]
pub enum CursorKind {
    None,
    Default,
    Pointer,
    ContextMenu,
    Help,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
    AllScroll,
    ZoomIn,
    ZoomOut,
    Auto,
}

#[derive(Clone)]
#[repr(C)]
pub struct CursorImage {
    pub url: BrowserUrl,
    pub has_hotspot: bool,
    pub hotspot_x: Number,
    pub hotspot_y: Number,
}

#[derive(Clone)]
#[repr(C)]
pub struct Cursor {
    pub images: CursorImage,
    pub keyword: CursorKind,
}
