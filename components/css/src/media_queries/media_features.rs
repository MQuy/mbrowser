use std::fmt::Write;

use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};

use crate::css_writer::{CssWriter, ToCss};
use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;

/// Whether a media feature allows ranges or not.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum AllowsRanges {
	Yes,
	No,
}

#[derive(Clone, Debug, PartialEq)]
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
	Enumerated(Enumerated),
	/// An identifier.
	Ident,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Enumerated {
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
}

#[derive(Clone, Debug, PartialEq)]
pub enum Orientation {
	Portrait,
	Landscape,
}

#[inline]
fn unexpected_media_feature_value<'i, 't, T>(
	input: &mut Parser<'i, 't>,
) -> Result<T, ParseError<'i>> {
	Err(input.new_custom_error(StyleParseErrorKind::MediaQueryExpectedFeatureValue))
}

macro_rules! enumerated_impl {
    ( $input:tt,
        $($name:path, $value:expr),+,
        $(,)?
    ) => {
        impl $input {
            pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
                let token = input.next()?;
                match token {
                    Token::Ident(ident) => {
                        match_ignore_ascii_case! { ident,
                            $($value => Ok($name),)+
                            _ => unexpected_media_feature_value(input),
                        }
                    },
                    _ => unexpected_media_feature_value(input),
                }
            }
        }

        impl ToCss for $input {
            fn to_css<W>(&self, dest: &mut CssWriter<W>) -> core::fmt::Result
            where
                W: std::fmt::Write,
            {
                dest.write_str(match self {
                    $($name => $value,)+
                })
            }
        }
    };
}

enumerated_impl! { Orientation,
	Orientation::Portrait, "portrait",
	Orientation::Landscape, "landscape",
}

#[derive(Clone, Debug, PartialEq)]
pub enum DisplayMode {
	Fullscreen,
	Standalone,
	MinimalUI,
	Browser,
}

enumerated_impl! { DisplayMode,
	DisplayMode::Fullscreen, "fullscreen",
	DisplayMode::Standalone, "standalone",
	DisplayMode::MinimalUI, "minimal-ui",
	DisplayMode::Browser, "browser",
}

#[derive(Clone, Debug, PartialEq)]
pub enum Scan {
	Interlace,
	Progressive,
}

enumerated_impl! { Scan,
	Scan::Interlace, "interlace",
	Scan::Progressive, "progressive",
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrefersReducedMotion {
	NoPreference,
	Reduce,
}

enumerated_impl! { PrefersReducedMotion,
	PrefersReducedMotion::NoPreference, "no-preference",
	PrefersReducedMotion::Reduce, "reduce",
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrefersContrast {
	NoPreference,
	Less,
	More,
	Custom,
}

enumerated_impl! { PrefersContrast,
	PrefersContrast::NoPreference, "no-preference",
	PrefersContrast::Less, "less",
	PrefersContrast::More, "more",
	PrefersContrast::Custom, "custom",
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForcedColors {
	None,
	Active,
}

enumerated_impl! { ForcedColors,
	ForcedColors::None, "none",
	ForcedColors::Active, "active",
}

#[derive(Clone, Debug, PartialEq)]
pub enum OverflowBlock {
	None,
	Scroll,
	Paged,
}

enumerated_impl! { OverflowBlock,
	OverflowBlock::None, "none",
	OverflowBlock::Scroll, "scroll",
	OverflowBlock::Paged, "paged",
}

#[derive(Clone, Debug, PartialEq)]
pub enum OverflowInline {
	None,
	Scroll,
}

enumerated_impl! { OverflowInline,
	OverflowInline::None, "none",
	OverflowInline::Scroll, "scroll",
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrefersColorScheme {
	Light,
	Dark,
}

enumerated_impl! { PrefersColorScheme,
	PrefersColorScheme::Light, "light",
	PrefersColorScheme::Dark, "dark",
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pointer {
	None,
	Coarse,
	Fine,
}

enumerated_impl! { Pointer,
	Pointer::None, "none",
	Pointer::Coarse, "coarse",
	Pointer::Fine, "fine",
}

#[derive(Clone, Debug, PartialEq)]
pub enum Hover {
	None,
	Hover,
}

enumerated_impl! { Hover,
	Hover::None, "none",
	Hover::Hover, "hover",
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
	feature!(
		"orientation",
		AllowsRanges::No,
		Evaluator::Enumerated(Enumerated::Orientation),
	),
	feature!("device-width", AllowsRanges::Yes, Evaluator::Length,),
	feature!("device-height", AllowsRanges::Yes, Evaluator::Length,),
	feature!(
		"device-aspect-ratio",
		AllowsRanges::Yes,
		Evaluator::NumberRatio,
	),
	feature!("resolution", AllowsRanges::Yes, Evaluator::Resolution,),
	feature!(
		"display-mode",
		AllowsRanges::No,
		Evaluator::Enumerated(Enumerated::DisplayMode),
	),
	feature!("grid", AllowsRanges::No, Evaluator::BoolInteger,),
	feature!(
		"scan",
		AllowsRanges::No,
		Evaluator::Enumerated(Enumerated::Scan),
	),
	feature!("color", AllowsRanges::Yes, Evaluator::Integer,),
	feature!("color-index", AllowsRanges::Yes, Evaluator::Integer,),
	feature!("monochrome", AllowsRanges::Yes, Evaluator::Integer,),
	feature!(
		"prefers-reduced-motion",
		AllowsRanges::No,
		Evaluator::Enumerated(Enumerated::PrefersReducedMotion),
	),
	feature!(
		"prefers-contrast",
		AllowsRanges::No,
		Evaluator::Enumerated(Enumerated::PrefersContrast),
	),
	feature!(
		"forced-colors",
		AllowsRanges::No,
		Evaluator::Enumerated(Enumerated::ForcedColors),
	),
	feature!(
		"overflow-block",
		AllowsRanges::No,
		Evaluator::Enumerated(Enumerated::OverflowBlock),
	),
	feature!(
		"overflow-inline",
		AllowsRanges::No,
		Evaluator::Enumerated(Enumerated::OverflowInline),
	),
	feature!(
		"prefers-color-scheme",
		AllowsRanges::No,
		Evaluator::Enumerated(Enumerated::PrefersColorScheme),
	),
	feature!(
		"pointer",
		AllowsRanges::No,
		Evaluator::Enumerated(Enumerated::Pointer),
	),
	feature!(
		"any-pointer",
		AllowsRanges::No,
		Evaluator::Enumerated(Enumerated::Pointer),
	),
	feature!(
		"hover",
		AllowsRanges::No,
		Evaluator::Enumerated(Enumerated::Hover),
	),
	feature!(
		"any-hover",
		AllowsRanges::No,
		Evaluator::Enumerated(Enumerated::Hover),
	),
];
