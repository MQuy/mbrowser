use crate::properties::longhands;
use crate::stylesheets::css_rule::CssRuleType;
use crate::stylesheets::origin::Origin;

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

        let passes_pref_check = || {
            static PREF_NAME: [Option<&str>; 225] = [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some("layout.columns.enabled"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some("layout.writing-mode.enabled"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some("layout.columns.enabled"),
                Some("layout.columns.enabled"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some("layout.columns.enabled"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ];
            let pref = match PREF_NAME[self.0] {
                None => return true,
                Some(pref) => pref,
            };

            prefs::pref_map().get(pref).as_bool().unwrap_or(false)
        };

        if ALWAYS_ENABLED.contains(self) {
            return true;
        }

        if EXPERIMENTAL.contains(self) && passes_pref_check() {
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

    fn allowed_in(self, context: &ParserContext) -> bool {
        if !self.allowed_in_rule(context.rule_type()) {
            return false;
        }

        self.allowed_in_ignoring_rule_type(context)
    }

    fn allowed_in_ignoring_rule_type(self, context: &ParserContext) -> bool {
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
