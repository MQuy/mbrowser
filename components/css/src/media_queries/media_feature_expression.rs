use crate::values::layout::Resolution;
use crate::values::length::Length;
use crate::values::percentage::Ratio;
use crate::values::CSSFloat;

/// The kind of matching that should be performed on a media feature value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Range {
    /// At least the specified value.
    Min,
    /// At most the specified value.
    Max,
}

/// The operator that was specified in this media feature.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operator {
    /// =
    Equal,
    /// >
    GreaterThan,
    /// >=
    GreaterThanEqual,
    /// <
    LessThan,
    /// <=
    LessThanEqual,
}

/// Either a `Range` or an `Operator`.
///
/// Ranged media features are not allowed with operations (that'd make no
/// sense).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RangeOrOperator {
    /// A `Range`.
    Range(Range),
    /// An `Operator`.
    Operator(Operator),
}

/// A feature expression contains a reference to the media feature, the value
/// the media query contained, and the range to evaluate.
#[derive(Clone, Debug, PartialEq)]
pub struct MediaFeatureExpression {
    feature_index: usize,
    value: Option<MediaExpressionValue>,
    range_or_operator: Option<RangeOrOperator>,
}

/// A value found or expected in a media expression.
///
/// FIXME(emilio): How should calc() serialize in the Number / Integer /
/// BoolInteger / NumberRatio case, as computed or as specified value?
///
/// If the first, this would need to store the relevant values.
///
/// See: https://github.com/w3c/csswg-drafts/issues/1968
#[derive(Clone, Debug, PartialEq)]
pub enum MediaExpressionValue {
    /// A length.
    Length(Length),
    /// A (non-negative) integer.
    Integer(u32),
    /// A floating point value.
    Float(CSSFloat),
    /// A boolean value, specified as an integer (i.e., either 0 or 1).
    BoolInteger(bool),
    /// A single non-negative number or two non-negative numbers separated by '/',
    /// with optional whitespace on either side of the '/'.
    NumberRatio(Ratio),
    /// A resolution.
    Resolution(Resolution),
    /// An enumerated value, defined by the variant keyword table in the
    /// feature's `mData` member.
    Enumerated(u8),
    /// An identifier.
    Ident(String),
}
