use core::fmt;

use cssparser::{ascii_case_insensitive_phf_map, match_ignore_ascii_case};

use crate::properties::{custom_properties, longhands};
use crate::stylesheets::css_rule::CssRuleType;
use crate::stylesheets::origin::Origin;
use crate::stylesheets::stylesheet::ParserContext;
use cssparser::_cssparser_internal_to_lowercase;

use super::longhand_id::LonghandId;
use super::shorthand_id::ShorthandId;

/// Servo's representation of a CSS property, that is, either a longhand, a
/// shorthand, or a custom property.
#[derive(Clone, Eq, PartialEq)]
pub enum PropertyId {
    /// A longhand property.
    Longhand(LonghandId),
    /// A shorthand property.
    Shorthand(ShorthandId),
    /// An alias for a longhand property.
    LonghandAlias(LonghandId, AliasId),
    /// An alias for a shorthand property.
    ShorthandAlias(ShorthandId, AliasId),
    /// A custom property.
    Custom(String),
}

impl PropertyId {
    /// Returns a given property from the given name, _regardless of whether it
    /// is enabled or not_, or Err(()) for unknown properties.
    fn parse_unchecked(property_name: &str) -> Result<Self, ()> {
        // A special id for css use counters.
        // ShorthandAlias is not used in the Servo build.
        // That's why we need to allow dead_code.
        #[allow(dead_code)]
        pub enum StaticId {
            Longhand(LonghandId),
            Shorthand(ShorthandId),
            LonghandAlias(LonghandId, AliasId),
            ShorthandAlias(ShorthandId, AliasId),
        }
        ascii_case_insensitive_phf_map! {
            static_id -> StaticId = {
                "align-content" => StaticId::Longhand(LonghandId::AlignContent),
                "align-items" => StaticId::Longhand(LonghandId::AlignItems),
                "align-self" => StaticId::Longhand(LonghandId::AlignSelf),
                "aspect-ratio" => StaticId::Longhand(LonghandId::AspectRatio),
                "backface-visibility" => StaticId::Longhand(LonghandId::BackfaceVisibility),
                "border-collapse" => StaticId::Longhand(LonghandId::BorderCollapse),
                "border-image-repeat" => StaticId::Longhand(LonghandId::BorderImageRepeat),
                "box-sizing" => StaticId::Longhand(LonghandId::BoxSizing),
                "caption-side" => StaticId::Longhand(LonghandId::CaptionSide),
                "clear" => StaticId::Longhand(LonghandId::Clear),
                "column-count" => StaticId::Longhand(LonghandId::ColumnCount),
                "direction" => StaticId::Longhand(LonghandId::Direction),
                "display" => StaticId::Longhand(LonghandId::Display),
                "empty-cells" => StaticId::Longhand(LonghandId::EmptyCells),
                "flex-direction" => StaticId::Longhand(LonghandId::FlexDirection),
                "flex-wrap" => StaticId::Longhand(LonghandId::FlexWrap),
                "float" => StaticId::Longhand(LonghandId::Float),
                "font-stretch" => StaticId::Longhand(LonghandId::FontStretch),
                "font-style" => StaticId::Longhand(LonghandId::FontStyle),
                "font-variant-caps" => StaticId::Longhand(LonghandId::FontVariantCaps),
                "font-weight" => StaticId::Longhand(LonghandId::FontWeight),
                "image-rendering" => StaticId::Longhand(LonghandId::ImageRendering),
                "justify-content" => StaticId::Longhand(LonghandId::JustifyContent),
                "list-style-position" => StaticId::Longhand(LonghandId::ListStylePosition),
                "list-style-type" => StaticId::Longhand(LonghandId::ListStyleType),
                "mix-blend-mode" => StaticId::Longhand(LonghandId::MixBlendMode),
                "opacity" => StaticId::Longhand(LonghandId::Opacity),
                "order" => StaticId::Longhand(LonghandId::Order),
                "outline-style" => StaticId::Longhand(LonghandId::OutlineStyle),
                "overflow-wrap" => StaticId::Longhand(LonghandId::OverflowWrap),
                "word-wrap" => {
                    StaticId::LonghandAlias(
                        LonghandId::OverflowWrap,
                        AliasId::WordWrap,
                    )
                },
                "pointer-events" => StaticId::Longhand(LonghandId::PointerEvents),
                "position" => StaticId::Longhand(LonghandId::Position),
                "-servo-overflow-clip-box" => StaticId::Longhand(LonghandId::ServoOverflowClipBox),
                "-servo-top-layer" => StaticId::Longhand(LonghandId::ServoTopLayer),
                "table-layout" => StaticId::Longhand(LonghandId::TableLayout),
                "text-align" => StaticId::Longhand(LonghandId::TextAlign),
                "text-decoration-line" => StaticId::Longhand(LonghandId::TextDecorationLine),
                "text-justify" => StaticId::Longhand(LonghandId::TextJustify),
                "text-rendering" => StaticId::Longhand(LonghandId::TextRendering),
                "text-transform" => StaticId::Longhand(LonghandId::TextTransform),
                "transform-style" => StaticId::Longhand(LonghandId::TransformStyle),
                "unicode-bidi" => StaticId::Longhand(LonghandId::UnicodeBidi),
                "visibility" => StaticId::Longhand(LonghandId::Visibility),
                "white-space" => StaticId::Longhand(LonghandId::WhiteSpace),
                "word-break" => StaticId::Longhand(LonghandId::WordBreak),
                "writing-mode" => StaticId::Longhand(LonghandId::WritingMode),
                "z-index" => StaticId::Longhand(LonghandId::ZIndex),
                "flex-grow" => StaticId::Longhand(LonghandId::FlexGrow),
                "flex-shrink" => StaticId::Longhand(LonghandId::FlexShrink),
                "overflow-block" => StaticId::Longhand(LonghandId::OverflowBlock),
                "overflow-inline" => StaticId::Longhand(LonghandId::OverflowInline),
                "overflow-x" => StaticId::Longhand(LonghandId::OverflowX),
                "overflow-y" => StaticId::Longhand(LonghandId::OverflowY),
                "border-block-end-style" => StaticId::Longhand(LonghandId::BorderBlockEndStyle),
                "border-block-start-style" => StaticId::Longhand(LonghandId::BorderBlockStartStyle),
                "border-bottom-style" => StaticId::Longhand(LonghandId::BorderBottomStyle),
                "border-inline-end-style" => StaticId::Longhand(LonghandId::BorderInlineEndStyle),
                "border-inline-start-style" => StaticId::Longhand(LonghandId::BorderInlineStartStyle),
                "border-left-style" => StaticId::Longhand(LonghandId::BorderLeftStyle),
                "border-right-style" => StaticId::Longhand(LonghandId::BorderRightStyle),
                "border-top-style" => StaticId::Longhand(LonghandId::BorderTopStyle),
                "animation-delay" => StaticId::Longhand(LonghandId::AnimationDelay),
                "animation-direction" => StaticId::Longhand(LonghandId::AnimationDirection),
                "animation-duration" => StaticId::Longhand(LonghandId::AnimationDuration),
                "animation-fill-mode" => StaticId::Longhand(LonghandId::AnimationFillMode),
                "animation-iteration-count" => StaticId::Longhand(LonghandId::AnimationIterationCount),
                "animation-name" => StaticId::Longhand(LonghandId::AnimationName),
                "animation-play-state" => StaticId::Longhand(LonghandId::AnimationPlayState),
                "animation-timing-function" => StaticId::Longhand(LonghandId::AnimationTimingFunction),
                "background-attachment" => StaticId::Longhand(LonghandId::BackgroundAttachment),
                "background-clip" => StaticId::Longhand(LonghandId::BackgroundClip),
                "background-image" => StaticId::Longhand(LonghandId::BackgroundImage),
                "background-origin" => StaticId::Longhand(LonghandId::BackgroundOrigin),
                "background-position-x" => StaticId::Longhand(LonghandId::BackgroundPositionX),
                "background-position-y" => StaticId::Longhand(LonghandId::BackgroundPositionY),
                "background-repeat" => StaticId::Longhand(LonghandId::BackgroundRepeat),
                "background-size" => StaticId::Longhand(LonghandId::BackgroundSize),
                "border-image-outset" => StaticId::Longhand(LonghandId::BorderImageOutset),
                "border-image-slice" => StaticId::Longhand(LonghandId::BorderImageSlice),
                "border-image-width" => StaticId::Longhand(LonghandId::BorderImageWidth),
                "border-spacing" => StaticId::Longhand(LonghandId::BorderSpacing),
                "box-shadow" => StaticId::Longhand(LonghandId::BoxShadow),
                "clip" => StaticId::Longhand(LonghandId::Clip),
                "color" => StaticId::Longhand(LonghandId::Color),
                "column-gap" => StaticId::Longhand(LonghandId::ColumnGap),
                "column-width" => StaticId::Longhand(LonghandId::ColumnWidth),
                "content" => StaticId::Longhand(LonghandId::Content),
                "counter-increment" => StaticId::Longhand(LonghandId::CounterIncrement),
                "counter-reset" => StaticId::Longhand(LonghandId::CounterReset),
                "cursor" => StaticId::Longhand(LonghandId::Cursor),
                "filter" => StaticId::Longhand(LonghandId::Filter),
                "flex-basis" => StaticId::Longhand(LonghandId::FlexBasis),
                "font-family" => StaticId::Longhand(LonghandId::FontFamily),
                "font-size" => StaticId::Longhand(LonghandId::FontSize),
                "letter-spacing" => StaticId::Longhand(LonghandId::LetterSpacing),
                "line-height" => StaticId::Longhand(LonghandId::LineHeight),
                "outline-offset" => StaticId::Longhand(LonghandId::OutlineOffset),
                "perspective" => StaticId::Longhand(LonghandId::Perspective),
                "perspective-origin" => StaticId::Longhand(LonghandId::PerspectiveOrigin),
                "quotes" => StaticId::Longhand(LonghandId::Quotes),
                "rotate" => StaticId::Longhand(LonghandId::Rotate),
                "scale" => StaticId::Longhand(LonghandId::Scale),
                "text-indent" => StaticId::Longhand(LonghandId::TextIndent),
                "text-overflow" => StaticId::Longhand(LonghandId::TextOverflow),
                "text-shadow" => StaticId::Longhand(LonghandId::TextShadow),
                "transform" => StaticId::Longhand(LonghandId::Transform),
                "transform-origin" => StaticId::Longhand(LonghandId::TransformOrigin),
                "transition-delay" => StaticId::Longhand(LonghandId::TransitionDelay),
                "transition-duration" => StaticId::Longhand(LonghandId::TransitionDuration),
                "transition-property" => StaticId::Longhand(LonghandId::TransitionProperty),
                "transition-timing-function" => StaticId::Longhand(LonghandId::TransitionTimingFunction),
                "translate" => StaticId::Longhand(LonghandId::Translate),
                "vertical-align" => StaticId::Longhand(LonghandId::VerticalAlign),
                "word-spacing" => StaticId::Longhand(LonghandId::WordSpacing),
                "border-image-source" => StaticId::Longhand(LonghandId::BorderImageSource),
                "list-style-image" => StaticId::Longhand(LonghandId::ListStyleImage),
                "max-block-size" => StaticId::Longhand(LonghandId::MaxBlockSize),
                "max-height" => StaticId::Longhand(LonghandId::MaxHeight),
                "max-inline-size" => StaticId::Longhand(LonghandId::MaxInlineSize),
                "max-width" => StaticId::Longhand(LonghandId::MaxWidth),
                "border-bottom-left-radius" => StaticId::Longhand(LonghandId::BorderBottomLeftRadius),
                "border-bottom-right-radius" => StaticId::Longhand(LonghandId::BorderBottomRightRadius),
                "border-end-end-radius" => StaticId::Longhand(LonghandId::BorderEndEndRadius),
                "border-end-start-radius" => StaticId::Longhand(LonghandId::BorderEndStartRadius),
                "border-start-end-radius" => StaticId::Longhand(LonghandId::BorderStartEndRadius),
                "border-start-start-radius" => StaticId::Longhand(LonghandId::BorderStartStartRadius),
                "border-top-left-radius" => StaticId::Longhand(LonghandId::BorderTopLeftRadius),
                "border-top-right-radius" => StaticId::Longhand(LonghandId::BorderTopRightRadius),
                "padding-block-end" => StaticId::Longhand(LonghandId::PaddingBlockEnd),
                "padding-block-start" => StaticId::Longhand(LonghandId::PaddingBlockStart),
                "padding-bottom" => StaticId::Longhand(LonghandId::PaddingBottom),
                "padding-inline-end" => StaticId::Longhand(LonghandId::PaddingInlineEnd),
                "padding-inline-start" => StaticId::Longhand(LonghandId::PaddingInlineStart),
                "padding-left" => StaticId::Longhand(LonghandId::PaddingLeft),
                "padding-right" => StaticId::Longhand(LonghandId::PaddingRight),
                "padding-top" => StaticId::Longhand(LonghandId::PaddingTop),
                "block-size" => StaticId::Longhand(LonghandId::BlockSize),
                "height" => StaticId::Longhand(LonghandId::Height),
                "inline-size" => StaticId::Longhand(LonghandId::InlineSize),
                "min-block-size" => StaticId::Longhand(LonghandId::MinBlockSize),
                "min-height" => StaticId::Longhand(LonghandId::MinHeight),
                "min-inline-size" => StaticId::Longhand(LonghandId::MinInlineSize),
                "min-width" => StaticId::Longhand(LonghandId::MinWidth),
                "width" => StaticId::Longhand(LonghandId::Width),
                "border-block-end-width" => StaticId::Longhand(LonghandId::BorderBlockEndWidth),
                "border-block-start-width" => StaticId::Longhand(LonghandId::BorderBlockStartWidth),
                "border-bottom-width" => StaticId::Longhand(LonghandId::BorderBottomWidth),
                "border-inline-end-width" => StaticId::Longhand(LonghandId::BorderInlineEndWidth),
                "border-inline-start-width" => StaticId::Longhand(LonghandId::BorderInlineStartWidth),
                "border-left-width" => StaticId::Longhand(LonghandId::BorderLeftWidth),
                "border-right-width" => StaticId::Longhand(LonghandId::BorderRightWidth),
                "border-top-width" => StaticId::Longhand(LonghandId::BorderTopWidth),
                "outline-width" => StaticId::Longhand(LonghandId::OutlineWidth),
                "background-color" => StaticId::Longhand(LonghandId::BackgroundColor),
                "border-block-end-color" => StaticId::Longhand(LonghandId::BorderBlockEndColor),
                "border-block-start-color" => StaticId::Longhand(LonghandId::BorderBlockStartColor),
                "border-bottom-color" => StaticId::Longhand(LonghandId::BorderBottomColor),
                "border-inline-end-color" => StaticId::Longhand(LonghandId::BorderInlineEndColor),
                "border-inline-start-color" => StaticId::Longhand(LonghandId::BorderInlineStartColor),
                "border-left-color" => StaticId::Longhand(LonghandId::BorderLeftColor),
                "border-right-color" => StaticId::Longhand(LonghandId::BorderRightColor),
                "border-top-color" => StaticId::Longhand(LonghandId::BorderTopColor),
                "outline-color" => StaticId::Longhand(LonghandId::OutlineColor),
                "bottom" => StaticId::Longhand(LonghandId::Bottom),
                "inset-block-end" => StaticId::Longhand(LonghandId::InsetBlockEnd),
                "inset-block-start" => StaticId::Longhand(LonghandId::InsetBlockStart),
                "inset-inline-end" => StaticId::Longhand(LonghandId::InsetInlineEnd),
                "inset-inline-start" => StaticId::Longhand(LonghandId::InsetInlineStart),
                "left" => StaticId::Longhand(LonghandId::Left),
                "margin-block-end" => StaticId::Longhand(LonghandId::MarginBlockEnd),
                "margin-block-start" => StaticId::Longhand(LonghandId::MarginBlockStart),
                "margin-bottom" => StaticId::Longhand(LonghandId::MarginBottom),
                "margin-inline-end" => StaticId::Longhand(LonghandId::MarginInlineEnd),
                "margin-inline-start" => StaticId::Longhand(LonghandId::MarginInlineStart),
                "margin-left" => StaticId::Longhand(LonghandId::MarginLeft),
                "margin-right" => StaticId::Longhand(LonghandId::MarginRight),
                "margin-top" => StaticId::Longhand(LonghandId::MarginTop),
                "right" => StaticId::Longhand(LonghandId::Right),
                "top" => StaticId::Longhand(LonghandId::Top),
                "background" => StaticId::Shorthand(ShorthandId::Background),
                "background-position" => StaticId::Shorthand(ShorthandId::BackgroundPosition),
                "border-color" => StaticId::Shorthand(ShorthandId::BorderColor),
                "border-style" => StaticId::Shorthand(ShorthandId::BorderStyle),
                "border-width" => StaticId::Shorthand(ShorthandId::BorderWidth),
                "border-top" => StaticId::Shorthand(ShorthandId::BorderTop),
                "border-right" => StaticId::Shorthand(ShorthandId::BorderRight),
                "border-bottom" => StaticId::Shorthand(ShorthandId::BorderBottom),
                "border-left" => StaticId::Shorthand(ShorthandId::BorderLeft),
                "border-block-start" => StaticId::Shorthand(ShorthandId::BorderBlockStart),
                "border-block-end" => StaticId::Shorthand(ShorthandId::BorderBlockEnd),
                "border-inline-start" => StaticId::Shorthand(ShorthandId::BorderInlineStart),
                "border-inline-end" => StaticId::Shorthand(ShorthandId::BorderInlineEnd),
                "border" => StaticId::Shorthand(ShorthandId::Border),
                "border-radius" => StaticId::Shorthand(ShorthandId::BorderRadius),
                "border-image" => StaticId::Shorthand(ShorthandId::BorderImage),
                "border-block-width" => StaticId::Shorthand(ShorthandId::BorderBlockWidth),
                "border-block-style" => StaticId::Shorthand(ShorthandId::BorderBlockStyle),
                "border-block-color" => StaticId::Shorthand(ShorthandId::BorderBlockColor),
                "border-inline-width" => StaticId::Shorthand(ShorthandId::BorderInlineWidth),
                "border-inline-style" => StaticId::Shorthand(ShorthandId::BorderInlineStyle),
                "border-inline-color" => StaticId::Shorthand(ShorthandId::BorderInlineColor),
                "border-block" => StaticId::Shorthand(ShorthandId::BorderBlock),
                "border-inline" => StaticId::Shorthand(ShorthandId::BorderInline),
                "overflow" => StaticId::Shorthand(ShorthandId::Overflow),
                "transition" => StaticId::Shorthand(ShorthandId::Transition),
                "animation" => StaticId::Shorthand(ShorthandId::Animation),
                "columns" => StaticId::Shorthand(ShorthandId::Columns),
                "font" => StaticId::Shorthand(ShorthandId::Font),
                "font-variant" => StaticId::Shorthand(ShorthandId::FontVariant),
                "list-style" => StaticId::Shorthand(ShorthandId::ListStyle),
                "margin" => StaticId::Shorthand(ShorthandId::Margin),
                "margin-block" => StaticId::Shorthand(ShorthandId::MarginBlock),
                "margin-inline" => StaticId::Shorthand(ShorthandId::MarginInline),
                "outline" => StaticId::Shorthand(ShorthandId::Outline),
                "padding" => StaticId::Shorthand(ShorthandId::Padding),
                "padding-block" => StaticId::Shorthand(ShorthandId::PaddingBlock),
                "padding-inline" => StaticId::Shorthand(ShorthandId::PaddingInline),
                "flex-flow" => StaticId::Shorthand(ShorthandId::FlexFlow),
                "flex" => StaticId::Shorthand(ShorthandId::Flex),
                "inset" => StaticId::Shorthand(ShorthandId::Inset),
                "inset-block" => StaticId::Shorthand(ShorthandId::InsetBlock),
                "inset-inline" => StaticId::Shorthand(ShorthandId::InsetInline),
                "text-decoration" => StaticId::Shorthand(ShorthandId::TextDecoration),
                "all" => StaticId::Shorthand(ShorthandId::All),
            }
        }

        if let Some(id) = static_id(property_name) {
            return Ok(match *id {
                StaticId::Longhand(id) => PropertyId::Longhand(id),
                StaticId::Shorthand(id) => PropertyId::Shorthand(id),
                StaticId::LonghandAlias(id, alias) => PropertyId::LonghandAlias(id, alias),
                StaticId::ShorthandAlias(id, alias) => PropertyId::ShorthandAlias(id, alias),
            });
        }

        let name = custom_properties::parse_name(property_name)?;
        Ok(PropertyId::Custom(name.to_string()))
    }

