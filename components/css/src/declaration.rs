use crate::values::number::Integer;
use crate::{properties, values};

#[derive(Clone)]
pub enum DeclarationProperty {
    /// `align-content`
    AlignContent(properties::align_content::SpecifiedValue),
    /// `align-items`
    AlignItems(properties::align_items::SpecifiedValue),
    /// `align-self`
    AlignSelf(properties::align_self::SpecifiedValue),
    /// `backface-visibility`
    BackfaceVisibility(properties::backface_visibility::SpecifiedValue),
    /// `border-collapse`
    BorderCollapse(properties::border_collapse::SpecifiedValue),
    /// `border-image-repeat`
    BorderImageRepeat(properties::border_image_repeat::BorderImageRepeat),
    /// `box-sizing`
    BoxSizing(properties::box_sizing::SpecifiedValue),
    /// `caption-side`
    CaptionSide(properties::caption_side::CaptionSide),
    /// `clear`
    Clear(properties::clear::Clear),
    /// `column-count`
    ColumnCount(properties::column_count::ColumnCount),
    /// `direction`
    Direction(properties::direction::SpecifiedValue),
    /// `display`
    Display(properties::display::Display),
    /// `empty-cells`
    EmptyCells(properties::empty_cells::SpecifiedValue),
    /// `flex-direction`
    FlexDirection(properties::flex_direction::SpecifiedValue),
    /// `flex-wrap`
    FlexWrap(properties::flex_wrap::SpecifiedValue),
    /// `float`
    Float(properties::float::Float),
    /// `font-stretch`
    FontStretch(properties::font_stretch::FontStretch),
    /// `font-style`
    FontStyle(properties::font_style::FontStyle),
    /// `font-variant-caps`
    FontVariantCaps(properties::font_variant_caps::FontVariantCaps),
    /// `font-weight`
    FontWeight(properties::font_weight::FontWeight),
    /// `image-rendering`
    ImageRendering(properties::image_rendering::SpecifiedValue),
    /// `justify-content`
    JustifyContent(properties::justify_content::SpecifiedValue),
    /// `list-style-position`
    ListStylePosition(properties::list_style_position::SpecifiedValue),
    /// `list-style-type`
    ListStyleType(properties::list_style_type::SpecifiedValue),
    /// `mix-blend-mode`
    MixBlendMode(properties::mix_blend_mode::SpecifiedValue),
    /// `opacity`
    Opacity(properties::opacity::Opacity),
    /// `order`
    Order(Integer),
    /// `outline-style`
    OutlineStyle(properties::outline_style::OutlineStyle),
    /// `overflow-wrap`
    OverflowWrap(properties::overflow_wrap::OverflowWrap),
    /// `pointer-events`
    PointerEvents(properties::pointer_events::SpecifiedValue),
    /// `position`
    Position(properties::position::SpecifiedValue),
    /// `table-layout`
    TableLayout(properties::table_layout::SpecifiedValue),
    /// `text-align`
    TextAlign(properties::text_align::TextAlign),
    /// `text-decoration-line`
    TextDecorationLine(properties::text_decoration_line::TextDecorationLine),
    /// `text-justify`
    TextJustify(properties::text_justify::SpecifiedValue),
    /// `text-rendering`
    TextRendering(properties::text_rendering::SpecifiedValue),
    /// `text-transform`
    TextTransform(properties::text_transform::TextTransform),
    /// `transform-style`
    TransformStyle(properties::transform_style::TransformStyle),
    /// `unicode-bidi`
    UnicodeBidi(properties::unicode_bidi::SpecifiedValue),
    /// `visibility`
    Visibility(properties::visibility::SpecifiedValue),
    /// `white-space`
    WhiteSpace(properties::white_space::SpecifiedValue),
    /// `word-break`
    WordBreak(properties::word_break::WordBreak),
    /// `writing-mode`
    WritingMode(properties::writing_mode::SpecifiedValue),
    /// `z-index`
    ZIndex(properties::z_index::ZIndex),
    /// `flex-grow`
    FlexGrow(values::number::NonNegativeNumber),
    /// `flex-shrink`
    FlexShrink(values::number::NonNegativeNumber),
    /// `overflow-block`
    OverflowBlock(values::layout::Overflow),
    /// `overflow-inline`
    OverflowInline(values::layout::Overflow),
    /// `overflow-x`
    OverflowX(values::layout::Overflow),
    /// `overflow-y`
    OverflowY(values::layout::Overflow),
    /// `border-block-end-style`
    BorderBlockEndStyle(values::layout::BorderStyle),
    /// `border-block-start-style`
    BorderBlockStartStyle(values::layout::BorderStyle),
    /// `border-bottom-style`
    BorderBottomStyle(values::layout::BorderStyle),
    /// `border-inline-end-style`
    BorderInlineEndStyle(values::layout::BorderStyle),
    /// `border-inline-start-style`
    BorderInlineStartStyle(values::layout::BorderStyle),
    /// `border-left-style`
    BorderLeftStyle(values::layout::BorderStyle),
    /// `border-right-style`
    BorderRightStyle(values::layout::BorderStyle),
    /// `border-top-style`
    BorderTopStyle(values::layout::BorderStyle),
    /// `animation-delay`
    AnimationDelay(properties::animation_delay::AnimationDelay),
    /// `animation-direction`
    AnimationDirection(properties::animation_direction::AnimationDirection),
    /// `animation-duration`
    AnimationDuration(properties::animation_duration::AnimationDuration),
    /// `animation-fill-mode`
    AnimationFillMode(properties::animation_fill_mode::AnimationFillMode),
    /// `animation-iteration-count`
    AnimationIterationCount(properties::animation_iteration_count::AnimationIterationCount),
    /// `animation-name`
    AnimationName(properties::animation_name::AnimationName),
    /// `animation-play-state`
    AnimationPlayState(properties::animation_play_state::AnimationPlayState),
    /// `animation-timing-function`
    AnimationTimingFunction(values::animation::TimingFunction),
    /// `background-attachment`
    BackgroundAttachment(properties::background_attachment::BackgroundAttachment),
    /// `background-clip`
    BackgroundClip(properties::background_clip::BackgroundClip),
    /// `background-image`
    BackgroundImage(properties::background_image::BackgroundImage),
    /// `background-origin`
    BackgroundOrigin(properties::background_origin::BackgroundOrigin),
    /// `background-position-x`
    BackgroundPositionX(properties::background_position_x::BackgroundPositionX),
    /// `background-position-y`
    BackgroundPositionY(properties::background_position_y::BackgroundPositionY),
    /// `background-repeat`
    BackgroundRepeat(properties::background_repeat::BackgroundRepeat),
    /// `background-size`
    BackgroundSize(properties::background_size::BackgroundSize),
    /// `box-shadow`
    BoxShadow(properties::box_shadow::BoxShadow),
    /// `clip`
    Clip(properties::clip::Clip),
    /// `color`
    Color(values::color::Color),
    /// `column-gap`
    ColumnGap(values::length::NonNegativeLengthPercentageOrNormal),
    /// `column-width`
    ColumnWidth(values::length::NonNegativeLengthOrAuto),
    /// `content`
    Content(properties::content::Content),
    /// `cursor`
    Cursor(properties::cursor::Cursor),
    /// `filter`
    Filter(properties::filter::Filter),
    /// `flex-basis`
    FlexBasis(properties::flex_basis::FlexBasis),
    /// `font-family`
    FontFamily(properties::font_family::FontFamily),
    /// `font-size`
    FontSize(properties::font_size::FontSize),
    /// `letter-spacing`
    LetterSpacing(properties::letter_spacing::LetterSpacing),
    /// `line-height`
    LineHeight(properties::line_height::LineHeight),
    /// `outline-offset`
    OutlineOffset(values::length::Length),
    /// `perspective`
    Perspective(properties::perspective::Perspective),
    /// `perspective-origin`
    PerspectiveOrigin(properties::perspective_origin::PerspectiveOrigin),
    /// `quotes`
    Quotes(properties::quotes::Quotes),
    /// `rotate`
    Rotate(properties::rotate::Rotate),
    /// `scale`
    Scale(properties::scale::Scale),
    /// `text-indent`
    TextIndent(values::length::LengthPercentage),
    /// `text-overflow`
    TextOverflow(properties::text_overflow::TextOverflow),
    /// `text-shadow`
    TextShadow(properties::text_shadow::TextShadow),
    /// `transform`
    Transform(properties::transform::Transform),
    /// `transform-origin`
    TransformOrigin(properties::transform_origin::TransformOrigin),
    /// `transition-delay`
    TransitionDelay(properties::transition_delay::TransitionDelay),
    /// `transition-duration`
    TransitionDuration(properties::transition_duration::TransitionDuration),
    /// `transition-property`
    TransitionProperty(properties::transition_property::TransitionProperty),
    /// `transition-timing-function`
    TransitionTimingFunction(values::animation::TimingFunction),
    /// `translate`
    Translate(properties::translate::Translate),
    /// `vertical-align`
    VerticalAlign(properties::vertical_align::VerticalAlign),
    /// `word-spacing`
    WordSpacing(properties::word_spacing::WordSpacing),
    /// `border-image-source`
    BorderImageSource(values::image::Image),
    /// `list-style-image`
    ListStyleImage(values::image::Image),
    /// `max-block-size`
    MaxBlockSize(values::length::MaxSize),
    /// `max-height`
    MaxHeight(values::length::MaxSize),
    /// `max-inline-size`
    MaxInlineSize(values::length::MaxSize),
    /// `max-width`
    MaxWidth(values::length::MaxSize),
    /// `border-bottom-left-radius`
    BorderBottomLeftRadius(values::border::BorderCornerRadius),
    /// `border-bottom-right-radius`
    BorderBottomRightRadius(values::border::BorderCornerRadius),
    /// `border-end-end-radius`
    BorderEndEndRadius(values::border::BorderCornerRadius),
    /// `border-end-start-radius`
    BorderEndStartRadius(values::border::BorderCornerRadius),
    /// `border-start-end-radius`
    BorderStartEndRadius(values::border::BorderCornerRadius),
    /// `border-start-start-radius`
    BorderStartStartRadius(values::border::BorderCornerRadius),
    /// `border-top-left-radius`
    BorderTopLeftRadius(values::border::BorderCornerRadius),
    /// `border-top-right-radius`
    BorderTopRightRadius(values::border::BorderCornerRadius),
    /// `padding-block-end`
    PaddingBlockEnd(values::length::NonNegativeLengthPercentage),
    /// `padding-block-start`
    PaddingBlockStart(values::length::NonNegativeLengthPercentage),
    /// `padding-bottom`
    PaddingBottom(values::length::NonNegativeLengthPercentage),
    /// `padding-inline-end`
    PaddingInlineEnd(values::length::NonNegativeLengthPercentage),
    /// `padding-inline-start`
    PaddingInlineStart(values::length::NonNegativeLengthPercentage),
    /// `padding-left`
    PaddingLeft(values::length::NonNegativeLengthPercentage),
    /// `padding-right`
    PaddingRight(values::length::NonNegativeLengthPercentage),
    /// `padding-top`
    PaddingTop(values::length::NonNegativeLengthPercentage),
    /// `block-size`
    BlockSize(values::length::Size),
    /// `height`
    Height(values::length::Size),
    /// `inline-size`
    InlineSize(values::length::Size),
    /// `min-block-size`
    MinBlockSize(values::length::Size),
    /// `min-height`
    MinHeight(values::length::Size),
    /// `min-inline-size`
    MinInlineSize(values::length::Size),
    /// `min-width`
    MinWidth(values::length::Size),
    /// `width`
    Width(values::length::Size),
    /// `border-block-end-width`
    BorderBlockEndWidth(values::border::BorderSideWidth),
    /// `border-block-start-width`
    BorderBlockStartWidth(values::border::BorderSideWidth),
    /// `border-bottom-width`
    BorderBottomWidth(values::border::BorderSideWidth),
    /// `border-inline-end-width`
    BorderInlineEndWidth(values::border::BorderSideWidth),
    /// `border-inline-start-width`
    BorderInlineStartWidth(values::border::BorderSideWidth),
    /// `border-left-width`
    BorderLeftWidth(values::border::BorderSideWidth),
    /// `border-right-width`
    BorderRightWidth(values::border::BorderSideWidth),
    /// `border-top-width`
    BorderTopWidth(values::border::BorderSideWidth),
    /// `outline-width`
    OutlineWidth(values::border::BorderSideWidth),
    /// `background-color`
    BackgroundColor(values::color::Color),
    /// `border-block-end-color`
    BorderBlockEndColor(values::color::Color),
    /// `border-block-start-color`
    BorderBlockStartColor(values::color::Color),
    /// `border-bottom-color`
    BorderBottomColor(values::color::Color),
    /// `border-inline-end-color`
    BorderInlineEndColor(values::color::Color),
    /// `border-inline-start-color`
    BorderInlineStartColor(values::color::Color),
    /// `border-left-color`
    BorderLeftColor(values::color::Color),
    /// `border-right-color`
    BorderRightColor(values::color::Color),
    /// `border-top-color`
    BorderTopColor(values::color::Color),
    /// `outline-color`
    OutlineColor(values::color::Color),
    /// `bottom`
    Bottom(values::length::LengthPercentageOrAuto),
    /// `inset-block-end`
    InsetBlockEnd(values::length::LengthPercentageOrAuto),
    /// `inset-block-start`
    InsetBlockStart(values::length::LengthPercentageOrAuto),
    /// `inset-inline-end`
    InsetInlineEnd(values::length::LengthPercentageOrAuto),
    /// `inset-inline-start`
    InsetInlineStart(values::length::LengthPercentageOrAuto),
    /// `left`
    Left(values::length::LengthPercentageOrAuto),
    /// `margin-block-end`
    MarginBlockEnd(values::length::LengthPercentageOrAuto),
    /// `margin-block-start`
    MarginBlockStart(values::length::LengthPercentageOrAuto),
    /// `margin-bottom`
    MarginBottom(values::length::LengthPercentageOrAuto),
    /// `margin-inline-end`
    MarginInlineEnd(values::length::LengthPercentageOrAuto),
    /// `margin-inline-start`
    MarginInlineStart(values::length::LengthPercentageOrAuto),
    /// `margin-left`
    MarginLeft(values::length::LengthPercentageOrAuto),
    /// `margin-right`
    MarginRight(values::length::LengthPercentageOrAuto),
    /// `margin-top`
    MarginTop(values::length::LengthPercentageOrAuto),
    /// `right`
    Right(values::length::LengthPercentageOrAuto),
    /// `top`
    Top(values::length::LengthPercentageOrAuto),
}

