#[derive(Clone)]
#[repr(u8)]
pub enum BorderStyle {
    Hidden,
    None,
    Inset,
    Groove,
    Outset,
    Ridge,
    Dotted,
    Dashed,
    Solid,
    Double,
}

#[derive(Clone)]
#[repr(C, u8)]
pub enum OutlineStyle {
    Auto,
    BorderStyle(BorderStyle),
}
