use cssparser::{Parser, ToCss};

use crate::parser::ParseError;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::properties::longhand_id::LonghandId;
use crate::properties::property_id::{CSSWideKeyword, PropertyId};
use crate::properties::shorthand_id::ShorthandId;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::Integer;
use crate::{properties, values};

#[derive(Clone)]
pub enum PropertyDeclaration {
    /// `align-content`
    AlignContent(properties::longhands::align_content::AlignContent),
    /// `align-items`
    AlignItems(properties::longhands::align_items::AlignItems),
    /// `align-self`
    AlignSelf(properties::longhands::align_self::AlignSelf),
    /// `aspect-ratio`
    AspectRatio(properties::longhands::aspect_ratio::AspectRatio),
    /// `backface-visibility`
    BackfaceVisibility(properties::longhands::backface_visibility::BackfaceVisibility),
    /// `border-collapse`
    BorderCollapse(properties::longhands::border_collapse::BorderCollapse),
    /// `border-image-repeat`
    BorderImageRepeat(properties::longhands::border_image_repeat::BorderImageRepeat),
    /// `box-sizing`
    BoxSizing(properties::longhands::box_sizing::BoxSizing),
    /// `caption-side`
    CaptionSide(properties::longhands::caption_side::CaptionSide),
    /// `clear`
    Clear(properties::longhands::clear::Clear),
    /// `column-count`
    ColumnCount(properties::longhands::column_count::ColumnCount),
    /// `direction`
    Direction(properties::longhands::direction::Direction),
    /// `display`
    Display(properties::longhands::display::Display),
    /// `empty-cells`
    EmptyCells(properties::longhands::empty_cells::EmptyCells),
    /// `flex-direction`
    FlexDirection(properties::longhands::flex_direction::FlexDirection),
    /// `flex-wrap`
    FlexWrap(properties::longhands::flex_wrap::FlexWrap),
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
    ImageRendering(properties::longhands::image_rendering::ImageRendering),
    /// `justify-content`
    JustifyContent(properties::longhands::justify_content::JustifyContent),
    /// `list-style-position`
    ListStylePosition(properties::longhands::list_style_position::ListStylePosition),
    /// `list-style-type`
    ListStyleType(properties::longhands::list_style_type::ListStyleType),
    /// `mix-blend-mode`
    MixBlendMode(properties::longhands::mix_blend_mode::MixBlendMode),
    /// `opacity`
    Opacity(properties::longhands::opacity::Opacity),
    /// `order`
    Order(Integer),
    /// `outline-style`
    OutlineStyle(properties::longhands::outline_style::OutlineStyle),
    /// `overflow-wrap`
    OverflowWrap(properties::longhands::overflow_wrap::OverflowWrap),
    /// `pointer-events`
    PointerEvents(properties::longhands::pointer_events::PointerEvents),
    /// `position`
    Position(properties::longhands::position::Position),
    /// `table-layout`
    TableLayout(properties::longhands::table_layout::TableLayout),
    /// `text-align`
    TextAlign(properties::longhands::text_align::TextAlign),
    /// `text-decoration-line`
    TextDecorationLine(properties::longhands::text_decoration_line::TextDecorationLine),
    /// `text-justify`
    TextJustify(properties::longhands::text_justify::TextJustify),
    /// `text-rendering`
    TextRendering(properties::longhands::text_rendering::TextRendering),
    /// `text-transform`
    TextTransform(properties::longhands::text_transform::TextTransform),
    /// `transform-style`
    TransformStyle(properties::longhands::transform_style::TransformStyle),
    /// `unicode-bidi`
    UnicodeBidi(properties::longhands::unicode_bidi::UnicodeBidi),
    /// `visibility`
    Visibility(properties::longhands::visibility::Visibility),
    /// `white-space`
    WhiteSpace(properties::longhands::white_space::WhiteSpace),
    /// `word-break`
    WordBreak(properties::longhands::word_break::WordBreak),
    /// `writing-mode`
    WritingMode(properties::longhands::writing_mode::WritingMode),
    /// `z-index`
    ZIndex(values::number::IntegerAuto),
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
    BorderBlockEndStyle(values::layout::LineStyle),
    /// `border-block-start-style`
    BorderBlockStartStyle(values::layout::LineStyle),
    /// `border-bottom-style`
    BorderBottomStyle(values::layout::LineStyle),
    /// `border-inline-end-style`
    BorderInlineEndStyle(values::layout::LineStyle),
    /// `border-inline-start-style`
    BorderInlineStartStyle(values::layout::LineStyle),
    /// `border-left-style`
    BorderLeftStyle(values::layout::LineStyle),
    /// `border-right-style`
    BorderRightStyle(values::layout::LineStyle),
    /// `border-top-style`
    BorderTopStyle(values::layout::LineStyle),
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
    AnimationTimingFunction(
        properties::longhands::animation_timing_function::AnimationTimingFunction,
    ),
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
    /// `border-image-outset`
    BorderImageOutset(properties::longhands::border_image_outset::BorderImageOutset),
    /// `border-image-slice`
    BorderImageSlice(properties::longhands::border_image_slice::BorderImageSlice),
    /// `border-image-width`
    BorderImageWidth(properties::longhands::border_image_width::BorderImageWidth),
    /// `border-spacing`
    BorderSpacing(properties::longhands::border_spacing::BorderSpacing),
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
    /// `counter-increment`
    CounterIncrement(values::specified::counter::CounterWithInteger),
    /// `counter-reset`
    CounterReset(properties::longhands::counter_reset::CounterReset),
    /// `counter-set`
    CounterSet(values::specified::counter::CounterWithInteger),
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
    LetterSpacing(values::length::LengthPercentageOrNormal),
    /// `line-height`
    LineHeight(properties::longhands::line_height::LineHeight),
    /// `outline-offset`
    OutlineOffset(values::length::Length),
    /// `perspective`
    Perspective(values::length::NonNegativeLengthOrNone),
    /// `perspective-origin`
    PerspectiveOrigin(values::specified::position::Position),
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
    TransitionTimingFunction(values::specified::easing::EasingFunction),
    /// `translate`
    Translate(properties::longhands::translate::Translate),
    /// `vertical-align`
    VerticalAlign(properties::longhands::vertical_align::VerticalAlign),
    /// `word-spacing`
    WordSpacing(values::length::LengthPercentageOrNormal),
    /// `border-image-source`
    BorderImageSource(values::image::Image),
    /// `list-style-image`
    ListStyleImage(values::image::Image),
    /// `max-block-size`
    MaxBlockSize(values::length::Size),
    /// `max-height`
    MaxHeight(values::length::Size),
    /// `max-inline-size`
    MaxInlineSize(values::length::Size),
    /// `max-width`
    MaxWidth(values::length::Size),
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
    BorderBlockEndWidth(values::specified::line::LineWidth),
    /// `border-block-start-width`
    BorderBlockStartWidth(values::specified::line::LineWidth),
    /// `border-bottom-width`
    BorderBottomWidth(values::specified::line::LineWidth),
    /// `border-inline-end-width`
    BorderInlineEndWidth(values::specified::line::LineWidth),
    /// `border-inline-start-width`
    BorderInlineStartWidth(values::specified::line::LineWidth),
    /// `border-left-width`
    BorderLeftWidth(values::specified::line::LineWidth),
    /// `border-right-width`
    BorderRightWidth(values::specified::line::LineWidth),
    /// `border-top-width`
    BorderTopWidth(values::specified::line::LineWidth),
    /// `outline-width`
    OutlineWidth(values::specified::line::LineWidth),
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

impl PropertyDeclaration {
    /// Returns a CSS-wide keyword declaration for a given property.
    #[inline]
    pub fn css_wide_keyword(id: LonghandId, keyword: CSSWideKeyword) -> Self {
        Self::CSSWideKeyword(WideKeywordDeclaration { id, keyword })
    }