#[derive(Clone)]
pub struct Declaration {
    property_name: DeclarationProperty,
    important: bool,
}

bitflags! {
    /// A set of flags for properties.
    pub struct PropertyFlags: u16 {
        /// This property requires a stacking context.
        const CREATES_STACKING_CONTEXT = 1 << 0;
        /// This property has values that can establish a containing block for
        /// fixed positioned and absolutely positioned elements.
        const FIXPOS_CB = 1 << 1;
        /// This property has values that can establish a containing block for
        /// absolutely positioned elements.
        const ABSPOS_CB = 1 << 2;
        /// This longhand property applies to ::first-letter.
        const APPLIES_TO_FIRST_LETTER = 1 << 3;
        /// This longhand property applies to ::first-line.
        const APPLIES_TO_FIRST_LINE = 1 << 4;
        /// This longhand property applies to ::placeholder.
        const APPLIES_TO_PLACEHOLDER = 1 << 5;
        ///  This longhand property applies to ::cue.
        const APPLIES_TO_CUE = 1 << 6;
        /// This longhand property applies to ::marker.
        const APPLIES_TO_MARKER = 1 << 7;
        /// This property is a legacy shorthand.
        ///
        /// https://drafts.csswg.org/css-cascade/#legacy-shorthand
        const IS_LEGACY_SHORTHAND = 1 << 8;

        /* The following flags are currently not used in Rust code, they
         * only need to be listed in corresponding properties so that
         * they can be checked in the C++ side via ServoCSSPropList.h. */
        /// This property can be animated on the compositor.
        const CAN_ANIMATE_ON_COMPOSITOR = 0;
        /// This shorthand property is accessible from getComputedStyle.
        const SHORTHAND_IN_GETCS = 0;
    }
}
