use cssparser::{ParseError, Parser};

use crate::declaration_block::{DeclarationBlock, SourcePropertyDeclaration};
use crate::properties::longhand_id::LonghandId;
use crate::properties::property_id::{CSSWideKeyword, PropertyId};
use crate::properties::shorthand_id::ShorthandId;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::Integer;
use crate::{properties, values};

#[derive(Clone)]
pub enum DeclarationProperty {
    /// `align-content`
    AlignContent(properties::longhands::align_content::SpecifiedValue),
    /// `align-items`
    AlignItems(properties::longhands::align_items::SpecifiedValue),
    /// `align-self`
    AlignSelf(properties::longhands::align_self::SpecifiedValue),
    /// `backface-visibility`
    BackfaceVisibility(properties::longhands::backface_visibility::SpecifiedValue),
    /// `border-collapse`
    BorderCollapse(properties::longhands::border_collapse::SpecifiedValue),
    /// `border-image-repeat`
    BorderImageRepeat(properties::longhands::border_image_repeat::BorderImageRepeat),
    /// `box-sizing`
    BoxSizing(properties::longhands::box_sizing::SpecifiedValue),
    /// `caption-side`
    CaptionSide(properties::longhands::caption_side::CaptionSide),
    /// `clear`
    Clear(properties::longhands::clear::Clear),
    /// `column-count`
    ColumnCount(properties::longhands::column_count::ColumnCount),
    /// `direction`
    Direction(properties::longhands::direction::SpecifiedValue),
    /// `display`
    Display(properties::longhands::display::Display),
    /// `empty-cells`
    EmptyCells(properties::longhands::empty_cells::SpecifiedValue),
    /// `flex-direction`
    FlexDirection(properties::longhands::flex_direction::SpecifiedValue),
    /// `flex-wrap`
    FlexWrap(properties::longhands::flex_wrap::SpecifiedValue),
    /// `float`
    Float(properties::longhands::float::Float),
    /// `font-stretch`
    FontStretch(properties::longhands::font_stretch::FontStretch),
    /// `font-style`
    FontStyle(properties::longhands::font_style::FontStyle),
    /// `font-variant-caps`
    FontVariantCaps(properties::longhands::font_variant_caps::FontVariantCaps),
    /// `font-weight`
    FontWeight(properties::longhands::font_weight::FontWeight),
    /// `image-rendering`
    ImageRendering(properties::longhands::image_rendering::SpecifiedValue),
    /// `justify-content`
    JustifyContent(properties::longhands::justify_content::SpecifiedValue),
    /// `list-style-position`
    ListStylePosition(properties::longhands::list_style_position::SpecifiedValue),
    /// `list-style-type`
    ListStyleType(properties::longhands::list_style_type::SpecifiedValue),
    /// `mix-blend-mode`
    MixBlendMode(properties::longhands::mix_blend_mode::SpecifiedValue),
    /// `opacity`
    Opacity(properties::longhands::opacity::Opacity),
    /// `order`
    Order(Integer),
    /// `outline-style`
    OutlineStyle(properties::longhands::outline_style::OutlineStyle),
    /// `overflow-wrap`
    OverflowWrap(properties::longhands::overflow_wrap::OverflowWrap),
    /// `pointer-events`
    PointerEvents(properties::longhands::pointer_events::SpecifiedValue),
    /// `position`
    Position(properties::longhands::position::SpecifiedValue),
    /// `table-layout`
    TableLayout(properties::longhands::table_layout::SpecifiedValue),
    /// `text-align`
    TextAlign(properties::longhands::text_align::TextAlign),
    /// `text-decoration-line`
    TextDecorationLine(properties::longhands::text_decoration_line::TextDecorationLine),
    /// `text-justify`
    TextJustify(properties::longhands::text_justify::SpecifiedValue),
    /// `text-rendering`
    TextRendering(properties::longhands::text_rendering::SpecifiedValue),
    /// `text-transform`
    TextTransform(properties::longhands::text_transform::TextTransform),
    /// `transform-style`
    TransformStyle(properties::longhands::transform_style::TransformStyle),
    /// `unicode-bidi`
    UnicodeBidi(properties::longhands::unicode_bidi::SpecifiedValue),
    /// `visibility`
    Visibility(properties::longhands::visibility::SpecifiedValue),
    /// `white-space`
    WhiteSpace(properties::longhands::white_space::SpecifiedValue),
    /// `word-break`
    WordBreak(properties::longhands::word_break::WordBreak),
    /// `writing-mode`
    WritingMode(properties::longhands::writing_mode::SpecifiedValue),
    /// `z-index`
    ZIndex(properties::longhands::z_index::ZIndex),
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
    AnimationDelay(properties::longhands::animation_delay::AnimationDelay),
    /// `animation-direction`
    AnimationDirection(properties::longhands::animation_direction::AnimationDirection),
    /// `animation-duration`
    AnimationDuration(properties::longhands::animation_duration::AnimationDuration),
    /// `animation-fill-mode`
    AnimationFillMode(properties::longhands::animation_fill_mode::AnimationFillMode),
    /// `animation-iteration-count`
    AnimationIterationCount(
        properties::longhands::animation_iteration_count::AnimationIterationCount,
    ),
    /// `animation-name`
    AnimationName(properties::longhands::animation_name::AnimationName),
    /// `animation-play-state`
    AnimationPlayState(properties::longhands::animation_play_state::AnimationPlayState),
    /// `animation-timing-function`
    AnimationTimingFunction(values::animation::TimingFunction),
    /// `background-attachment`
    BackgroundAttachment(properties::longhands::background_attachment::BackgroundAttachment),
    /// `background-clip`
    BackgroundClip(properties::longhands::background_clip::BackgroundClip),
    /// `background-image`
    BackgroundImage(properties::longhands::background_image::BackgroundImage),
    /// `background-origin`
    BackgroundOrigin(properties::longhands::background_origin::BackgroundOrigin),
    /// `background-position-x`
    BackgroundPositionX(properties::longhands::background_position_x::BackgroundPositionX),
    /// `background-position-y`
    BackgroundPositionY(properties::longhands::background_position_y::BackgroundPositionY),
    /// `background-repeat`
    BackgroundRepeat(properties::longhands::background_repeat::BackgroundRepeat),
    /// `background-size`
    BackgroundSize(properties::longhands::background_size::BackgroundSize),
    /// `box-shadow`
    BoxShadow(properties::longhands::box_shadow::BoxShadow),
    /// `clip`
    Clip(properties::longhands::clip::Clip),
    /// `color`
    Color(values::color::Color),
    /// `column-gap`
    ColumnGap(values::length::NonNegativeLengthPercentageOrNormal),
    /// `column-width`
    ColumnWidth(values::length::NonNegativeLengthOrAuto),
    /// `content`
    Content(properties::longhands::content::Content),
    /// `cursor`
    Cursor(properties::longhands::cursor::Cursor),
    /// `filter`
    Filter(properties::longhands::filter::Filter),
    /// `flex-basis`
    FlexBasis(properties::longhands::flex_basis::FlexBasis),
    /// `font-family`
    FontFamily(properties::longhands::font_family::FontFamily),
    /// `font-size`
    FontSize(properties::longhands::font_size::FontSize),
    /// `letter-spacing`
    LetterSpacing(properties::longhands::letter_spacing::LetterSpacing),
    /// `line-height`
    LineHeight(properties::longhands::line_height::LineHeight),
    /// `outline-offset`
    OutlineOffset(values::length::Length),
    /// `perspective`
    Perspective(properties::longhands::perspective::Perspective),
    /// `perspective-origin`
    PerspectiveOrigin(properties::longhands::perspective_origin::PerspectiveOrigin),
    /// `quotes`
    Quotes(properties::longhands::quotes::Quotes),
    /// `rotate`
    Rotate(properties::longhands::rotate::Rotate),
    /// `scale`
    Scale(properties::longhands::scale::Scale),
    /// `text-indent`
    TextIndent(values::length::LengthPercentage),
    /// `text-overflow`
    TextOverflow(properties::longhands::text_overflow::TextOverflow),
    /// `text-shadow`
    TextShadow(properties::longhands::text_shadow::TextShadow),
    /// `transform`
    Transform(properties::longhands::transform::Transform),
    /// `transform-origin`
    TransformOrigin(properties::longhands::transform_origin::TransformOrigin),
    /// `transition-delay`
    TransitionDelay(properties::longhands::transition_delay::TransitionDelay),
    /// `transition-duration`
    TransitionDuration(properties::longhands::transition_duration::TransitionDuration),
    /// `transition-property`
    TransitionProperty(properties::longhands::transition_property::TransitionProperty),
    /// `transition-timing-function`
    TransitionTimingFunction(values::animation::TimingFunction),
    /// `translate`
    Translate(properties::longhands::translate::Translate),
    /// `vertical-align`
    VerticalAlign(properties::longhands::vertical_align::VerticalAlign),
    /// `word-spacing`
    WordSpacing(properties::longhands::word_spacing::WordSpacing),
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
    /// A CSS-wide keyword.
    CSSWideKeyword(WideKeywordDeclaration),
}