    /// Parses a property name, and returns an error if it's unknown or isn't
    /// allowed in this context.
    #[inline]
    pub fn parse(name: &str, context: &ParserContext) -> Result<Self, ()> {
        let id = Self::parse_unchecked(name)?;

        if !id.allowed_in(context) {
            return Err(());
        }

        Ok(id)
    }

    /// Returns longhand id if it is, None otherwise.
    #[inline]
    pub fn as_longhand(&self) -> Option<LonghandId> {
        match *self {
            PropertyId::Longhand(id) => Some(id),
            _ => None,
        }
    }

    pub fn allowed_in(&self, context: &ParserContext) -> bool {
        let id = match self.non_custom_id() {
            // Custom properties are allowed everywhere
            None => return true,
            Some(id) => id,
        };
        id.allowed_in(context)
    }

    /// Returns the `NonCustomPropertyId` corresponding to this property id.
    pub fn non_custom_id(&self) -> Option<NonCustomPropertyId> {
        Some(match *self {
            PropertyId::Custom(_) => return None,
            PropertyId::Shorthand(shorthand_id) => shorthand_id.into(),
            PropertyId::Longhand(longhand_id) => longhand_id.into(),
            PropertyId::ShorthandAlias(_, alias_id) => alias_id.into(),
            PropertyId::LonghandAlias(_, alias_id) => alias_id.into(),
        })
    }
}

