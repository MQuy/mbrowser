/// Whether a media feature allows ranges or not.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum AllowsRanges {
    Yes,
    No,
}

pub enum Evaluator {
    Length,
    Integer,
    Float,
    BoolInteger,
    /// A non-negative number ratio, such as the one from device-pixel-ratio.
    NumberRatio,
    /// A resolution.
    Resolution,
    /// A keyword value.
    Orientation,
    DisplayMode,
    Scan,
    PrefersReducedMotion,
    PrefersContrast,
    ForcedColors,
    OverflowBlock,
    OverflowInline,
    PrefersColorScheme,
    Pointer,
    AnyPointer,
    Hover,
    AnyHover,
    /// An identifier.
    Ident,
}

pub enum Orientation {
    Portrait,
    Landscape,
}

pub enum Scan {
    Interlace,
    Progressive,
}

pub enum PrefersReducedMotion {
    NoPreference,
    Reduce,
}

pub enum PrefersContrast {
    NoPreference,
    Less,
    More,
    Custom,
}

pub enum ForcedColors {
    None,
    Active,
}

pub enum OverflowBlock {
    None,
    Scroll,
    Paged,
}

pub enum OverflowInline {
    None,
    Scroll,
}

pub enum PrefersColorScheme {
    Light,
    Dark,
}

pub enum Pointer {
    None,
    Coarse,
    Fine,
}

pub enum Hover {
    None,
    Hover,
}

/// A description of a media feature.
pub struct MediaFeatureDescription {
    /// The media feature name, in ascii lowercase.
    pub name: &'static str,
    /// Whether min- / max- prefixes are allowed or not.
    pub allows_ranges: AllowsRanges,
    /// The evaluator, which we also use to determine which kind of value to
    /// parse.
    pub evaluator: Evaluator,
}

/// A simple helper to construct a `MediaFeatureDescription`.
macro_rules! feature {
    ($name:expr, $allows_ranges:expr, $evaluator: expr,) => {
        MediaFeatureDescription {
            name: $name,
            allows_ranges: $allows_ranges,
            evaluator: $evaluator,
        }
    };
}

/// Adding new media features requires (1) adding the new feature to this
/// array, with appropriate entries (and potentially any new code needed
/// to support new types in these entries and (2) ensuring that either
/// nsPresContext::MediaFeatureValuesChanged is called when the value that
/// would be returned by the evaluator function could change.
pub static MEDIA_FEATURES: [MediaFeatureDescription; 24] = [
    feature!("width", AllowsRanges::Yes, Evaluator::Length,),
    feature!("height", AllowsRanges::Yes, Evaluator::Length,),
    feature!("aspect-ratio", AllowsRanges::Yes, Evaluator::NumberRatio,),
    feature!("orientation", AllowsRanges::No, Evaluator::Orientation,),
    feature!("device-width", AllowsRanges::Yes, Evaluator::Length,),
    feature!("device-height", AllowsRanges::Yes, Evaluator::Length,),
    feature!(
        "device-aspect-ratio",
        AllowsRanges::Yes,
        Evaluator::NumberRatio,
    ),
    feature!("resolution", AllowsRanges::Yes, Evaluator::Resolution,),
    feature!("display-mode", AllowsRanges::No, Evaluator::DisplayMode,),
    feature!("grid", AllowsRanges::No, Evaluator::BoolInteger,),
    feature!("scan", AllowsRanges::No, Evaluator::Scan,),
    feature!("color", AllowsRanges::Yes, Evaluator::Integer,),
    feature!("color-index", AllowsRanges::Yes, Evaluator::Integer,),
    feature!("monochrome", AllowsRanges::Yes, Evaluator::Integer,),
    feature!(
        "prefers-reduced-motion",
        AllowsRanges::No,
        Evaluator::PrefersReducedMotion,
    ),
    feature!(
        "prefers-contrast",
        AllowsRanges::No,
        Evaluator::PrefersContrast,
    ),
    feature!("forced-colors", AllowsRanges::No, Evaluator::ForcedColors,),
    feature!("overflow-block", AllowsRanges::No, Evaluator::OverflowBlock,),
    feature!(
        "overflow-inline",
        AllowsRanges::No,
        Evaluator::OverflowInline,
    ),
    feature!(
        "prefers-color-scheme",
        AllowsRanges::No,
        Evaluator::PrefersColorScheme,
    ),
    feature!("pointer", AllowsRanges::No, Evaluator::Pointer,),
    feature!("any-pointer", AllowsRanges::No, Evaluator::Pointer,),
    feature!("hover", AllowsRanges::No, Evaluator::Hover,),
    feature!("any-hover", AllowsRanges::No, Evaluator::Hover,),
];
