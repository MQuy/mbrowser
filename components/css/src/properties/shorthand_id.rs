use core::fmt;
use std::fmt::Write;

use crate::css_writer::CssWriter;
use crate::declaration::PropertyFlags;
use crate::declaration_block::{DeclarationBlock, SourcePropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use cssparser::{ParseError, Parser, ToCss};

use crate::declaration::Declaration;

use super::longhand_id::LonghandId;
use super::property_id::{NonCustomPropertyId, NonCustomPropertyIterator};
use super::shorthands;

/// An identifier for a given shorthand property.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
pub enum ShorthandId {
    /// background
    Background = 0,
    /// background-position
    BackgroundPosition = 1,
    /// border-color
    BorderColor = 2,
    /// border-style
    BorderStyle = 3,
    /// border-width
    BorderWidth = 4,
    /// border-top
    BorderTop = 5,
    /// border-right
    BorderRight = 6,
    /// border-bottom
    BorderBottom = 7,
    /// border-left
    BorderLeft = 8,
    /// border-block-start
    BorderBlockStart = 9,
    /// border-block-end
    BorderBlockEnd = 10,
    /// border-inline-start
    BorderInlineStart = 11,
    /// border-inline-end
    BorderInlineEnd = 12,
    /// border
    Border = 13,
    /// border-radius
    BorderRadius = 14,
    /// border-image
    BorderImage = 15,
    /// border-block-width
    BorderBlockWidth = 16,
    /// border-block-style
    BorderBlockStyle = 17,
    /// border-block-color
    BorderBlockColor = 18,
    /// border-inline-width
    BorderInlineWidth = 19,
    /// border-inline-style
    BorderInlineStyle = 20,
    /// border-inline-color
    BorderInlineColor = 21,
    /// border-block
    BorderBlock = 22,
    /// border-inline
    BorderInline = 23,
    /// overflow
    Overflow = 24,
    /// transition
    Transition = 25,
    /// animation
    Animation = 26,
    /// columns
    Columns = 27,
    /// font
    Font = 28,
    /// font-variant
    FontVariant = 29,
    /// list-style
    ListStyle = 30,
    /// margin
    Margin = 31,
    /// margin-block
    MarginBlock = 32,
    /// margin-inline
    MarginInline = 33,
    /// outline
    Outline = 34,
    /// padding
    Padding = 35,
    /// padding-block
    PaddingBlock = 36,
    /// padding-inline
    PaddingInline = 37,
    /// flex-flow
    FlexFlow = 38,
    /// flex
    Flex = 39,
    /// inset
    Inset = 40,
    /// inset-block
    InsetBlock = 41,
    /// inset-inline
    InsetInline = 42,
    /// text-decoration
    TextDecoration = 43,
    /// all
    All = 44,
}

impl ToCss for ShorthandId {
    #[inline]
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result
    where
        W: Write,
    {
        dest.write_str(self.name())
    }
}

impl ShorthandId {
    /// Get the name for this shorthand property.
    #[inline]
    pub fn name(&self) -> &'static str {
        NonCustomPropertyId::from(*self).name()
    }

    /// Converts from a ShorthandId to an adequate nsCSSPropertyID.
    #[cfg(feature = "gecko")]
    #[inline]
    pub fn to_nscsspropertyid(self) -> nsCSSPropertyID {
        NonCustomPropertyId::from(self).to_nscsspropertyid()
    }

    /// Converts from a nsCSSPropertyID to a ShorthandId.
    #[cfg(feature = "gecko")]
    #[inline]
    pub fn from_nscsspropertyid(prop: nsCSSPropertyID) -> Result<Self, ()> {
        PropertyId::from_nscsspropertyid(prop)?
            .as_shorthand()
            .map_err(|_| ())
    }

    /// Get the longhand ids that form this shorthand.
    pub fn longhands(&self) -> NonCustomPropertyIterator<LonghandId> {
        static BACKGROUND: &'static [LonghandId] = &[
            LonghandId::BackgroundColor,
            LonghandId::BackgroundPositionX,
            LonghandId::BackgroundPositionY,
            LonghandId::BackgroundRepeat,
            LonghandId::BackgroundAttachment,
            LonghandId::BackgroundImage,
            LonghandId::BackgroundSize,
            LonghandId::BackgroundOrigin,
            LonghandId::BackgroundClip,
        ];
        static BACKGROUND_POSITION: &'static [LonghandId] = &[
            LonghandId::BackgroundPositionX,
            LonghandId::BackgroundPositionY,
        ];
        static BORDER_COLOR: &'static [LonghandId] = &[
            LonghandId::BorderTopColor,
            LonghandId::BorderRightColor,
            LonghandId::BorderBottomColor,
            LonghandId::BorderLeftColor,
        ];
        static BORDER_STYLE: &'static [LonghandId] = &[
            LonghandId::BorderTopStyle,
            LonghandId::BorderRightStyle,
            LonghandId::BorderBottomStyle,
            LonghandId::BorderLeftStyle,
        ];
        static BORDER_WIDTH: &'static [LonghandId] = &[
            LonghandId::BorderTopWidth,
            LonghandId::BorderRightWidth,
            LonghandId::BorderBottomWidth,
            LonghandId::BorderLeftWidth,
        ];
        static BORDER_TOP: &'static [LonghandId] = &[
            LonghandId::BorderTopColor,
            LonghandId::BorderTopStyle,
            LonghandId::BorderTopWidth,
        ];
        static BORDER_RIGHT: &'static [LonghandId] = &[
            LonghandId::BorderRightColor,
            LonghandId::BorderRightStyle,
            LonghandId::BorderRightWidth,
        ];
        static BORDER_BOTTOM: &'static [LonghandId] = &[
            LonghandId::BorderBottomColor,
            LonghandId::BorderBottomStyle,
            LonghandId::BorderBottomWidth,
        ];
        static BORDER_LEFT: &'static [LonghandId] = &[
            LonghandId::BorderLeftColor,
            LonghandId::BorderLeftStyle,
            LonghandId::BorderLeftWidth,
        ];
        static BORDER_BLOCK_START: &'static [LonghandId] = &[
            LonghandId::BorderBlockStartColor,
            LonghandId::BorderBlockStartStyle,
            LonghandId::BorderBlockStartWidth,
        ];
        static BORDER_BLOCK_END: &'static [LonghandId] = &[
            LonghandId::BorderBlockEndColor,
            LonghandId::BorderBlockEndStyle,
            LonghandId::BorderBlockEndWidth,
        ];
        static BORDER_INLINE_START: &'static [LonghandId] = &[
            LonghandId::BorderInlineStartColor,
            LonghandId::BorderInlineStartStyle,
            LonghandId::BorderInlineStartWidth,
        ];
        static BORDER_INLINE_END: &'static [LonghandId] = &[
            LonghandId::BorderInlineEndColor,
            LonghandId::BorderInlineEndStyle,
            LonghandId::BorderInlineEndWidth,
        ];
        static BORDER: &'static [LonghandId] = &[
            LonghandId::BorderTopColor,
            LonghandId::BorderTopStyle,
            LonghandId::BorderTopWidth,
            LonghandId::BorderRightColor,
            LonghandId::BorderRightStyle,
            LonghandId::BorderRightWidth,
            LonghandId::BorderBottomColor,
            LonghandId::BorderBottomStyle,
            LonghandId::BorderBottomWidth,
            LonghandId::BorderLeftColor,
            LonghandId::BorderLeftStyle,
            LonghandId::BorderLeftWidth,
            LonghandId::BorderImageOutset,
            LonghandId::BorderImageRepeat,
            LonghandId::BorderImageSlice,
            LonghandId::BorderImageSource,
            LonghandId::BorderImageWidth,
        ];
        static BORDER_RADIUS: &'static [LonghandId] = &[
            LonghandId::BorderTopLeftRadius,
            LonghandId::BorderTopRightRadius,
            LonghandId::BorderBottomRightRadius,
            LonghandId::BorderBottomLeftRadius,
        ];
        static BORDER_IMAGE: &'static [LonghandId] = &[
            LonghandId::BorderImageOutset,
            LonghandId::BorderImageRepeat,
            LonghandId::BorderImageSlice,
            LonghandId::BorderImageSource,
            LonghandId::BorderImageWidth,
        ];
        static BORDER_BLOCK_WIDTH: &'static [LonghandId] = &[
            LonghandId::BorderBlockStartWidth,
            LonghandId::BorderBlockEndWidth,
        ];
        static BORDER_BLOCK_STYLE: &'static [LonghandId] = &[
            LonghandId::BorderBlockStartStyle,
            LonghandId::BorderBlockEndStyle,
        ];
        static BORDER_BLOCK_COLOR: &'static [LonghandId] = &[
            LonghandId::BorderBlockStartColor,
            LonghandId::BorderBlockEndColor,
        ];
        static BORDER_INLINE_WIDTH: &'static [LonghandId] = &[
            LonghandId::BorderInlineStartWidth,
            LonghandId::BorderInlineEndWidth,
        ];
        static BORDER_INLINE_STYLE: &'static [LonghandId] = &[
            LonghandId::BorderInlineStartStyle,
            LonghandId::BorderInlineEndStyle,
        ];
        static BORDER_INLINE_COLOR: &'static [LonghandId] = &[
            LonghandId::BorderInlineStartColor,
            LonghandId::BorderInlineEndColor,
        ];
        static BORDER_BLOCK: &'static [LonghandId] = &[
            LonghandId::BorderBlockStartWidth,
            LonghandId::BorderBlockEndWidth,
            LonghandId::BorderBlockStartStyle,
            LonghandId::BorderBlockEndStyle,
            LonghandId::BorderBlockStartColor,
            LonghandId::BorderBlockEndColor,
        ];
        static BORDER_INLINE: &'static [LonghandId] = &[
            LonghandId::BorderInlineStartWidth,
            LonghandId::BorderInlineEndWidth,
            LonghandId::BorderInlineStartStyle,
            LonghandId::BorderInlineEndStyle,
            LonghandId::BorderInlineStartColor,
            LonghandId::BorderInlineEndColor,
        ];
        static OVERFLOW: &'static [LonghandId] = &[LonghandId::OverflowX, LonghandId::OverflowY];
        static TRANSITION: &'static [LonghandId] = &[
            LonghandId::TransitionProperty,
            LonghandId::TransitionDuration,
            LonghandId::TransitionTimingFunction,
            LonghandId::TransitionDelay,
        ];
        static ANIMATION: &'static [LonghandId] = &[
            LonghandId::AnimationName,
            LonghandId::AnimationDuration,
            LonghandId::AnimationTimingFunction,
            LonghandId::AnimationDelay,
            LonghandId::AnimationIterationCount,
            LonghandId::AnimationDirection,
            LonghandId::AnimationFillMode,
            LonghandId::AnimationPlayState,
        ];
        static COLUMNS: &'static [LonghandId] = &[LonghandId::ColumnWidth, LonghandId::ColumnCount];
        static FONT: &'static [LonghandId] = &[
            LonghandId::FontStyle,
            LonghandId::FontVariantCaps,
            LonghandId::FontWeight,
            LonghandId::FontStretch,
            LonghandId::FontSize,
            LonghandId::LineHeight,
            LonghandId::FontFamily,
        ];
        static FONT_VARIANT: &'static [LonghandId] = &[LonghandId::FontVariantCaps];
        static LIST_STYLE: &'static [LonghandId] = &[
            LonghandId::ListStylePosition,
            LonghandId::ListStyleImage,
            LonghandId::ListStyleType,
        ];
        static MARGIN: &'static [LonghandId] = &[
            LonghandId::MarginTop,
            LonghandId::MarginRight,
            LonghandId::MarginBottom,
            LonghandId::MarginLeft,
        ];
        static MARGIN_BLOCK: &'static [LonghandId] =
            &[LonghandId::MarginBlockStart, LonghandId::MarginBlockEnd];
        static MARGIN_INLINE: &'static [LonghandId] =
            &[LonghandId::MarginInlineStart, LonghandId::MarginInlineEnd];
        static OUTLINE: &'static [LonghandId] = &[
            LonghandId::OutlineColor,
            LonghandId::OutlineStyle,
            LonghandId::OutlineWidth,
        ];
        static PADDING: &'static [LonghandId] = &[
            LonghandId::PaddingTop,
            LonghandId::PaddingRight,
            LonghandId::PaddingBottom,
            LonghandId::PaddingLeft,
        ];
        static PADDING_BLOCK: &'static [LonghandId] =
            &[LonghandId::PaddingBlockStart, LonghandId::PaddingBlockEnd];
        static PADDING_INLINE: &'static [LonghandId] =
            &[LonghandId::PaddingInlineStart, LonghandId::PaddingInlineEnd];
        static FLEX_FLOW: &'static [LonghandId] =
            &[LonghandId::FlexDirection, LonghandId::FlexWrap];
        static FLEX: &'static [LonghandId] = &[
            LonghandId::FlexGrow,
            LonghandId::FlexShrink,
            LonghandId::FlexBasis,
        ];
        static INSET: &'static [LonghandId] = &[
            LonghandId::Top,
            LonghandId::Right,
            LonghandId::Bottom,
            LonghandId::Left,
        ];
        static INSET_BLOCK: &'static [LonghandId] =
            &[LonghandId::InsetBlockStart, LonghandId::InsetBlockEnd];
        static INSET_INLINE: &'static [LonghandId] =
            &[LonghandId::InsetInlineStart, LonghandId::InsetInlineEnd];
        static TEXT_DECORATION: &'static [LonghandId] = &[LonghandId::TextDecorationLine];
        static ALL: &'static [LonghandId] = &[
            LonghandId::BorderBlockStartColor,
            LonghandId::BorderBlockStartStyle,
            LonghandId::BorderBlockStartWidth,
            LonghandId::BorderBlockEndColor,
            LonghandId::BorderBlockEndStyle,
            LonghandId::BorderBlockEndWidth,
            LonghandId::BorderInlineStartColor,
            LonghandId::BorderInlineStartStyle,
            LonghandId::BorderInlineStartWidth,
            LonghandId::BorderInlineEndColor,
            LonghandId::BorderInlineEndStyle,
            LonghandId::BorderInlineEndWidth,
            LonghandId::BorderStartStartRadius,
            LonghandId::BorderStartEndRadius,
            LonghandId::BorderEndStartRadius,
            LonghandId::BorderEndEndRadius,
            LonghandId::OverflowInline,
            LonghandId::OverflowBlock,
            LonghandId::MarginBlockStart,
            LonghandId::MarginBlockEnd,
            LonghandId::MarginInlineStart,
            LonghandId::MarginInlineEnd,
            LonghandId::PaddingBlockStart,
            LonghandId::PaddingBlockEnd,
            LonghandId::PaddingInlineStart,
            LonghandId::PaddingInlineEnd,
            LonghandId::InsetBlockStart,
            LonghandId::InsetBlockEnd,
            LonghandId::InsetInlineStart,
            LonghandId::InsetInlineEnd,
            LonghandId::BlockSize,
            LonghandId::MinBlockSize,
            LonghandId::MaxBlockSize,
            LonghandId::InlineSize,
            LonghandId::MinInlineSize,
            LonghandId::MaxInlineSize,
            LonghandId::BackgroundColor,
            LonghandId::BackgroundImage,
            LonghandId::BackgroundPositionX,
            LonghandId::BackgroundPositionY,
            LonghandId::BackgroundRepeat,
            LonghandId::BackgroundAttachment,
            LonghandId::BackgroundClip,
            LonghandId::BackgroundOrigin,
            LonghandId::BackgroundSize,
            LonghandId::BorderTopColor,
            LonghandId::BorderTopStyle,
            LonghandId::BorderTopWidth,
            LonghandId::BorderRightColor,
            LonghandId::BorderRightStyle,
            LonghandId::BorderRightWidth,
            LonghandId::BorderBottomColor,
            LonghandId::BorderBottomStyle,
            LonghandId::BorderBottomWidth,
            LonghandId::BorderLeftColor,
            LonghandId::BorderLeftStyle,
            LonghandId::BorderLeftWidth,
            LonghandId::BorderTopLeftRadius,
            LonghandId::BorderTopRightRadius,
            LonghandId::BorderBottomRightRadius,
            LonghandId::BorderBottomLeftRadius,
            LonghandId::BorderImageSource,
            LonghandId::BorderImageOutset,
            LonghandId::BorderImageRepeat,
            LonghandId::BorderImageWidth,
            LonghandId::BorderImageSlice,
            LonghandId::Display,
            LonghandId::Position,
            LonghandId::Float,
            LonghandId::Clear,
            LonghandId::VerticalAlign,
            LonghandId::OverflowX,
            LonghandId::OverflowY,
            LonghandId::TransitionDuration,
            LonghandId::TransitionTimingFunction,
            LonghandId::TransitionProperty,
            LonghandId::TransitionDelay,
            LonghandId::AnimationName,
            LonghandId::AnimationDuration,
            LonghandId::AnimationTimingFunction,
            LonghandId::AnimationIterationCount,
            LonghandId::AnimationDirection,
            LonghandId::AnimationPlayState,
            LonghandId::AnimationFillMode,
            LonghandId::AnimationDelay,
            LonghandId::Transform,
            LonghandId::Rotate,
            LonghandId::Scale,
            LonghandId::Translate,
            LonghandId::Perspective,
            LonghandId::PerspectiveOrigin,
            LonghandId::BackfaceVisibility,
            LonghandId::TransformStyle,
            LonghandId::TransformOrigin,
            LonghandId::ColumnWidth,
            LonghandId::ColumnCount,
            LonghandId::Content,
            LonghandId::CounterIncrement,
            LonghandId::CounterReset,
            LonghandId::Opacity,
            LonghandId::BoxShadow,
            LonghandId::Clip,
            LonghandId::Filter,
            LonghandId::MixBlendMode,
            LonghandId::FontFamily,
            LonghandId::FontStyle,
            LonghandId::FontVariantCaps,
            LonghandId::FontWeight,
            LonghandId::FontSize,
            LonghandId::FontStretch,
            LonghandId::Visibility,
            LonghandId::WritingMode,
            LonghandId::ImageRendering,
            LonghandId::BorderCollapse,
            LonghandId::EmptyCells,
            LonghandId::CaptionSide,
            LonghandId::BorderSpacing,
            LonghandId::Color,
            LonghandId::LineHeight,
            LonghandId::TextTransform,
            LonghandId::TextIndent,
            LonghandId::OverflowWrap,
            LonghandId::WordBreak,
            LonghandId::TextJustify,
            LonghandId::TextAlign,
            LonghandId::LetterSpacing,
            LonghandId::WordSpacing,
            LonghandId::WhiteSpace,
            LonghandId::TextShadow,
            LonghandId::TextRendering,
            LonghandId::Cursor,
            LonghandId::PointerEvents,
            LonghandId::ListStylePosition,
            LonghandId::ListStyleType,
            LonghandId::ListStyleImage,
            LonghandId::Quotes,
            LonghandId::MarginTop,
            LonghandId::MarginRight,
            LonghandId::MarginBottom,
            LonghandId::MarginLeft,
            LonghandId::OutlineColor,
            LonghandId::OutlineStyle,
            LonghandId::OutlineWidth,
            LonghandId::OutlineOffset,
            LonghandId::PaddingTop,
            LonghandId::PaddingRight,
            LonghandId::PaddingBottom,
            LonghandId::PaddingLeft,
            LonghandId::Top,
            LonghandId::Right,
            LonghandId::Bottom,
            LonghandId::Left,
            LonghandId::ZIndex,
            LonghandId::FlexDirection,
            LonghandId::FlexWrap,
            LonghandId::JustifyContent,
            LonghandId::AlignContent,
            LonghandId::AlignItems,
            LonghandId::FlexGrow,
            LonghandId::FlexShrink,
            LonghandId::AlignSelf,
            LonghandId::Order,
            LonghandId::FlexBasis,
            LonghandId::Width,
            LonghandId::MinWidth,
            LonghandId::MaxWidth,
            LonghandId::Height,
            LonghandId::MinHeight,
            LonghandId::MaxHeight,
            LonghandId::BoxSizing,
            LonghandId::ColumnGap,
            LonghandId::AspectRatio,
            LonghandId::TableLayout,
            LonghandId::TextOverflow,
            LonghandId::TextDecorationLine,
        ];
        NonCustomPropertyIterator {
            filter: NonCustomPropertyId::from(*self).enabled_for_all_content(),
            iter: match *self {
                ShorthandId::Background => BACKGROUND,
                ShorthandId::BackgroundPosition => BACKGROUND_POSITION,
                ShorthandId::BorderColor => BORDER_COLOR,
                ShorthandId::BorderStyle => BORDER_STYLE,
                ShorthandId::BorderWidth => BORDER_WIDTH,
                ShorthandId::BorderTop => BORDER_TOP,
                ShorthandId::BorderRight => BORDER_RIGHT,
                ShorthandId::BorderBottom => BORDER_BOTTOM,
                ShorthandId::BorderLeft => BORDER_LEFT,
                ShorthandId::BorderBlockStart => BORDER_BLOCK_START,
                ShorthandId::BorderBlockEnd => BORDER_BLOCK_END,
                ShorthandId::BorderInlineStart => BORDER_INLINE_START,
                ShorthandId::BorderInlineEnd => BORDER_INLINE_END,
                ShorthandId::Border => BORDER,
                ShorthandId::BorderRadius => BORDER_RADIUS,
                ShorthandId::BorderImage => BORDER_IMAGE,
                ShorthandId::BorderBlockWidth => BORDER_BLOCK_WIDTH,
                ShorthandId::BorderBlockStyle => BORDER_BLOCK_STYLE,
                ShorthandId::BorderBlockColor => BORDER_BLOCK_COLOR,
                ShorthandId::BorderInlineWidth => BORDER_INLINE_WIDTH,
                ShorthandId::BorderInlineStyle => BORDER_INLINE_STYLE,
                ShorthandId::BorderInlineColor => BORDER_INLINE_COLOR,
                ShorthandId::BorderBlock => BORDER_BLOCK,
                ShorthandId::BorderInline => BORDER_INLINE,
                ShorthandId::Overflow => OVERFLOW,
                ShorthandId::Transition => TRANSITION,
                ShorthandId::Animation => ANIMATION,
                ShorthandId::Columns => COLUMNS,
                ShorthandId::Font => FONT,
                ShorthandId::FontVariant => FONT_VARIANT,
                ShorthandId::ListStyle => LIST_STYLE,
                ShorthandId::Margin => MARGIN,
                ShorthandId::MarginBlock => MARGIN_BLOCK,
                ShorthandId::MarginInline => MARGIN_INLINE,
                ShorthandId::Outline => OUTLINE,
                ShorthandId::Padding => PADDING,
                ShorthandId::PaddingBlock => PADDING_BLOCK,
                ShorthandId::PaddingInline => PADDING_INLINE,
                ShorthandId::FlexFlow => FLEX_FLOW,
                ShorthandId::Flex => FLEX,
                ShorthandId::Inset => INSET,
                ShorthandId::InsetBlock => INSET_BLOCK,
                ShorthandId::InsetInline => INSET_INLINE,
                ShorthandId::TextDecoration => TEXT_DECORATION,
                ShorthandId::All => ALL,
            }
            .iter(),
        }
    }

    /// Try to serialize the given declarations as this shorthand.
    ///
    /// Returns an error if writing to the stream fails, or if the declarations
    /// do not map to a shorthand.
    pub fn longhands_to_css<'a, W, I>(
        &self,
        declarations: I,
        dest: &mut CssWriter<W>,
    ) -> fmt::Result
    where
        W: Write,
        I: Iterator<Item = &'a Declaration>,
    {
        match *self {
            ShorthandId::All => {
                // No need to try to serialize the declarations as the 'all'
                // shorthand, since it only accepts CSS-wide keywords (and
                // variable references), which will be handled in
                // get_shorthand_appendable_value.
                Err(fmt::Error)
            },
            ShorthandId::Background => {
                match shorthands::background::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BackgroundPosition => {
                match shorthands::background_position::LonghandsToSerialize::from_iter(declarations)
                {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderColor => {
                match shorthands::border_color::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderStyle => {
                match shorthands::border_style::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderWidth => {
                match shorthands::border_width::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderTop => {
                match shorthands::border_top::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderRight => {
                match shorthands::border_right::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderBottom => {
                match shorthands::border_bottom::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderLeft => {
                match shorthands::border_left::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderBlockStart => {
                match shorthands::border_block_start::LonghandsToSerialize::from_iter(declarations)
                {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderBlockEnd => {
                match shorthands::border_block_end::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderInlineStart => {
                match shorthands::border_inline_start::LonghandsToSerialize::from_iter(declarations)
                {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderInlineEnd => {
                match shorthands::border_inline_end::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::Border => {
                match shorthands::border::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderRadius => {
                match shorthands::border_radius::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderImage => {
                match shorthands::border_image::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderBlockWidth => {
                match shorthands::border_block_width::LonghandsToSerialize::from_iter(declarations)
                {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderBlockStyle => {
                match shorthands::border_block_style::LonghandsToSerialize::from_iter(declarations)
                {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderBlockColor => {
                match shorthands::border_block_color::LonghandsToSerialize::from_iter(declarations)
                {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderInlineWidth => {
                match shorthands::border_inline_width::LonghandsToSerialize::from_iter(declarations)
                {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderInlineStyle => {
                match shorthands::border_inline_style::LonghandsToSerialize::from_iter(declarations)
                {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderInlineColor => {
                match shorthands::border_inline_color::LonghandsToSerialize::from_iter(declarations)
                {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderBlock => {
                match shorthands::border_block::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::BorderInline => {
                match shorthands::border_inline::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::Overflow => {
                match shorthands::overflow::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::Transition => {
                match shorthands::transition::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::Animation => {
                match shorthands::animation::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::Columns => {
                match shorthands::columns::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::Font => {
                match shorthands::font::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::FontVariant => {
                match shorthands::font_variant::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::ListStyle => {
                match shorthands::list_style::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::Margin => {
                match shorthands::margin::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::MarginBlock => {
                match shorthands::margin_block::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::MarginInline => {
                match shorthands::margin_inline::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::Outline => {
                match shorthands::outline::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::Padding => {
                match shorthands::padding::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::PaddingBlock => {
                match shorthands::padding_block::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::PaddingInline => {
                match shorthands::padding_inline::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::FlexFlow => {
                match shorthands::flex_flow::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::Flex => {
                match shorthands::flex::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::Inset => {
                match shorthands::inset::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::InsetBlock => {
                match shorthands::inset_block::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::InsetInline => {
                match shorthands::inset_inline::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
            ShorthandId::TextDecoration => {
                match shorthands::text_decoration::LonghandsToSerialize::from_iter(declarations) {
                    Ok(longhands) => longhands.to_css(dest),
                    Err(_) => Err(fmt::Error),
                }
            },
        }
    }

    /// Finds and returns an appendable value for the given declarations.
    ///
    /// Returns the optional appendable value.
    pub fn get_shorthand_appendable_value<'a, I>(
        self,
        declarations: I,
    ) -> Option<AppendableValue<'a, I::IntoIter>>
    where
        I: IntoIterator<Item = &'a Declaration>,
        I::IntoIter: Clone,
    {
        let declarations = declarations.into_iter();

        // Only cloning iterators (a few pointers each) not declarations.
        let mut declarations2 = declarations.clone();
        let mut declarations3 = declarations.clone();

        let first_declaration = declarations2.next()?;

        // https://drafts.csswg.org/css-variables/#variables-in-shorthands
        if let Some(css) = first_declaration.with_variables_from_shorthand(self) {
            if declarations2.all(|d| d.with_variables_from_shorthand(self) == Some(css)) {
                return Some(AppendableValue::Css {
                    css,
                    with_variables: true,
                });
            }
            return None;
        }

        // Check whether they are all the same CSS-wide keyword.
        if let Some(keyword) = first_declaration.get_css_wide_keyword() {
            if declarations2.all(|d| d.get_css_wide_keyword() == Some(keyword)) {
                return Some(AppendableValue::Css {
                    css: keyword.to_str(),
                    with_variables: false,
                });
            }
            return None;
        }

        // Check whether all declarations can be serialized as part of shorthand.
        if declarations3.all(|d| d.may_serialize_as_part_of_shorthand()) {
            return Some(AppendableValue::DeclarationsForShorthand(
                self,
                declarations,
            ));
        }

        None
    }

    /// Returns PropertyFlags for the given shorthand property.
    #[inline]
    pub fn flags(self) -> PropertyFlags {
        const FLAGS: [u16; 45] = [
            0,
            PropertyFlags::SHORTHAND_IN_GETCS.bits | 0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            PropertyFlags::SHORTHAND_IN_GETCS.bits | 0,
            0,
            0,
            0,
            0,
            PropertyFlags::SHORTHAND_IN_GETCS.bits | 0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            PropertyFlags::SHORTHAND_IN_GETCS.bits | 0,
            0,
        ];
        PropertyFlags::from_bits_truncate(FLAGS[self as usize])
    }

    /// Returns whether this property is a legacy shorthand.
    #[inline]
    pub fn is_legacy_shorthand(self) -> bool {
        self.flags().contains(PropertyFlags::IS_LEGACY_SHORTHAND)
    }

    /// Returns the order in which this property appears relative to other
    /// shorthands in idl-name-sorting order.
    #[inline]
    pub fn idl_name_sort_order(self) -> u32 {
        static IDL_NAME_SORT_ORDER: [u32; 45] = [
            2, 3, 12, 23, 25, 24, 22, 11, 20, 8, 7, 17, 16, 4, 21, 13, 10, 9, 6, 19, 18, 15, 5, 14,
            39, 44, 1, 26, 29, 30, 34, 35, 36, 37, 38, 40, 41, 42, 28, 27, 31, 32, 33, 43, 0,
        ];
        IDL_NAME_SORT_ORDER[self as usize]
    }

    pub fn parse_into<'i, 't>(
        &self,
        declarations: &mut SourcePropertyDeclaration,
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
        type ParseIntoFn = for<'i, 't> fn(
            declarations: &mut DeclarationBlock,
            context: &ParserContext,
            input: &mut Parser<'i, 't>,
        )
            -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>>;

        fn unreachable<'i, 't>(
            _: &mut DeclarationBlock,
            _: &ParserContext,
            _: &mut Parser<'i, 't>,
        ) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
            unreachable!()
        }

        // 'all' accepts no value other than CSS-wide keywords
        if *self == ShorthandId::All {
            return Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError));
        }

        static PARSE_INTO: [ParseIntoFn; 45] = [
            shorthands::background::parse_into,
            shorthands::background_position::parse_into,
            shorthands::border_color::parse_into,
            shorthands::border_style::parse_into,
            shorthands::border_width::parse_into,
            shorthands::border_top::parse_into,
            shorthands::border_right::parse_into,
            shorthands::border_bottom::parse_into,
            shorthands::border_left::parse_into,
            shorthands::border_block_start::parse_into,
            shorthands::border_block_end::parse_into,
            shorthands::border_inline_start::parse_into,
            shorthands::border_inline_end::parse_into,
            shorthands::border::parse_into,
            shorthands::border_radius::parse_into,
            shorthands::border_image::parse_into,
            shorthands::border_block_width::parse_into,
            shorthands::border_block_style::parse_into,
            shorthands::border_block_color::parse_into,
            shorthands::border_inline_width::parse_into,
            shorthands::border_inline_style::parse_into,
            shorthands::border_inline_color::parse_into,
            shorthands::border_block::parse_into,
            shorthands::border_inline::parse_into,
            shorthands::overflow::parse_into,
            shorthands::transition::parse_into,
            shorthands::animation::parse_into,
            shorthands::columns::parse_into,
            shorthands::font::parse_into,
            shorthands::font_variant::parse_into,
            shorthands::list_style::parse_into,
            shorthands::margin::parse_into,
            shorthands::margin_block::parse_into,
            shorthands::margin_inline::parse_into,
            shorthands::outline::parse_into,
            shorthands::padding::parse_into,
            shorthands::padding_block::parse_into,
            shorthands::padding_inline::parse_into,
            shorthands::flex_flow::parse_into,
            shorthands::flex::parse_into,
            shorthands::inset::parse_into,
            shorthands::inset_block::parse_into,
            shorthands::inset_inline::parse_into,
            shorthands::text_decoration::parse_into,
            unreachable,
        ];

        (PARSE_INTO[*self as usize])(declarations, context, input)
    }
}