impl fmt::Debug for PropertyId {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.to_css(&mut CssWriter::new(formatter))
    }
}

/// A longhand or shorthand property.
#[derive(Clone, Copy, Debug)]
pub struct NonCustomPropertyId(usize);

/// An identifier for a given alias property.
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u16)]
pub enum AliasId {
    /// word-wrap
    WordWrap = 0,
}

/// An iterator over all the property ids that are enabled for a given
/// shorthand, if that shorthand is enabled for all content too.
pub struct NonCustomPropertyIterator<Item: 'static> {
    filter: bool,
    iter: std::slice::Iter<'static, Item>,
}

impl<Item> Iterator for NonCustomPropertyIterator<Item>
where
    Item: 'static + Copy + Into<NonCustomPropertyId>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let id = *self.iter.next()?;
            if !self.filter || id.into().enabled_for_all_content() {
                return Some(id);
            }
        }
    }
}

impl NonCustomPropertyId {
    /// Returns the underlying index, used for use counter.
    pub fn bit(self) -> usize {
        self.0
    }

    /// Get the property name.
    #[inline]
    pub fn name(self) -> &'static str {
        static MAP: [&'static str; NON_CUSTOM_PROPERTY_ID_COUNT] = [
            "align-content",
            "align-items",
            "align-self",
            "aspect-ratio",
            "backface-visibility",
            "border-collapse",
            "border-image-repeat",
            "box-sizing",
            "caption-side",
            "clear",
            "column-count",
            "direction",
            "display",
            "empty-cells",
            "flex-direction",
            "flex-wrap",
            "float",
            "font-stretch",
            "font-style",
            "font-variant-caps",
            "font-weight",
            "image-rendering",
            "justify-content",
            "list-style-position",
            "list-style-type",
            "mix-blend-mode",
            "opacity",
            "order",
            "outline-style",
            "overflow-wrap",
            "pointer-events",
            "position",
            "-servo-overflow-clip-box",
            "-servo-top-layer",
            "table-layout",
            "text-align",
            "text-decoration-line",
            "text-justify",
            "text-rendering",
            "text-transform",
            "transform-style",
            "unicode-bidi",
            "visibility",
            "white-space",
            "word-break",
            "writing-mode",
            "z-index",
            "flex-grow",
            "flex-shrink",
            "overflow-block",
            "overflow-inline",
            "overflow-x",
            "overflow-y",
            "border-block-end-style",
            "border-block-start-style",
            "border-bottom-style",
            "border-inline-end-style",
            "border-inline-start-style",
            "border-left-style",
            "border-right-style",
            "border-top-style",
            "animation-delay",
            "animation-direction",
            "animation-duration",
            "animation-fill-mode",
            "animation-iteration-count",
            "animation-name",
            "animation-play-state",
            "animation-timing-function",
            "background-attachment",
            "background-clip",
            "background-image",
            "background-origin",
            "background-position-x",
            "background-position-y",
            "background-repeat",
            "background-size",
            "border-image-outset",
            "border-image-slice",
            "border-image-width",
            "border-spacing",
            "box-shadow",
            "clip",
            "color",
            "column-gap",
            "column-width",
            "content",
            "counter-increment",
            "counter-reset",
            "cursor",
            "filter",
            "flex-basis",
            "font-family",
            "font-size",
            "letter-spacing",
            "line-height",
            "outline-offset",
            "perspective",
            "perspective-origin",
            "quotes",
            "rotate",
            "scale",
            "text-indent",
            "text-overflow",
            "text-shadow",
            "transform",
            "transform-origin",
            "transition-delay",
            "transition-duration",
            "transition-property",
            "transition-timing-function",
            "translate",
            "vertical-align",
            "word-spacing",
            "border-image-source",
            "list-style-image",
            "max-block-size",
            "max-height",
            "max-inline-size",
            "max-width",
            "border-bottom-left-radius",
            "border-bottom-right-radius",
            "border-end-end-radius",
            "border-end-start-radius",
            "border-start-end-radius",
            "border-start-start-radius",
            "border-top-left-radius",
            "border-top-right-radius",
            "padding-block-end",
            "padding-block-start",
            "padding-bottom",
            "padding-inline-end",
            "padding-inline-start",
            "padding-left",
            "padding-right",
            "padding-top",
            "block-size",
            "height",
            "inline-size",
            "min-block-size",
            "min-height",
            "min-inline-size",
            "min-width",
            "width",
            "border-block-end-width",
            "border-block-start-width",
            "border-bottom-width",
            "border-inline-end-width",
            "border-inline-start-width",
            "border-left-width",
            "border-right-width",
            "border-top-width",
            "outline-width",
            "background-color",
            "border-block-end-color",
            "border-block-start-color",
            "border-bottom-color",
            "border-inline-end-color",
            "border-inline-start-color",
            "border-left-color",
            "border-right-color",
            "border-top-color",
            "outline-color",
            "bottom",
            "inset-block-end",
            "inset-block-start",
            "inset-inline-end",
            "inset-inline-start",
            "left",
            "margin-block-end",
            "margin-block-start",
            "margin-bottom",
            "margin-inline-end",
            "margin-inline-start",
            "margin-left",
            "margin-right",
            "margin-top",
            "right",
            "top",
            "background",
            "background-position",
            "border-color",
            "border-style",
            "border-width",
            "border-top",
            "border-right",
            "border-bottom",
            "border-left",
            "border-block-start",
            "border-block-end",
            "border-inline-start",
            "border-inline-end",
            "border",
            "border-radius",
            "border-image",
            "border-block-width",
            "border-block-style",
            "border-block-color",
            "border-inline-width",
            "border-inline-style",
            "border-inline-color",
            "border-block",
            "border-inline",
            "overflow",
            "transition",
            "animation",
            "columns",
            "font",
            "font-variant",
            "list-style",
            "margin",
            "margin-block",
            "margin-inline",
            "outline",
            "padding",
            "padding-block",
            "padding-inline",
            "flex-flow",
            "flex",
            "inset",
            "inset-block",
            "inset-inline",
            "text-decoration",
            "all",
            "word-wrap",
        ];
        MAP[self.0]
    }

    /// Returns whether this property is transitionable.
    #[inline]
    pub fn is_transitionable(self) -> bool {
        static TRANSITIONABLE: NonCustomPropertyIdSet = NonCustomPropertyIdSet {
            storage: [
                0xc160408, 0x1c400, 0xec3ff600, 0xfff38777, 0xffffffff, 0xffbfffff, 0xbdfcc76f, 0x0,
            ],
        };

        TRANSITIONABLE.contains(self)
    }

    /// Returns whether this property is animatable.
    #[inline]
    pub fn is_animatable(self) -> bool {
        static ANIMATABLE: NonCustomPropertyIdSet = NonCustomPropertyIdSet {
            storage: [
                0xfffff7ff, 0x1c9fddfc, 0xffffffe0, 0xffff87ff, 0xffffffff, 0xffffffff, 0xffffcf6f,
                0x1,
            ],
        };

        ANIMATABLE.contains(self)
    }

    #[inline]
    pub fn enabled_for_all_content(self) -> bool {
        static EXPERIMENTAL: NonCustomPropertyIdSet = NonCustomPropertyIdSet {
            storage: [0x400, 0x2000, 0x300000, 0x0, 0x0, 0x0, 0x4000, 0x0],
        };

        static ALWAYS_ENABLED: NonCustomPropertyIdSet = NonCustomPropertyIdSet {
            storage: [
                0xfffffbff, 0xffffdffc, 0xffcfffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffbfff,
                0x1,
            ],
        };

        if ALWAYS_ENABLED.contains(self) {
            return true;
        }

        if EXPERIMENTAL.contains(self) {
            return true;
        }

        false
    }

    /// Returns whether a given rule allows a given property.
    #[inline]
    pub fn allowed_in_rule(self, rule_type: CssRuleType) -> bool {
        debug_assert!(
            matches!(
                rule_type,
                CssRuleType::Keyframe | CssRuleType::Page | CssRuleType::Style
            ),
            "Declarations are only expected inside a keyframe, page, or style rule."
        );

        static MAP: [u8; NON_CUSTOM_PROPERTY_ID_COUNT] = [
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 1, 1, 5, 1, 1, 1, 1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 7,
            7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 1, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        ];
        match rule_type {
            CssRuleType::Style => MAP[self.0] & 1 != 0,
            CssRuleType::Page => MAP[self.0] & 2 != 0,
            CssRuleType::Keyframe => MAP[self.0] & 4 != 0,
            _ => true,
        }
    }

    pub fn allowed_in(self, context: &ParserContext) -> bool {
        if !self.allowed_in_rule(context.rule_type()) {
            return false;
        }

        self.allowed_in_ignoring_rule_type(context)
    }

    pub fn allowed_in_ignoring_rule_type(self, context: &ParserContext) -> bool {
        // The semantics of these are kinda hard to reason about, what follows
        // is a description of the different combinations that can happen with
        // these three sets.
        //
        // Experimental properties are generally controlled by prefs, but an
        // experimental property explicitly enabled in certain context (UA or
        // chrome sheets) is always usable in the context regardless of the
        // pref value.
        //
        // Non-experimental properties are either normal properties which are
        // usable everywhere, or internal-only properties which are only usable
        // in certain context they are explicitly enabled in.
        if self.enabled_for_all_content() {
            return true;
        }

        static ENABLED_IN_UA_SHEETS: NonCustomPropertyIdSet = NonCustomPropertyIdSet {
            storage: [0x0, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        };

        static ENABLED_IN_CHROME: NonCustomPropertyIdSet = NonCustomPropertyIdSet {
            storage: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        };

        if context.stylesheet_origin == Origin::UserAgent && ENABLED_IN_UA_SHEETS.contains(self) {
            return true;
        }

        if context.chrome_rules_enabled() && ENABLED_IN_CHROME.contains(self) {
            return true;
        }

        false
    }

    /// The supported types of this property. The return value should be
    /// style_traits::CssType when it can become a bitflags type.
    fn supported_types(&self) -> u8 {
        const SUPPORTED_TYPES: [u8; 224] = [
                <longhands::align_content::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::align_items::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::align_self::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::AspectRatio as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::backface_visibility::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::border_collapse::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderImageRepeat as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::box_sizing::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::table::CaptionSide as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Clear as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::ColumnCount as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::direction::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Display as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::empty_cells::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::flex_direction::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::flex_wrap::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Float as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::FontStretch as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::FontStyle as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::font_variant_caps::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::FontWeight as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::image_rendering::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::justify_content::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::list_style_position::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::list_style_type::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::mix_blend_mode::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Opacity as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Integer as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::OutlineStyle as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::OverflowWrap as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::pointer_events::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::position::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::_servo_overflow_clip_box::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::_servo_top_layer::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::table_layout::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::TextAlign as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::TextDecorationLine as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::text_justify::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::text_rendering::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::TextTransform as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::TransformStyle as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::unicode_bidi::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::visibility::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::white_space::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::WordBreak as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::writing_mode::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::ZIndex as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::NonNegativeNumber as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::NonNegativeNumber as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Overflow as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Overflow as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Overflow as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Overflow as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::animation_delay::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::animation_direction::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::animation_duration::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::animation_fill_mode::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::animation_iteration_count::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::animation_name::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::animation_play_state::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::animation_timing_function::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::background_attachment::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::background_clip::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::background_image::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::background_origin::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::background_position_x::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::background_position_y::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::background_repeat::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::background_size::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::NonNegativeLengthOrNumberRect> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::BorderImageSlice> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::BorderImageWidth> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::BorderSpacing> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::box_shadow::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::ClipRectOrAuto> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::ColorPropertyValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::length::NonNegativeLengthPercentageOrNormal as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::length::NonNegativeLengthOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Content as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::CounterIncrement as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::CounterSetOrReset as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Cursor as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::filter::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::FlexBasis> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::FontFamily as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::FontSize as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LetterSpacing as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LineHeight as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Length as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Perspective as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::Position> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Quotes as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::Rotate> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::Scale> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentage as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::TextOverflow> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::text_shadow::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Transform as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::TransformOrigin> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::transition_delay::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::transition_duration::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::transition_property::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <longhands::transition_timing_function::SpecifiedValue as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::Translate> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::VerticalAlign as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::WordSpacing as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::Image> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::Image> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::MaxSize as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::MaxSize as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::MaxSize as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::MaxSize as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Size as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Size as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Size as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Size as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Size as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Size as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Size as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Size as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Color as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Color as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Color as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Color as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Color as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Color as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Color as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Color as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Color as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::Color as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::background::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::background_position::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_color::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_style::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_width::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_top::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_right::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_bottom::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_left::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_block_start::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_block_end::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_inline_start::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_inline_end::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_radius::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_image::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_block_width::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_block_style::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_block_color::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_inline_width::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_inline_style::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_inline_color::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_block::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::border_inline::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::overflow::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::transition::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::animation::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::columns::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::font::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::font_variant::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::list_style::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::margin::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::margin_block::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::margin_inline::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::outline::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::padding::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::padding_block::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::padding_inline::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::flex_flow::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::flex::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::inset::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::inset_block::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::inset_inline::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                <shorthands::text_decoration::Longhands as SpecifiedValueInfo>::SUPPORTED_TYPES,
                0, // 'all' accepts no value other than CSS-wide keywords
        ];
        SUPPORTED_TYPES[self.0]
    }

    /// See PropertyId::collect_property_completion_keywords.
    fn collect_property_completion_keywords(&self, f: KeywordsCollectFn) {
        fn do_nothing(_: KeywordsCollectFn) {}
        const COLLECT_FUNCTIONS: [fn(KeywordsCollectFn);
                                  224] = [
                <longhands::align_content::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::align_items::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::align_self::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::AspectRatio as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::backface_visibility::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::border_collapse::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderImageRepeat as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::box_sizing::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::table::CaptionSide as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Clear as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::ColumnCount as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::direction::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Display as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::empty_cells::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::flex_direction::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::flex_wrap::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Float as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::FontStretch as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::FontStyle as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::font_variant_caps::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::FontWeight as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::image_rendering::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::justify_content::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::list_style_position::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::list_style_type::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::mix_blend_mode::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Opacity as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Integer as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::OutlineStyle as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::OverflowWrap as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::pointer_events::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::position::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::_servo_overflow_clip_box::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::_servo_top_layer::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::table_layout::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::TextAlign as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::TextDecorationLine as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::text_justify::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::text_rendering::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::TextTransform as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::TransformStyle as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::unicode_bidi::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::visibility::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::white_space::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::WordBreak as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::writing_mode::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::ZIndex as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::NonNegativeNumber as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::NonNegativeNumber as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Overflow as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Overflow as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Overflow as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Overflow as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderStyle as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::animation_delay::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::animation_direction::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::animation_duration::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::animation_fill_mode::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::animation_iteration_count::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::animation_name::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::animation_play_state::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::animation_timing_function::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::background_attachment::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::background_clip::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::background_image::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::background_origin::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::background_position_x::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::background_position_y::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::background_repeat::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::background_size::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::NonNegativeLengthOrNumberRect> as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::BorderImageSlice> as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::BorderImageWidth> as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::BorderSpacing> as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::box_shadow::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::ClipRectOrAuto> as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::ColorPropertyValue as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::length::NonNegativeLengthPercentageOrNormal as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::length::NonNegativeLengthOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Content as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::CounterIncrement as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::CounterSetOrReset as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Cursor as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::filter::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::FlexBasis> as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::FontFamily as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::FontSize as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LetterSpacing as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LineHeight as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Length as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Perspective as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::Position> as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Quotes as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::Rotate> as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::Scale> as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentage as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::TextOverflow> as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::text_shadow::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Transform as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::TransformOrigin> as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::transition_delay::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::transition_duration::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::transition_property::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <longhands::transition_timing_function::SpecifiedValue as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::Translate> as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::VerticalAlign as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::WordSpacing as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::Image> as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::Image> as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::MaxSize as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::MaxSize as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::MaxSize as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::MaxSize as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::collect_completion_keywords,
                <Box<crate::values::specified::BorderCornerRadius> as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::NonNegativeLengthPercentage as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Size as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Size as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Size as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Size as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Size as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Size as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Size as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Size as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::BorderSideWidth as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Color as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Color as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Color as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Color as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Color as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Color as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Color as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Color as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Color as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::Color as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <crate::values::specified::LengthPercentageOrAuto as SpecifiedValueInfo>::collect_completion_keywords,
                <shorthands::background::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::background_position::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_color::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_style::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_width::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_top::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_right::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_bottom::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_left::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_block_start::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_block_end::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_inline_start::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_inline_end::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_radius::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_image::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_block_width::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_block_style::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_block_color::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_inline_width::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_inline_style::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_inline_color::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_block::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::border_inline::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::overflow::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::transition::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::animation::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::columns::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::font::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::font_variant::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::list_style::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::margin::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::margin_block::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::margin_inline::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::outline::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::padding::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::padding_block::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::padding_inline::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::flex_flow::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::flex::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::inset::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::inset_block::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::inset_inline::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                <shorthands::text_decoration::Longhands as SpecifiedValueInfo>::
                    collect_completion_keywords,
                do_nothing, // 'all' accepts no value other than CSS-wide keywords
        ];
        COLLECT_FUNCTIONS[self.0](f);
    }

    /// Turns this `NonCustomPropertyId` into a `PropertyId`.
    #[inline]
    pub fn to_property_id(self) -> PropertyId {
        use std::mem::transmute;
        if self.0 < 179 {
            return unsafe { PropertyId::Longhand(transmute(self.0 as u16)) };
        }
        if self.0 < 224 {
            return unsafe { PropertyId::Shorthand(transmute((self.0 - 179) as u16)) };
        }
        assert!(self.0 < NON_CUSTOM_PROPERTY_ID_COUNT);
        let alias_id: AliasId = unsafe { transmute((self.0 - 224) as u16) };

        match alias_id.aliased_property() {
            AliasedPropertyId::Longhand(longhand) => PropertyId::LonghandAlias(longhand, alias_id),
            AliasedPropertyId::Shorthand(shorthand) => {
                PropertyId::ShorthandAlias(shorthand, alias_id)
            },
        }
    }
}