    pub fn id(&self) -> PropertyId {
        match *self {
            PropertyDeclaration::CSSWideKeyword(ref declaration) => {
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
                PropertyDeclaration::AlignContent(..) => LonghandId::AlignContent,
                PropertyDeclaration::AlignItems(..) => LonghandId::AlignItems,
                PropertyDeclaration::AlignSelf(..) => LonghandId::AlignSelf,
                PropertyDeclaration::BackfaceVisibility(..) => LonghandId::BackfaceVisibility,
                PropertyDeclaration::BorderCollapse(..) => LonghandId::BorderCollapse,
                PropertyDeclaration::BorderImageRepeat(..) => LonghandId::BorderImageRepeat,
                PropertyDeclaration::BoxSizing(..) => LonghandId::BoxSizing,
                PropertyDeclaration::CaptionSide(..) => LonghandId::CaptionSide,
                PropertyDeclaration::Clear(..) => LonghandId::Clear,
                PropertyDeclaration::ColumnCount(..) => LonghandId::ColumnCount,
                PropertyDeclaration::Direction(..) => LonghandId::Direction,
                PropertyDeclaration::Display(..) => LonghandId::Display,
                PropertyDeclaration::EmptyCells(..) => LonghandId::EmptyCells,
                PropertyDeclaration::FlexDirection(..) => LonghandId::FlexDirection,
                PropertyDeclaration::FlexWrap(..) => LonghandId::FlexWrap,
                PropertyDeclaration::Float(..) => LonghandId::Float,
                PropertyDeclaration::FontStretch(..) => LonghandId::FontStretch,
                PropertyDeclaration::FontStyle(..) => LonghandId::FontStyle,
                PropertyDeclaration::FontVariantCaps(..) => LonghandId::FontVariantCaps,
                PropertyDeclaration::FontWeight(..) => LonghandId::FontWeight,
                PropertyDeclaration::ImageRendering(..) => LonghandId::ImageRendering,
                PropertyDeclaration::JustifyContent(..) => LonghandId::JustifyContent,
                PropertyDeclaration::ListStylePosition(..) => LonghandId::ListStylePosition,
                PropertyDeclaration::ListStyleType(..) => LonghandId::ListStyleType,
                PropertyDeclaration::MixBlendMode(..) => LonghandId::MixBlendMode,
                PropertyDeclaration::Opacity(..) => LonghandId::Opacity,
                PropertyDeclaration::Order(..) => LonghandId::Order,
                PropertyDeclaration::OutlineStyle(..) => LonghandId::OutlineStyle,
                PropertyDeclaration::OverflowWrap(..) => LonghandId::OverflowWrap,
                PropertyDeclaration::PointerEvents(..) => LonghandId::PointerEvents,
                PropertyDeclaration::Position(..) => LonghandId::Position,
                PropertyDeclaration::TableLayout(..) => LonghandId::TableLayout,
                PropertyDeclaration::TextAlign(..) => LonghandId::TextAlign,
                PropertyDeclaration::TextDecorationLine(..) => LonghandId::TextDecorationLine,
                PropertyDeclaration::TextJustify(..) => LonghandId::TextJustify,
                PropertyDeclaration::TextRendering(..) => LonghandId::TextRendering,
                PropertyDeclaration::TextTransform(..) => LonghandId::TextTransform,
                PropertyDeclaration::TransformStyle(..) => LonghandId::TransformStyle,
                PropertyDeclaration::UnicodeBidi(..) => LonghandId::UnicodeBidi,
                PropertyDeclaration::Visibility(..) => LonghandId::Visibility,
                PropertyDeclaration::WhiteSpace(..) => LonghandId::WhiteSpace,
                PropertyDeclaration::WordBreak(..) => LonghandId::WordBreak,
                PropertyDeclaration::WritingMode(..) => LonghandId::WritingMode,
                PropertyDeclaration::ZIndex(..) => LonghandId::ZIndex,
                PropertyDeclaration::FlexGrow(..) => LonghandId::FlexGrow,
                PropertyDeclaration::FlexShrink(..) => LonghandId::FlexShrink,
                PropertyDeclaration::OverflowBlock(..) => LonghandId::OverflowBlock,
                PropertyDeclaration::OverflowInline(..) => LonghandId::OverflowInline,
                PropertyDeclaration::OverflowX(..) => LonghandId::OverflowX,
                PropertyDeclaration::OverflowY(..) => LonghandId::OverflowY,
                PropertyDeclaration::BorderBlockEndStyle(..) => LonghandId::BorderBlockEndStyle,
                PropertyDeclaration::BorderBlockStartStyle(..) => LonghandId::BorderBlockStartStyle,
                PropertyDeclaration::BorderBottomStyle(..) => LonghandId::BorderBottomStyle,
                PropertyDeclaration::BorderInlineEndStyle(..) => LonghandId::BorderInlineEndStyle,
                PropertyDeclaration::BorderInlineStartStyle(..) =>
                    LonghandId::BorderInlineStartStyle,
                PropertyDeclaration::BorderLeftStyle(..) => LonghandId::BorderLeftStyle,
                PropertyDeclaration::BorderRightStyle(..) => LonghandId::BorderRightStyle,
                PropertyDeclaration::BorderTopStyle(..) => LonghandId::BorderTopStyle,
                PropertyDeclaration::AnimationDelay(..) => LonghandId::AnimationDelay,
                PropertyDeclaration::AnimationDirection(..) => LonghandId::AnimationDirection,
                PropertyDeclaration::AnimationDuration(..) => LonghandId::AnimationDuration,
                PropertyDeclaration::AnimationFillMode(..) => LonghandId::AnimationFillMode,
                PropertyDeclaration::AnimationIterationCount(..) =>
                    LonghandId::AnimationIterationCount,
                PropertyDeclaration::AnimationName(..) => LonghandId::AnimationName,
                PropertyDeclaration::AnimationPlayState(..) => LonghandId::AnimationPlayState,
                PropertyDeclaration::AnimationTimingFunction(..) =>
                    LonghandId::AnimationTimingFunction,
                PropertyDeclaration::BackgroundAttachment(..) => LonghandId::BackgroundAttachment,
                PropertyDeclaration::BackgroundClip(..) => LonghandId::BackgroundClip,
                PropertyDeclaration::BackgroundImage(..) => LonghandId::BackgroundImage,
                PropertyDeclaration::BackgroundOrigin(..) => LonghandId::BackgroundOrigin,
                PropertyDeclaration::BackgroundPositionX(..) => LonghandId::BackgroundPositionX,
                PropertyDeclaration::BackgroundPositionY(..) => LonghandId::BackgroundPositionY,
                PropertyDeclaration::BackgroundRepeat(..) => LonghandId::BackgroundRepeat,
                PropertyDeclaration::BackgroundSize(..) => LonghandId::BackgroundSize,
                PropertyDeclaration::BoxShadow(..) => LonghandId::BoxShadow,
                PropertyDeclaration::Clip(..) => LonghandId::Clip,
                PropertyDeclaration::Color(..) => LonghandId::Color,
                PropertyDeclaration::ColumnGap(..) => LonghandId::ColumnGap,
                PropertyDeclaration::ColumnWidth(..) => LonghandId::ColumnWidth,
                PropertyDeclaration::Content(..) => LonghandId::Content,
                PropertyDeclaration::Cursor(..) => LonghandId::Cursor,
                PropertyDeclaration::Filter(..) => LonghandId::Filter,
                PropertyDeclaration::FlexBasis(..) => LonghandId::FlexBasis,
                PropertyDeclaration::FontFamily(..) => LonghandId::FontFamily,
                PropertyDeclaration::FontSize(..) => LonghandId::FontSize,
                PropertyDeclaration::LetterSpacing(..) => LonghandId::LetterSpacing,
                PropertyDeclaration::LineHeight(..) => LonghandId::LineHeight,
                PropertyDeclaration::OutlineOffset(..) => LonghandId::OutlineOffset,
                PropertyDeclaration::Perspective(..) => LonghandId::Perspective,
                PropertyDeclaration::PerspectiveOrigin(..) => LonghandId::PerspectiveOrigin,
                PropertyDeclaration::Quotes(..) => LonghandId::Quotes,
                PropertyDeclaration::Rotate(..) => LonghandId::Rotate,
                PropertyDeclaration::Scale(..) => LonghandId::Scale,
                PropertyDeclaration::TextIndent(..) => LonghandId::TextIndent,
                PropertyDeclaration::TextOverflow(..) => LonghandId::TextOverflow,
                PropertyDeclaration::TextShadow(..) => LonghandId::TextShadow,
                PropertyDeclaration::Transform(..) => LonghandId::Transform,
                PropertyDeclaration::TransformOrigin(..) => LonghandId::TransformOrigin,
                PropertyDeclaration::TransitionDelay(..) => LonghandId::TransitionDelay,
                PropertyDeclaration::TransitionDuration(..) => LonghandId::TransitionDuration,
                PropertyDeclaration::TransitionProperty(..) => LonghandId::TransitionProperty,
                PropertyDeclaration::TransitionTimingFunction(..) =>
                    LonghandId::TransitionTimingFunction,
                PropertyDeclaration::Translate(..) => LonghandId::Translate,
                PropertyDeclaration::VerticalAlign(..) => LonghandId::VerticalAlign,
                PropertyDeclaration::WordSpacing(..) => LonghandId::WordSpacing,
                PropertyDeclaration::BorderImageSource(..) => LonghandId::BorderImageSource,
                PropertyDeclaration::ListStyleImage(..) => LonghandId::ListStyleImage,
                PropertyDeclaration::MaxBlockSize(..) => LonghandId::MaxBlockSize,
                PropertyDeclaration::MaxHeight(..) => LonghandId::MaxHeight,
                PropertyDeclaration::MaxInlineSize(..) => LonghandId::MaxInlineSize,
                PropertyDeclaration::MaxWidth(..) => LonghandId::MaxWidth,
                PropertyDeclaration::BorderBottomLeftRadius(..) =>
                    LonghandId::BorderBottomLeftRadius,
                PropertyDeclaration::BorderBottomRightRadius(..) =>
                    LonghandId::BorderBottomRightRadius,
                PropertyDeclaration::BorderEndEndRadius(..) => LonghandId::BorderEndEndRadius,
                PropertyDeclaration::BorderEndStartRadius(..) => LonghandId::BorderEndStartRadius,
                PropertyDeclaration::BorderStartEndRadius(..) => LonghandId::BorderStartEndRadius,
                PropertyDeclaration::BorderStartStartRadius(..) =>
                    LonghandId::BorderStartStartRadius,
                PropertyDeclaration::BorderTopLeftRadius(..) => LonghandId::BorderTopLeftRadius,
                PropertyDeclaration::BorderTopRightRadius(..) => LonghandId::BorderTopRightRadius,
                PropertyDeclaration::PaddingBlockEnd(..) => LonghandId::PaddingBlockEnd,
                PropertyDeclaration::PaddingBlockStart(..) => LonghandId::PaddingBlockStart,
                PropertyDeclaration::PaddingBottom(..) => LonghandId::PaddingBottom,
                PropertyDeclaration::PaddingInlineEnd(..) => LonghandId::PaddingInlineEnd,
                PropertyDeclaration::PaddingInlineStart(..) => LonghandId::PaddingInlineStart,
                PropertyDeclaration::PaddingLeft(..) => LonghandId::PaddingLeft,
                PropertyDeclaration::PaddingRight(..) => LonghandId::PaddingRight,
                PropertyDeclaration::PaddingTop(..) => LonghandId::PaddingTop,
                PropertyDeclaration::BlockSize(..) => LonghandId::BlockSize,
                PropertyDeclaration::Height(..) => LonghandId::Height,
                PropertyDeclaration::InlineSize(..) => LonghandId::InlineSize,
                PropertyDeclaration::MinBlockSize(..) => LonghandId::MinBlockSize,
                PropertyDeclaration::MinHeight(..) => LonghandId::MinHeight,
                PropertyDeclaration::MinInlineSize(..) => LonghandId::MinInlineSize,
                PropertyDeclaration::MinWidth(..) => LonghandId::MinWidth,
                PropertyDeclaration::Width(..) => LonghandId::Width,
                PropertyDeclaration::BorderBlockEndWidth(..) => LonghandId::BorderBlockEndWidth,
                PropertyDeclaration::BorderBlockStartWidth(..) => LonghandId::BorderBlockStartWidth,
                PropertyDeclaration::BorderBottomWidth(..) => LonghandId::BorderBottomWidth,
                PropertyDeclaration::BorderInlineEndWidth(..) => LonghandId::BorderInlineEndWidth,
                PropertyDeclaration::BorderInlineStartWidth(..) =>
                    LonghandId::BorderInlineStartWidth,
                PropertyDeclaration::BorderLeftWidth(..) => LonghandId::BorderLeftWidth,
                PropertyDeclaration::BorderRightWidth(..) => LonghandId::BorderRightWidth,
                PropertyDeclaration::BorderTopWidth(..) => LonghandId::BorderTopWidth,
                PropertyDeclaration::OutlineWidth(..) => LonghandId::OutlineWidth,
                PropertyDeclaration::BackgroundColor(..) => LonghandId::BackgroundColor,
                PropertyDeclaration::BorderBlockEndColor(..) => LonghandId::BorderBlockEndColor,
                PropertyDeclaration::BorderBlockStartColor(..) => LonghandId::BorderBlockStartColor,
                PropertyDeclaration::BorderBottomColor(..) => LonghandId::BorderBottomColor,
                PropertyDeclaration::BorderInlineEndColor(..) => LonghandId::BorderInlineEndColor,
                PropertyDeclaration::BorderInlineStartColor(..) =>
                    LonghandId::BorderInlineStartColor,
                PropertyDeclaration::BorderLeftColor(..) => LonghandId::BorderLeftColor,
                PropertyDeclaration::BorderRightColor(..) => LonghandId::BorderRightColor,
                PropertyDeclaration::BorderTopColor(..) => LonghandId::BorderTopColor,
                PropertyDeclaration::OutlineColor(..) => LonghandId::OutlineColor,
                PropertyDeclaration::Bottom(..) => LonghandId::Bottom,
                PropertyDeclaration::InsetBlockEnd(..) => LonghandId::InsetBlockEnd,
                PropertyDeclaration::InsetBlockStart(..) => LonghandId::InsetBlockStart,
                PropertyDeclaration::InsetInlineEnd(..) => LonghandId::InsetInlineEnd,
                PropertyDeclaration::InsetInlineStart(..) => LonghandId::InsetInlineStart,
                PropertyDeclaration::Left(..) => LonghandId::Left,
                PropertyDeclaration::MarginBlockEnd(..) => LonghandId::MarginBlockEnd,
                PropertyDeclaration::MarginBlockStart(..) => LonghandId::MarginBlockStart,
                PropertyDeclaration::MarginBottom(..) => LonghandId::MarginBottom,
                PropertyDeclaration::MarginInlineEnd(..) => LonghandId::MarginInlineEnd,
                PropertyDeclaration::MarginInlineStart(..) => LonghandId::MarginInlineStart,
                PropertyDeclaration::MarginLeft(..) => LonghandId::MarginLeft,
                PropertyDeclaration::MarginRight(..) => LonghandId::MarginRight,
                PropertyDeclaration::MarginTop(..) => LonghandId::MarginTop,
                PropertyDeclaration::Right(..) => LonghandId::Right,
                PropertyDeclaration::Top(..) => LonghandId::Top,
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
    ) -> Result<(), ParseError<'i>> {
        assert!(declarations.is_empty());
        debug_assert!(id.allowed_in(context), "{:?}", id);

        let non_custom_id = id.non_custom_id();
        match id {
            PropertyId::Custom(property_name) => {
                todo!()
            },
            PropertyId::Longhand(id) => {
                input.skip_whitespace(); // Unnecessary for correctness, but may help try() rewind less.
                input
                    .try_parse(CSSWideKeyword::parse)
                    .map(|keyword| PropertyDeclaration::css_wide_keyword(id, keyword))
                    .or_else(|()| {
                        input.look_for_var_or_env_functions();
                        input.parse_entirely(|input| id.parse_value(context, input))
                    })
                    .map(|declaration| declarations.push(declaration))?;
            },
            PropertyId::Shorthand(id) => {
                input.skip_whitespace(); // Unnecessary for correctness, but may help try() rewind less.
                if let Ok(keyword) = input.try_parse(CSSWideKeyword::parse) {
                    if id == ShorthandId::All {
                        todo!()
                    } else {
                        for longhand in id.longhands() {
                            declarations
                                .push(PropertyDeclaration::css_wide_keyword(longhand, keyword));
                        }
                    }
                } else {
                    input.look_for_var_or_env_functions();
                    // Not using parse_entirely here: each
                    // all::parse_into function needs to do so
                    // *before* pushing to `declarations`.
                    id.parse_into(declarations, context, input).or_else(|err| {
                        while let Ok(_) = input.next() {} // Look for var() after the error.
                        if !input.seen_var_or_env_functions() {
                            return Err(err);
                        }

                        todo!()
                    })?;
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

impl ToCss for PropertyDeclaration {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: std::fmt::Write,
    {
        match self {
            PropertyDeclaration::AlignContent(property) => property.to_css(dest),
            PropertyDeclaration::AlignItems(property) => property.to_css(dest),
            PropertyDeclaration::AlignSelf(_) => todo!(),
            PropertyDeclaration::AspectRatio(_) => todo!(),
            PropertyDeclaration::BackfaceVisibility(_) => todo!(),
            PropertyDeclaration::BorderCollapse(_) => todo!(),
            PropertyDeclaration::BorderImageRepeat(_) => todo!(),
            PropertyDeclaration::BoxSizing(_) => todo!(),
            PropertyDeclaration::CaptionSide(_) => todo!(),
            PropertyDeclaration::Clear(_) => todo!(),
            PropertyDeclaration::ColumnCount(_) => todo!(),
            PropertyDeclaration::Direction(_) => todo!(),
            PropertyDeclaration::Display(display) => display.to_css(dest),
            PropertyDeclaration::EmptyCells(_) => todo!(),
            PropertyDeclaration::FlexDirection(_) => todo!(),
            PropertyDeclaration::FlexWrap(_) => todo!(),
            PropertyDeclaration::Float(_) => todo!(),
            PropertyDeclaration::FontStretch(_) => todo!(),
            PropertyDeclaration::FontStyle(_) => todo!(),
            PropertyDeclaration::FontVariantCaps(_) => todo!(),
            PropertyDeclaration::FontWeight(_) => todo!(),
            PropertyDeclaration::ImageRendering(_) => todo!(),
            PropertyDeclaration::JustifyContent(_) => todo!(),
            PropertyDeclaration::ListStylePosition(_) => todo!(),
            PropertyDeclaration::ListStyleType(_) => todo!(),
            PropertyDeclaration::MixBlendMode(_) => todo!(),
            PropertyDeclaration::Opacity(_) => todo!(),
            PropertyDeclaration::Order(_) => todo!(),
            PropertyDeclaration::OutlineStyle(_) => todo!(),
            PropertyDeclaration::OverflowWrap(_) => todo!(),
            PropertyDeclaration::PointerEvents(_) => todo!(),
            PropertyDeclaration::Position(_) => todo!(),
            PropertyDeclaration::TableLayout(_) => todo!(),
            PropertyDeclaration::TextAlign(_) => todo!(),
            PropertyDeclaration::TextDecorationLine(_) => todo!(),
            PropertyDeclaration::TextJustify(_) => todo!(),
            PropertyDeclaration::TextRendering(_) => todo!(),
            PropertyDeclaration::TextTransform(_) => todo!(),
            PropertyDeclaration::TransformStyle(_) => todo!(),
            PropertyDeclaration::UnicodeBidi(_) => todo!(),
            PropertyDeclaration::Visibility(_) => todo!(),
            PropertyDeclaration::WhiteSpace(_) => todo!(),
            PropertyDeclaration::WordBreak(_) => todo!(),
            PropertyDeclaration::WritingMode(_) => todo!(),
            PropertyDeclaration::ZIndex(_) => todo!(),
            PropertyDeclaration::FlexGrow(_) => todo!(),
            PropertyDeclaration::FlexShrink(_) => todo!(),
            PropertyDeclaration::OverflowBlock(_) => todo!(),
            PropertyDeclaration::OverflowInline(_) => todo!(),
            PropertyDeclaration::OverflowX(_) => todo!(),
            PropertyDeclaration::OverflowY(_) => todo!(),
            PropertyDeclaration::BorderBlockEndStyle(_) => todo!(),
            PropertyDeclaration::BorderBlockStartStyle(_) => todo!(),
            PropertyDeclaration::BorderBottomStyle(_) => todo!(),
            PropertyDeclaration::BorderInlineEndStyle(_) => todo!(),
            PropertyDeclaration::BorderInlineStartStyle(_) => todo!(),
            PropertyDeclaration::BorderLeftStyle(_) => todo!(),
            PropertyDeclaration::BorderRightStyle(_) => todo!(),
            PropertyDeclaration::BorderTopStyle(_) => todo!(),
            PropertyDeclaration::AnimationDelay(_) => todo!(),
            PropertyDeclaration::AnimationDirection(_) => todo!(),
            PropertyDeclaration::AnimationDuration(_) => todo!(),
            PropertyDeclaration::AnimationFillMode(_) => todo!(),
            PropertyDeclaration::AnimationIterationCount(_) => todo!(),
            PropertyDeclaration::AnimationName(_) => todo!(),
            PropertyDeclaration::AnimationPlayState(_) => todo!(),
            PropertyDeclaration::AnimationTimingFunction(_) => todo!(),
            PropertyDeclaration::BackgroundAttachment(_) => todo!(),
            PropertyDeclaration::BackgroundClip(_) => todo!(),
            PropertyDeclaration::BackgroundImage(_) => todo!(),
            PropertyDeclaration::BackgroundOrigin(_) => todo!(),
            PropertyDeclaration::BackgroundPositionX(_) => todo!(),
            PropertyDeclaration::BackgroundPositionY(_) => todo!(),
            PropertyDeclaration::BackgroundRepeat(_) => todo!(),
            PropertyDeclaration::BackgroundSize(_) => todo!(),
            PropertyDeclaration::BorderImageOutset(_) => todo!(),
            PropertyDeclaration::BorderImageSlice(_) => todo!(),
            PropertyDeclaration::BorderImageWidth(_) => todo!(),
            PropertyDeclaration::BorderSpacing(_) => todo!(),
            PropertyDeclaration::BoxShadow(_) => todo!(),
            PropertyDeclaration::Clip(_) => todo!(),
            PropertyDeclaration::Color(_) => todo!(),
            PropertyDeclaration::ColumnGap(_) => todo!(),
            PropertyDeclaration::ColumnWidth(_) => todo!(),
            PropertyDeclaration::Content(_) => todo!(),
            PropertyDeclaration::CounterIncrement(_) => todo!(),
            PropertyDeclaration::CounterReset(_) => todo!(),
            PropertyDeclaration::CounterSet(_) => todo!(),
            PropertyDeclaration::Cursor(_) => todo!(),
            PropertyDeclaration::Filter(_) => todo!(),
            PropertyDeclaration::FlexBasis(_) => todo!(),
            PropertyDeclaration::FontFamily(_) => todo!(),
            PropertyDeclaration::FontSize(_) => todo!(),
            PropertyDeclaration::LetterSpacing(_) => todo!(),
            PropertyDeclaration::LineHeight(_) => todo!(),
            PropertyDeclaration::OutlineOffset(_) => todo!(),
            PropertyDeclaration::Perspective(_) => todo!(),
            PropertyDeclaration::PerspectiveOrigin(_) => todo!(),
            PropertyDeclaration::Quotes(_) => todo!(),
            PropertyDeclaration::Rotate(_) => todo!(),
            PropertyDeclaration::Scale(_) => todo!(),
            PropertyDeclaration::TextIndent(_) => todo!(),
            PropertyDeclaration::TextOverflow(_) => todo!(),
            PropertyDeclaration::TextShadow(_) => todo!(),
            PropertyDeclaration::Transform(_) => todo!(),
            PropertyDeclaration::TransformOrigin(_) => todo!(),
            PropertyDeclaration::TransitionDelay(_) => todo!(),
            PropertyDeclaration::TransitionDuration(_) => todo!(),
            PropertyDeclaration::TransitionProperty(_) => todo!(),
            PropertyDeclaration::TransitionTimingFunction(_) => todo!(),
            PropertyDeclaration::Translate(_) => todo!(),
            PropertyDeclaration::VerticalAlign(_) => todo!(),
            PropertyDeclaration::WordSpacing(_) => todo!(),
            PropertyDeclaration::BorderImageSource(_) => todo!(),
            PropertyDeclaration::ListStyleImage(_) => todo!(),
            PropertyDeclaration::MaxBlockSize(_) => todo!(),
            PropertyDeclaration::MaxHeight(_) => todo!(),
            PropertyDeclaration::MaxInlineSize(_) => todo!(),
            PropertyDeclaration::MaxWidth(_) => todo!(),
            PropertyDeclaration::BorderBottomLeftRadius(_) => todo!(),
            PropertyDeclaration::BorderBottomRightRadius(_) => todo!(),
            PropertyDeclaration::BorderEndEndRadius(_) => todo!(),
            PropertyDeclaration::BorderEndStartRadius(_) => todo!(),
            PropertyDeclaration::BorderStartEndRadius(_) => todo!(),
            PropertyDeclaration::BorderStartStartRadius(_) => todo!(),
            PropertyDeclaration::BorderTopLeftRadius(_) => todo!(),
            PropertyDeclaration::BorderTopRightRadius(_) => todo!(),
            PropertyDeclaration::PaddingBlockEnd(_) => todo!(),
            PropertyDeclaration::PaddingBlockStart(_) => todo!(),
            PropertyDeclaration::PaddingBottom(_) => todo!(),
            PropertyDeclaration::PaddingInlineEnd(_) => todo!(),
            PropertyDeclaration::PaddingInlineStart(_) => todo!(),
            PropertyDeclaration::PaddingLeft(_) => todo!(),
            PropertyDeclaration::PaddingRight(_) => todo!(),
            PropertyDeclaration::PaddingTop(_) => todo!(),
            PropertyDeclaration::BlockSize(_) => todo!(),
            PropertyDeclaration::Height(_) => todo!(),
            PropertyDeclaration::InlineSize(_) => todo!(),
            PropertyDeclaration::MinBlockSize(_) => todo!(),
            PropertyDeclaration::MinHeight(_) => todo!(),
            PropertyDeclaration::MinInlineSize(_) => todo!(),
            PropertyDeclaration::MinWidth(_) => todo!(),
            PropertyDeclaration::Width(_) => todo!(),
            PropertyDeclaration::BorderBlockEndWidth(_) => todo!(),
            PropertyDeclaration::BorderBlockStartWidth(_) => todo!(),
            PropertyDeclaration::BorderBottomWidth(_) => todo!(),
            PropertyDeclaration::BorderInlineEndWidth(_) => todo!(),
            PropertyDeclaration::BorderInlineStartWidth(_) => todo!(),
            PropertyDeclaration::BorderLeftWidth(_) => todo!(),
            PropertyDeclaration::BorderRightWidth(_) => todo!(),
            PropertyDeclaration::BorderTopWidth(_) => todo!(),
            PropertyDeclaration::OutlineWidth(_) => todo!(),
            PropertyDeclaration::BackgroundColor(_) => todo!(),
            PropertyDeclaration::BorderBlockEndColor(_) => todo!(),
            PropertyDeclaration::BorderBlockStartColor(_) => todo!(),
            PropertyDeclaration::BorderBottomColor(_) => todo!(),
            PropertyDeclaration::BorderInlineEndColor(_) => todo!(),
            PropertyDeclaration::BorderInlineStartColor(_) => todo!(),
            PropertyDeclaration::BorderLeftColor(_) => todo!(),
            PropertyDeclaration::BorderRightColor(_) => todo!(),
            PropertyDeclaration::BorderTopColor(_) => todo!(),
            PropertyDeclaration::OutlineColor(_) => todo!(),
            PropertyDeclaration::Bottom(_) => todo!(),
            PropertyDeclaration::InsetBlockEnd(_) => todo!(),
            PropertyDeclaration::InsetBlockStart(_) => todo!(),
            PropertyDeclaration::InsetInlineEnd(_) => todo!(),
            PropertyDeclaration::InsetInlineStart(_) => todo!(),
            PropertyDeclaration::Left(_) => todo!(),
            PropertyDeclaration::MarginBlockEnd(_) => todo!(),
            PropertyDeclaration::MarginBlockStart(_) => todo!(),
            PropertyDeclaration::MarginBottom(_) => todo!(),
            PropertyDeclaration::MarginInlineEnd(_) => todo!(),
            PropertyDeclaration::MarginInlineStart(_) => todo!(),
            PropertyDeclaration::MarginLeft(_) => todo!(),
            PropertyDeclaration::MarginRight(_) => todo!(),
            PropertyDeclaration::MarginTop(_) => todo!(),
            PropertyDeclaration::Right(_) => todo!(),
            PropertyDeclaration::Top(_) => todo!(),
            PropertyDeclaration::CSSWideKeyword(_) => todo!(),
        }
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

#[derive(Clone, PartialEq)]
pub struct WideKeywordDeclaration {
    id: LonghandId,
    /// The CSS-wide keyword.
    pub keyword: CSSWideKeyword,
}

macro_rules! property_keywords_impl {
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
                            _ => Err(input.new_custom_error(StyleParseErrorKind::MediaQueryExpectedFeatureValue)),
                        }
                    },
                    _ =>     Err(input.new_custom_error(StyleParseErrorKind::MediaQueryExpectedFeatureValue)),
                }
            }
        }

        impl ToCss for $input {
            fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
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
pub(crate) use property_keywords_impl;