impl DeclarationProperty {
    /// Returns a CSS-wide keyword declaration for a given property.
    #[inline]
    pub fn css_wide_keyword(id: LonghandId, keyword: CSSWideKeyword) -> Self {
        Self::CSSWideKeyword(WideKeywordDeclaration { id, keyword })
    }

    pub fn id(&self) -> PropertyId {
        match *self {
            DeclarationProperty::CSSWideKeyword(ref declaration) => {
                return PropertyId::Longhand(declaration.id);
            },
            _ => {},
        }
        // This is just fine because DeclarationProperty and LonghandId
        // have corresponding discriminants.
        let id = unsafe { *(self as *const _ as *const LonghandId) };
        debug_assert_eq!(
            id,
            match *self {
                DeclarationProperty::AlignContent(..) => LonghandId::AlignContent,
                DeclarationProperty::AlignItems(..) => LonghandId::AlignItems,
                DeclarationProperty::AlignSelf(..) => LonghandId::AlignSelf,
                DeclarationProperty::BackfaceVisibility(..) => LonghandId::BackfaceVisibility,
                DeclarationProperty::BorderCollapse(..) => LonghandId::BorderCollapse,
                DeclarationProperty::BorderImageRepeat(..) => LonghandId::BorderImageRepeat,
                DeclarationProperty::BoxSizing(..) => LonghandId::BoxSizing,
                DeclarationProperty::CaptionSide(..) => LonghandId::CaptionSide,
                DeclarationProperty::Clear(..) => LonghandId::Clear,
                DeclarationProperty::ColumnCount(..) => LonghandId::ColumnCount,
                DeclarationProperty::Direction(..) => LonghandId::Direction,
                DeclarationProperty::Display(..) => LonghandId::Display,
                DeclarationProperty::EmptyCells(..) => LonghandId::EmptyCells,
                DeclarationProperty::FlexDirection(..) => LonghandId::FlexDirection,
                DeclarationProperty::FlexWrap(..) => LonghandId::FlexWrap,
                DeclarationProperty::Float(..) => LonghandId::Float,
                DeclarationProperty::FontStretch(..) => LonghandId::FontStretch,
                DeclarationProperty::FontStyle(..) => LonghandId::FontStyle,
                DeclarationProperty::FontVariantCaps(..) => LonghandId::FontVariantCaps,
                DeclarationProperty::FontWeight(..) => LonghandId::FontWeight,
                DeclarationProperty::ImageRendering(..) => LonghandId::ImageRendering,
                DeclarationProperty::JustifyContent(..) => LonghandId::JustifyContent,
                DeclarationProperty::ListStylePosition(..) => LonghandId::ListStylePosition,
                DeclarationProperty::ListStyleType(..) => LonghandId::ListStyleType,
                DeclarationProperty::MixBlendMode(..) => LonghandId::MixBlendMode,
                DeclarationProperty::Opacity(..) => LonghandId::Opacity,
                DeclarationProperty::Order(..) => LonghandId::Order,
                DeclarationProperty::OutlineStyle(..) => LonghandId::OutlineStyle,
                DeclarationProperty::OverflowWrap(..) => LonghandId::OverflowWrap,
                DeclarationProperty::PointerEvents(..) => LonghandId::PointerEvents,
                DeclarationProperty::Position(..) => LonghandId::Position,
                DeclarationProperty::TableLayout(..) => LonghandId::TableLayout,
                DeclarationProperty::TextAlign(..) => LonghandId::TextAlign,
                DeclarationProperty::TextDecorationLine(..) => LonghandId::TextDecorationLine,
                DeclarationProperty::TextJustify(..) => LonghandId::TextJustify,
                DeclarationProperty::TextRendering(..) => LonghandId::TextRendering,
                DeclarationProperty::TextTransform(..) => LonghandId::TextTransform,
                DeclarationProperty::TransformStyle(..) => LonghandId::TransformStyle,
                DeclarationProperty::UnicodeBidi(..) => LonghandId::UnicodeBidi,
                DeclarationProperty::Visibility(..) => LonghandId::Visibility,
                DeclarationProperty::WhiteSpace(..) => LonghandId::WhiteSpace,
                DeclarationProperty::WordBreak(..) => LonghandId::WordBreak,
                DeclarationProperty::WritingMode(..) => LonghandId::WritingMode,
                DeclarationProperty::ZIndex(..) => LonghandId::ZIndex,
                DeclarationProperty::FlexGrow(..) => LonghandId::FlexGrow,
                DeclarationProperty::FlexShrink(..) => LonghandId::FlexShrink,
                DeclarationProperty::OverflowBlock(..) => LonghandId::OverflowBlock,
                DeclarationProperty::OverflowInline(..) => LonghandId::OverflowInline,
                DeclarationProperty::OverflowX(..) => LonghandId::OverflowX,
                DeclarationProperty::OverflowY(..) => LonghandId::OverflowY,
                DeclarationProperty::BorderBlockEndStyle(..) => LonghandId::BorderBlockEndStyle,
                DeclarationProperty::BorderBlockStartStyle(..) => LonghandId::BorderBlockStartStyle,
                DeclarationProperty::BorderBottomStyle(..) => LonghandId::BorderBottomStyle,
                DeclarationProperty::BorderInlineEndStyle(..) => LonghandId::BorderInlineEndStyle,
                DeclarationProperty::BorderInlineStartStyle(..) =>
                    LonghandId::BorderInlineStartStyle,
                DeclarationProperty::BorderLeftStyle(..) => LonghandId::BorderLeftStyle,
                DeclarationProperty::BorderRightStyle(..) => LonghandId::BorderRightStyle,
                DeclarationProperty::BorderTopStyle(..) => LonghandId::BorderTopStyle,
                DeclarationProperty::AnimationDelay(..) => LonghandId::AnimationDelay,
                DeclarationProperty::AnimationDirection(..) => LonghandId::AnimationDirection,
                DeclarationProperty::AnimationDuration(..) => LonghandId::AnimationDuration,
                DeclarationProperty::AnimationFillMode(..) => LonghandId::AnimationFillMode,
                DeclarationProperty::AnimationIterationCount(..) =>
                    LonghandId::AnimationIterationCount,
                DeclarationProperty::AnimationName(..) => LonghandId::AnimationName,
                DeclarationProperty::AnimationPlayState(..) => LonghandId::AnimationPlayState,
                DeclarationProperty::AnimationTimingFunction(..) =>
                    LonghandId::AnimationTimingFunction,
                DeclarationProperty::BackgroundAttachment(..) => LonghandId::BackgroundAttachment,
                DeclarationProperty::BackgroundClip(..) => LonghandId::BackgroundClip,
                DeclarationProperty::BackgroundImage(..) => LonghandId::BackgroundImage,
                DeclarationProperty::BackgroundOrigin(..) => LonghandId::BackgroundOrigin,
                DeclarationProperty::BackgroundPositionX(..) => LonghandId::BackgroundPositionX,
                DeclarationProperty::BackgroundPositionY(..) => LonghandId::BackgroundPositionY,
                DeclarationProperty::BackgroundRepeat(..) => LonghandId::BackgroundRepeat,
                DeclarationProperty::BackgroundSize(..) => LonghandId::BackgroundSize,
                DeclarationProperty::BoxShadow(..) => LonghandId::BoxShadow,
                DeclarationProperty::Clip(..) => LonghandId::Clip,
                DeclarationProperty::Color(..) => LonghandId::Color,
                DeclarationProperty::ColumnGap(..) => LonghandId::ColumnGap,
                DeclarationProperty::ColumnWidth(..) => LonghandId::ColumnWidth,
                DeclarationProperty::Content(..) => LonghandId::Content,
                DeclarationProperty::Cursor(..) => LonghandId::Cursor,
                DeclarationProperty::Filter(..) => LonghandId::Filter,
                DeclarationProperty::FlexBasis(..) => LonghandId::FlexBasis,
                DeclarationProperty::FontFamily(..) => LonghandId::FontFamily,
                DeclarationProperty::FontSize(..) => LonghandId::FontSize,
                DeclarationProperty::LetterSpacing(..) => LonghandId::LetterSpacing,
                DeclarationProperty::LineHeight(..) => LonghandId::LineHeight,
                DeclarationProperty::OutlineOffset(..) => LonghandId::OutlineOffset,
                DeclarationProperty::Perspective(..) => LonghandId::Perspective,
                DeclarationProperty::PerspectiveOrigin(..) => LonghandId::PerspectiveOrigin,
                DeclarationProperty::Quotes(..) => LonghandId::Quotes,
                DeclarationProperty::Rotate(..) => LonghandId::Rotate,
                DeclarationProperty::Scale(..) => LonghandId::Scale,
                DeclarationProperty::TextIndent(..) => LonghandId::TextIndent,
                DeclarationProperty::TextOverflow(..) => LonghandId::TextOverflow,
                DeclarationProperty::TextShadow(..) => LonghandId::TextShadow,
                DeclarationProperty::Transform(..) => LonghandId::Transform,
                DeclarationProperty::TransformOrigin(..) => LonghandId::TransformOrigin,
                DeclarationProperty::TransitionDelay(..) => LonghandId::TransitionDelay,
                DeclarationProperty::TransitionDuration(..) => LonghandId::TransitionDuration,
                DeclarationProperty::TransitionProperty(..) => LonghandId::TransitionProperty,
                DeclarationProperty::TransitionTimingFunction(..) =>
                    LonghandId::TransitionTimingFunction,
                DeclarationProperty::Translate(..) => LonghandId::Translate,
                DeclarationProperty::VerticalAlign(..) => LonghandId::VerticalAlign,
                DeclarationProperty::WordSpacing(..) => LonghandId::WordSpacing,
                DeclarationProperty::BorderImageSource(..) => LonghandId::BorderImageSource,
                DeclarationProperty::ListStyleImage(..) => LonghandId::ListStyleImage,
                DeclarationProperty::MaxBlockSize(..) => LonghandId::MaxBlockSize,
                DeclarationProperty::MaxHeight(..) => LonghandId::MaxHeight,
                DeclarationProperty::MaxInlineSize(..) => LonghandId::MaxInlineSize,
                DeclarationProperty::MaxWidth(..) => LonghandId::MaxWidth,
                DeclarationProperty::BorderBottomLeftRadius(..) =>
                    LonghandId::BorderBottomLeftRadius,
                DeclarationProperty::BorderBottomRightRadius(..) =>
                    LonghandId::BorderBottomRightRadius,
                DeclarationProperty::BorderEndEndRadius(..) => LonghandId::BorderEndEndRadius,
                DeclarationProperty::BorderEndStartRadius(..) => LonghandId::BorderEndStartRadius,
                DeclarationProperty::BorderStartEndRadius(..) => LonghandId::BorderStartEndRadius,
                DeclarationProperty::BorderStartStartRadius(..) =>
                    LonghandId::BorderStartStartRadius,
                DeclarationProperty::BorderTopLeftRadius(..) => LonghandId::BorderTopLeftRadius,
                DeclarationProperty::BorderTopRightRadius(..) => LonghandId::BorderTopRightRadius,
                DeclarationProperty::PaddingBlockEnd(..) => LonghandId::PaddingBlockEnd,
                DeclarationProperty::PaddingBlockStart(..) => LonghandId::PaddingBlockStart,
                DeclarationProperty::PaddingBottom(..) => LonghandId::PaddingBottom,
                DeclarationProperty::PaddingInlineEnd(..) => LonghandId::PaddingInlineEnd,
                DeclarationProperty::PaddingInlineStart(..) => LonghandId::PaddingInlineStart,
                DeclarationProperty::PaddingLeft(..) => LonghandId::PaddingLeft,
                DeclarationProperty::PaddingRight(..) => LonghandId::PaddingRight,
                DeclarationProperty::PaddingTop(..) => LonghandId::PaddingTop,
                DeclarationProperty::BlockSize(..) => LonghandId::BlockSize,
                DeclarationProperty::Height(..) => LonghandId::Height,
                DeclarationProperty::InlineSize(..) => LonghandId::InlineSize,
                DeclarationProperty::MinBlockSize(..) => LonghandId::MinBlockSize,
                DeclarationProperty::MinHeight(..) => LonghandId::MinHeight,
                DeclarationProperty::MinInlineSize(..) => LonghandId::MinInlineSize,
                DeclarationProperty::MinWidth(..) => LonghandId::MinWidth,
                DeclarationProperty::Width(..) => LonghandId::Width,
                DeclarationProperty::BorderBlockEndWidth(..) => LonghandId::BorderBlockEndWidth,
                DeclarationProperty::BorderBlockStartWidth(..) => LonghandId::BorderBlockStartWidth,
                DeclarationProperty::BorderBottomWidth(..) => LonghandId::BorderBottomWidth,
                DeclarationProperty::BorderInlineEndWidth(..) => LonghandId::BorderInlineEndWidth,
                DeclarationProperty::BorderInlineStartWidth(..) =>
                    LonghandId::BorderInlineStartWidth,
                DeclarationProperty::BorderLeftWidth(..) => LonghandId::BorderLeftWidth,
                DeclarationProperty::BorderRightWidth(..) => LonghandId::BorderRightWidth,
                DeclarationProperty::BorderTopWidth(..) => LonghandId::BorderTopWidth,
                DeclarationProperty::OutlineWidth(..) => LonghandId::OutlineWidth,
                DeclarationProperty::BackgroundColor(..) => LonghandId::BackgroundColor,
                DeclarationProperty::BorderBlockEndColor(..) => LonghandId::BorderBlockEndColor,
                DeclarationProperty::BorderBlockStartColor(..) => LonghandId::BorderBlockStartColor,
                DeclarationProperty::BorderBottomColor(..) => LonghandId::BorderBottomColor,
                DeclarationProperty::BorderInlineEndColor(..) => LonghandId::BorderInlineEndColor,
                DeclarationProperty::BorderInlineStartColor(..) =>
                    LonghandId::BorderInlineStartColor,
                DeclarationProperty::BorderLeftColor(..) => LonghandId::BorderLeftColor,
                DeclarationProperty::BorderRightColor(..) => LonghandId::BorderRightColor,
                DeclarationProperty::BorderTopColor(..) => LonghandId::BorderTopColor,
                DeclarationProperty::OutlineColor(..) => LonghandId::OutlineColor,
                DeclarationProperty::Bottom(..) => LonghandId::Bottom,
                DeclarationProperty::InsetBlockEnd(..) => LonghandId::InsetBlockEnd,
                DeclarationProperty::InsetBlockStart(..) => LonghandId::InsetBlockStart,
                DeclarationProperty::InsetInlineEnd(..) => LonghandId::InsetInlineEnd,
                DeclarationProperty::InsetInlineStart(..) => LonghandId::InsetInlineStart,
                DeclarationProperty::Left(..) => LonghandId::Left,
                DeclarationProperty::MarginBlockEnd(..) => LonghandId::MarginBlockEnd,
                DeclarationProperty::MarginBlockStart(..) => LonghandId::MarginBlockStart,
                DeclarationProperty::MarginBottom(..) => LonghandId::MarginBottom,
                DeclarationProperty::MarginInlineEnd(..) => LonghandId::MarginInlineEnd,
                DeclarationProperty::MarginInlineStart(..) => LonghandId::MarginInlineStart,
                DeclarationProperty::MarginLeft(..) => LonghandId::MarginLeft,
                DeclarationProperty::MarginRight(..) => LonghandId::MarginRight,
                DeclarationProperty::MarginTop(..) => LonghandId::MarginTop,
                DeclarationProperty::Right(..) => LonghandId::Right,
                DeclarationProperty::Top(..) => LonghandId::Top,
                _ => id,
            }
        );
        PropertyId::Longhand(id)
    }