impl From<LonghandId> for NonCustomPropertyId {
    #[inline]
    fn from(id: LonghandId) -> Self {
        NonCustomPropertyId(id as usize)
    }
}

impl From<ShorthandId> for NonCustomPropertyId {
    #[inline]
    fn from(id: ShorthandId) -> Self {
        NonCustomPropertyId((id as usize) + 179)
    }
}

impl From<AliasId> for NonCustomPropertyId {
    #[inline]
    fn from(id: AliasId) -> Self {
        NonCustomPropertyId(id as usize + 224)
    }
}

/// An enum to represent a CSS Wide keyword.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CSSWideKeyword {
    /// The `initial` keyword.
    Initial,
    /// The `inherit` keyword.
    Inherit,
    /// The `unset` keyword.
    Unset,
    /// The `revert` keyword.
    Revert,
}

impl CSSWideKeyword {
    pub fn to_str(&self) -> &'static str {
        match *self {
            CSSWideKeyword::Initial => "initial",
            CSSWideKeyword::Inherit => "inherit",
            CSSWideKeyword::Unset => "unset",
            CSSWideKeyword::Revert => "revert",
        }
    }
}

impl CSSWideKeyword {
    pub fn parse(input: &mut Parser) -> Result<Self, ()> {
        let keyword = {
            let ident = input.expect_ident().map_err(|_| ())?;
            match_ignore_ascii_case! { ident,
                // If modifying this set of keyword, also update values::CustomIdent::from_ident
                "initial" => CSSWideKeyword::Initial,
                "inherit" => CSSWideKeyword::Inherit,
                "unset" => CSSWideKeyword::Unset,
                "revert" => CSSWideKeyword::Revert,
                _ => return Err(()),
            }
        };
        input.expect_exhausted().map_err(|_| ())?;
        Ok(keyword)
    }
}
