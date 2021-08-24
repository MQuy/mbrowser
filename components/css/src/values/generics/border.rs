use cssparser::ToCss;

use crate::values::length::Pair;

#[derive(Clone)]
#[repr(C)]
pub struct GenericBorderCornerRadius<L>(pub Pair<L>);

impl<L: ToCss> ToCss for GenericBorderCornerRadius<L> {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        self.0.to_css(dest)
    }
}