    /// The `context` parameter controls this:
    ///
    /// <https://drafts.csswg.org/css-animations/#keyframes>
    /// > The <declaration-list> inside of <keyframe-block> accepts any CSS property
    /// > except those defined in this specification,
    /// > but does accept the `animation-play-state` property and interprets it specially.
    ///
    /// This will not actually parse Importance values, and will always set things
    /// to Importance::Normal. Parsing Importance values is the job of DeclarationPropertyParser,
    /// we only set them here so that we don't have to reallocate
    pub fn parse_into<'i, 't>(
        declarations: &mut SourcePropertyDeclaration,
        id: PropertyId,
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
        assert!(declarations.is_empty());
        debug_assert!(id.allowed_in(context), "{:?}", id);

        let non_custom_id = id.non_custom_id();
        let start = input.state();
        match id {
            PropertyId::Custom(property_name) => {
                todo!()
            },
            PropertyId::LonghandAlias(id, _) | PropertyId::Longhand(id) => {
                input.skip_whitespace(); // Unnecessary for correctness, but may help try() rewind less.
                input
                    .try_parse(CSSWideKeyword::parse)
                    .map(|keyword| DeclarationProperty::css_wide_keyword(id, keyword))
                    .or_else(|()| {
                        input.look_for_var_or_env_functions();
                        input.parse_entirely(|input| id.parse_value(context, input))
                    })
                    .map(|declaration| declarations.push(declaration))?;
            },
            PropertyId::ShorthandAlias(id, _) | PropertyId::Shorthand(id) => {
                input.skip_whitespace(); // Unnecessary for correctness, but may help try() rewind less.
                if let Ok(keyword) = input.try_parse(CSSWideKeyword::parse) {
                    if id == ShorthandId::All {
                        todo!()
                    } else {
                        for longhand in id.longhands() {
                            declarations
                                .push(DeclarationProperty::css_wide_keyword(longhand, keyword));
                        }
                    }
                } else {
                    input.look_for_var_or_env_functions();
                    // Not using parse_entirely here: each
                    // all::parse_into function needs to do so
                    // *before* pushing to `declarations`.
                    id.parse_into(declarations, context, input);
                }
            },
        }
        debug_assert!(
            non_custom_id.is_some(),
            "Custom properties should've returned earlier"
        );
        Ok(())
    }
}

/// A declaration [importance][importance].
///
/// [importance]: https://drafts.csswg.org/css-cascade/#importance
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Importance {
    /// Indicates a declaration without `!important`.
    Normal,

    /// Indicates a declaration with `!important`.
    Important,
}

impl Importance {
    /// Return whether this is an important declaration.
    pub fn important(self) -> bool {
        match self {
            Importance::Normal => false,
            Importance::Important => true,
        }
    }
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

#[derive(Clone, PartialEq)]
pub struct WideKeywordDeclaration {
    id: LonghandId,
    /// The CSS-wide keyword.
    pub keyword: CSSWideKeyword,
}
