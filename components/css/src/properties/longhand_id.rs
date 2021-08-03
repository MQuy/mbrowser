use core::fmt;
use std::fmt::Write;

use crate::css_writer::{CssWriter, ToCss};
use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use cssparser::Parser;

use crate::properties::longhands;
use crate::properties::shorthand_id::ShorthandId;
use crate::stylesheets::stylesheet::ParserContext;

use super::property_id::{NonCustomPropertyId, NonCustomPropertyIterator};

/// An identifier for a given longhand property.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
#[repr(u16)]
pub enum LonghandId {
    /// align-content
    AlignContent = 0,
    /// align-items
    AlignItems = 1,
    /// align-self
    AlignSelf = 2,
    /// aspect-ratio
    AspectRatio = 3,
    /// backface-visibility
    BackfaceVisibility = 4,
    /// border-collapse
    BorderCollapse = 5,
    /// border-image-repeat
    BorderImageRepeat = 6,
    /// box-sizing
    BoxSizing = 7,
    /// caption-side
    CaptionSide = 8,
    /// clear
    Clear = 9,
    /// column-count
    ColumnCount = 10,
    /// direction
    Direction = 11,
    /// display
    Display = 12,
    /// empty-cells
    EmptyCells = 13,
    /// flex-direction
    FlexDirection = 14,
    /// flex-wrap
    FlexWrap = 15,
    /// float
    Float = 16,
    /// font-stretch
    FontStretch = 17,
    /// font-style
    FontStyle = 18,
    /// font-variant-caps
    FontVariantCaps = 19,
    /// font-weight
    FontWeight = 20,
    /// image-rendering
    ImageRendering = 21,
    /// justify-content
    JustifyContent = 22,
    /// list-style-position
    ListStylePosition = 23,
    /// list-style-type
    ListStyleType = 24,
    /// mix-blend-mode
    MixBlendMode = 25,
    /// opacity
    Opacity = 26,
    /// order
    Order = 27,
    /// outline-style
    OutlineStyle = 28,
    /// overflow-wrap
    OverflowWrap = 29,
    /// pointer-events
    PointerEvents = 30,
    /// position
    Position = 31,
    /// -servo-overflow-clip-box
    ServoOverflowClipBox = 32,
    /// -servo-top-layer
    ServoTopLayer = 33,
    /// table-layout
    TableLayout = 34,
    /// text-align
    TextAlign = 35,
    /// text-decoration-line
    TextDecorationLine = 36,
    /// text-justify
    TextJustify = 37,
    /// text-rendering
    TextRendering = 38,
    /// text-transform
    TextTransform = 39,
    /// transform-style
    TransformStyle = 40,
    /// unicode-bidi
    UnicodeBidi = 41,
    /// visibility
    Visibility = 42,
    /// white-space
    WhiteSpace = 43,
    /// word-break
    WordBreak = 44,
    /// writing-mode
    WritingMode = 45,
    /// z-index
    ZIndex = 46,
    /// flex-grow
    FlexGrow = 47,
    /// flex-shrink
    FlexShrink = 48,
    /// overflow-block
    OverflowBlock = 49,
    /// overflow-inline
    OverflowInline = 50,
    /// overflow-x
    OverflowX = 51,
    /// overflow-y
    OverflowY = 52,
    /// border-block-end-style
    BorderBlockEndStyle = 53,
    /// border-block-start-style
    BorderBlockStartStyle = 54,
    /// border-bottom-style
    BorderBottomStyle = 55,
    /// border-inline-end-style
    BorderInlineEndStyle = 56,
    /// border-inline-start-style
    BorderInlineStartStyle = 57,
    /// border-left-style
    BorderLeftStyle = 58,
    /// border-right-style
    BorderRightStyle = 59,
    /// border-top-style
    BorderTopStyle = 60,
    /// animation-delay
    AnimationDelay = 61,
    /// animation-direction
    AnimationDirection = 62,
    /// animation-duration
    AnimationDuration = 63,
    /// animation-fill-mode
    AnimationFillMode = 64,
    /// animation-iteration-count
    AnimationIterationCount = 65,
    /// animation-name
    AnimationName = 66,
    /// animation-play-state
    AnimationPlayState = 67,
    /// animation-timing-function
    AnimationTimingFunction = 68,
    /// background-attachment
    BackgroundAttachment = 69,
    /// background-clip
    BackgroundClip = 70,
    /// background-image
    BackgroundImage = 71,
    /// background-origin
    BackgroundOrigin = 72,
    /// background-position-x
    BackgroundPositionX = 73,
    /// background-position-y
    BackgroundPositionY = 74,
    /// background-repeat
    BackgroundRepeat = 75,
    /// background-size
    BackgroundSize = 76,
    /// border-image-outset
    BorderImageOutset = 77,
    /// border-image-slice
    BorderImageSlice = 78,
    /// border-image-width
    BorderImageWidth = 79,
    /// border-spacing
    BorderSpacing = 80,
    /// box-shadow
    BoxShadow = 81,
    /// clip
    Clip = 82,
    /// color
    Color = 83,
    /// column-gap
    ColumnGap = 84,
    /// column-width
    ColumnWidth = 85,
    /// content
    Content = 86,
    /// counter-increment
    CounterIncrement = 87,
    /// counter-reset
    CounterReset = 88,
    /// cursor
    Cursor = 89,
    /// filter
    Filter = 90,
    /// flex-basis
    FlexBasis = 91,
    /// font-family
    FontFamily = 92,
    /// font-size
    FontSize = 93,
    /// letter-spacing
    LetterSpacing = 94,
    /// line-height
    LineHeight = 95,
    /// outline-offset
    OutlineOffset = 96,
    /// perspective
    Perspective = 97,
    /// perspective-origin
    PerspectiveOrigin = 98,
    /// quotes
    Quotes = 99,
    /// rotate
    Rotate = 100,
    /// scale
    Scale = 101,
    /// text-indent
    TextIndent = 102,
    /// text-overflow
    TextOverflow = 103,
    /// text-shadow
    TextShadow = 104,
    /// transform
    Transform = 105,
    /// transform-origin
    TransformOrigin = 106,
    /// transition-delay
    TransitionDelay = 107,
    /// transition-duration
    TransitionDuration = 108,
    /// transition-property
    TransitionProperty = 109,
    /// transition-timing-function
    TransitionTimingFunction = 110,
    /// translate
    Translate = 111,
    /// vertical-align
    VerticalAlign = 112,
    /// word-spacing
    WordSpacing = 113,
    /// border-image-source
    BorderImageSource = 114,
    /// list-style-image
    ListStyleImage = 115,
    /// max-block-size
    MaxBlockSize = 116,
    /// max-height
    MaxHeight = 117,
    /// max-inline-size
    MaxInlineSize = 118,
    /// max-width
    MaxWidth = 119,
    /// border-bottom-left-radius
    BorderBottomLeftRadius = 120,
    /// border-bottom-right-radius
    BorderBottomRightRadius = 121,
    /// border-end-end-radius
    BorderEndEndRadius = 122,
    /// border-end-start-radius
    BorderEndStartRadius = 123,
    /// border-start-end-radius
    BorderStartEndRadius = 124,
    /// border-start-start-radius
    BorderStartStartRadius = 125,
    /// border-top-left-radius
    BorderTopLeftRadius = 126,
    /// border-top-right-radius
    BorderTopRightRadius = 127,
    /// padding-block-end
    PaddingBlockEnd = 128,
    /// padding-block-start
    PaddingBlockStart = 129,
    /// padding-bottom
    PaddingBottom = 130,
    /// padding-inline-end
    PaddingInlineEnd = 131,
    /// padding-inline-start
    PaddingInlineStart = 132,
    /// padding-left
    PaddingLeft = 133,
    /// padding-right
    PaddingRight = 134,
    /// padding-top
    PaddingTop = 135,
    /// block-size
    BlockSize = 136,
    /// height
    Height = 137,
    /// inline-size
    InlineSize = 138,
    /// min-block-size
    MinBlockSize = 139,
    /// min-height
    MinHeight = 140,
    /// min-inline-size
    MinInlineSize = 141,
    /// min-width
    MinWidth = 142,
    /// width
    Width = 143,
    /// border-block-end-width
    BorderBlockEndWidth = 144,
    /// border-block-start-width
    BorderBlockStartWidth = 145,
    /// border-bottom-width
    BorderBottomWidth = 146,
    /// border-inline-end-width
    BorderInlineEndWidth = 147,
    /// border-inline-start-width
    BorderInlineStartWidth = 148,
    /// border-left-width
    BorderLeftWidth = 149,
    /// border-right-width
    BorderRightWidth = 150,
    /// border-top-width
    BorderTopWidth = 151,
    /// outline-width
    OutlineWidth = 152,
    /// background-color
    BackgroundColor = 153,
    /// border-block-end-color
    BorderBlockEndColor = 154,
    /// border-block-start-color
    BorderBlockStartColor = 155,
    /// border-bottom-color
    BorderBottomColor = 156,
    /// border-inline-end-color
    BorderInlineEndColor = 157,
    /// border-inline-start-color
    BorderInlineStartColor = 158,
    /// border-left-color
    BorderLeftColor = 159,
    /// border-right-color
    BorderRightColor = 160,
    /// border-top-color
    BorderTopColor = 161,
    /// outline-color
    OutlineColor = 162,
    /// bottom
    Bottom = 163,
    /// inset-block-end
    InsetBlockEnd = 164,
    /// inset-block-start
    InsetBlockStart = 165,
    /// inset-inline-end
    InsetInlineEnd = 166,
    /// inset-inline-start
    InsetInlineStart = 167,
    /// left
    Left = 168,
    /// margin-block-end
    MarginBlockEnd = 169,
    /// margin-block-start
    MarginBlockStart = 170,
    /// margin-bottom
    MarginBottom = 171,
    /// margin-inline-end
    MarginInlineEnd = 172,
    /// margin-inline-start
    MarginInlineStart = 173,
    /// margin-left
    MarginLeft = 174,
    /// margin-right
    MarginRight = 175,
    /// margin-top
    MarginTop = 176,
    /// right
    Right = 177,
    /// top
    Top = 178,
}

impl ToCss for LonghandId {
    #[inline]
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result
    where
        W: Write,
    {
        dest.write_str(self.name())
    }
}

impl fmt::Debug for LonghandId {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

impl LonghandId {
    /// Get the name of this longhand property.
    #[inline]
    pub fn name(&self) -> &'static str {
        NonCustomPropertyId::from(*self).name()
    }

    fn shorthands(&self) -> NonCustomPropertyIterator<ShorthandId> {
        // first generate longhand to shorthands lookup map
        //
        // NOTE(emilio): This currently doesn't exclude the "all" shorthand. It
        // could potentially do so, which would speed up serialization
        // algorithms and what not, I guess.

        // based on lookup results for each longhand, create result arrays
        static ALIGN_CONTENT: &'static [ShorthandId] = &[ShorthandId::All];
        static ALIGN_ITEMS: &'static [ShorthandId] = &[ShorthandId::All];
        static ALIGN_SELF: &'static [ShorthandId] = &[ShorthandId::All];
        static ASPECT_RATIO: &'static [ShorthandId] = &[ShorthandId::All];
        static BACKFACE_VISIBILITY: &'static [ShorthandId] = &[ShorthandId::All];
        static BORDER_COLLAPSE: &'static [ShorthandId] = &[ShorthandId::All];
        static BORDER_IMAGE_REPEAT: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderImage,
        ];
        static BOX_SIZING: &'static [ShorthandId] = &[ShorthandId::All];
        static CAPTION_SIDE: &'static [ShorthandId] = &[ShorthandId::All];
        static CLEAR: &'static [ShorthandId] = &[ShorthandId::All];
        static COLUMN_COUNT: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Columns];
        static DIRECTION: &'static [ShorthandId] = &[];
        static DISPLAY: &'static [ShorthandId] = &[ShorthandId::All];
        static EMPTY_CELLS: &'static [ShorthandId] = &[ShorthandId::All];
        static FLEX_DIRECTION: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::FlexFlow];
        static FLEX_WRAP: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::FlexFlow];
        static FLOAT: &'static [ShorthandId] = &[ShorthandId::All];
        static FONT_STRETCH: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Font];
        static FONT_STYLE: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Font];
        static FONT_VARIANT_CAPS: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Font,
            ShorthandId::FontVariant,
        ];
        static FONT_WEIGHT: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Font];
        static IMAGE_RENDERING: &'static [ShorthandId] = &[ShorthandId::All];
        static JUSTIFY_CONTENT: &'static [ShorthandId] = &[ShorthandId::All];
        static LIST_STYLE_POSITION: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::ListStyle];
        static LIST_STYLE_TYPE: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::ListStyle];
        static MIX_BLEND_MODE: &'static [ShorthandId] = &[ShorthandId::All];
        static OPACITY: &'static [ShorthandId] = &[ShorthandId::All];
        static ORDER: &'static [ShorthandId] = &[ShorthandId::All];
        static OUTLINE_STYLE: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Outline];
        static OVERFLOW_WRAP: &'static [ShorthandId] = &[ShorthandId::All];
        static POINTER_EVENTS: &'static [ShorthandId] = &[ShorthandId::All];
        static POSITION: &'static [ShorthandId] = &[ShorthandId::All];
        static _SERVO_OVERFLOW_CLIP_BOX: &'static [ShorthandId] = &[];
        static _SERVO_TOP_LAYER: &'static [ShorthandId] = &[];
        static TABLE_LAYOUT: &'static [ShorthandId] = &[ShorthandId::All];
        static TEXT_ALIGN: &'static [ShorthandId] = &[ShorthandId::All];
        static TEXT_DECORATION_LINE: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::TextDecoration];
        static TEXT_JUSTIFY: &'static [ShorthandId] = &[ShorthandId::All];
        static TEXT_RENDERING: &'static [ShorthandId] = &[ShorthandId::All];
        static TEXT_TRANSFORM: &'static [ShorthandId] = &[ShorthandId::All];
        static TRANSFORM_STYLE: &'static [ShorthandId] = &[ShorthandId::All];
        static UNICODE_BIDI: &'static [ShorthandId] = &[];
        static VISIBILITY: &'static [ShorthandId] = &[ShorthandId::All];
        static WHITE_SPACE: &'static [ShorthandId] = &[ShorthandId::All];
        static WORD_BREAK: &'static [ShorthandId] = &[ShorthandId::All];
        static WRITING_MODE: &'static [ShorthandId] = &[ShorthandId::All];
        static Z_INDEX: &'static [ShorthandId] = &[ShorthandId::All];
        static FLEX_GROW: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Flex];
        static FLEX_SHRINK: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Flex];
        static OVERFLOW_BLOCK: &'static [ShorthandId] = &[ShorthandId::All];
        static OVERFLOW_INLINE: &'static [ShorthandId] = &[ShorthandId::All];
        static OVERFLOW_X: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Overflow];
        static OVERFLOW_Y: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Overflow];
        static BORDER_BLOCK_END_STYLE: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::BorderBlock,
            ShorthandId::BorderBlockEnd,
            ShorthandId::BorderBlockStyle,
        ];
        static BORDER_BLOCK_START_STYLE: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::BorderBlock,
            ShorthandId::BorderBlockStart,
            ShorthandId::BorderBlockStyle,
        ];
        static BORDER_BOTTOM_STYLE: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderStyle,
            ShorthandId::BorderBottom,
        ];
        static BORDER_INLINE_END_STYLE: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::BorderInline,
            ShorthandId::BorderInlineEnd,
            ShorthandId::BorderInlineStyle,
        ];
        static BORDER_INLINE_START_STYLE: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::BorderInline,
            ShorthandId::BorderInlineStart,
            ShorthandId::BorderInlineStyle,
        ];
        static BORDER_LEFT_STYLE: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderStyle,
            ShorthandId::BorderLeft,
        ];
        static BORDER_RIGHT_STYLE: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderStyle,
            ShorthandId::BorderRight,
        ];
        static BORDER_TOP_STYLE: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderStyle,
            ShorthandId::BorderTop,
        ];
        static ANIMATION_DELAY: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Animation];
        static ANIMATION_DIRECTION: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Animation];
        static ANIMATION_DURATION: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Animation];
        static ANIMATION_FILL_MODE: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Animation];
        static ANIMATION_ITERATION_COUNT: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Animation];
        static ANIMATION_NAME: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Animation];
        static ANIMATION_PLAY_STATE: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Animation];
        static ANIMATION_TIMING_FUNCTION: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Animation];
        static BACKGROUND_ATTACHMENT: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Background];
        static BACKGROUND_CLIP: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Background];
        static BACKGROUND_IMAGE: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Background];
        static BACKGROUND_ORIGIN: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Background];
        static BACKGROUND_POSITION_X: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Background,
            ShorthandId::BackgroundPosition,
        ];
        static BACKGROUND_POSITION_Y: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Background,
            ShorthandId::BackgroundPosition,
        ];
        static BACKGROUND_REPEAT: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Background];
        static BACKGROUND_SIZE: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Background];
        static BORDER_IMAGE_OUTSET: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderImage,
        ];
        static BORDER_IMAGE_SLICE: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderImage,
        ];
        static BORDER_IMAGE_WIDTH: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderImage,
        ];
        static BORDER_SPACING: &'static [ShorthandId] = &[ShorthandId::All];
        static BOX_SHADOW: &'static [ShorthandId] = &[ShorthandId::All];
        static CLIP: &'static [ShorthandId] = &[ShorthandId::All];
        static COLOR: &'static [ShorthandId] = &[ShorthandId::All];
        static COLUMN_GAP: &'static [ShorthandId] = &[ShorthandId::All];
        static COLUMN_WIDTH: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Columns];
        static CONTENT: &'static [ShorthandId] = &[ShorthandId::All];
        static COUNTER_INCREMENT: &'static [ShorthandId] = &[ShorthandId::All];
        static COUNTER_RESET: &'static [ShorthandId] = &[ShorthandId::All];
        static CURSOR: &'static [ShorthandId] = &[ShorthandId::All];
        static FILTER: &'static [ShorthandId] = &[ShorthandId::All];
        static FLEX_BASIS: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Flex];
        static FONT_FAMILY: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Font];
        static FONT_SIZE: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Font];
        static LETTER_SPACING: &'static [ShorthandId] = &[ShorthandId::All];
        static LINE_HEIGHT: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Font];
        static OUTLINE_OFFSET: &'static [ShorthandId] = &[ShorthandId::All];
        static PERSPECTIVE: &'static [ShorthandId] = &[ShorthandId::All];
        static PERSPECTIVE_ORIGIN: &'static [ShorthandId] = &[ShorthandId::All];
        static QUOTES: &'static [ShorthandId] = &[ShorthandId::All];
        static ROTATE: &'static [ShorthandId] = &[ShorthandId::All];
        static SCALE: &'static [ShorthandId] = &[ShorthandId::All];
        static TEXT_INDENT: &'static [ShorthandId] = &[ShorthandId::All];
        static TEXT_OVERFLOW: &'static [ShorthandId] = &[ShorthandId::All];
        static TEXT_SHADOW: &'static [ShorthandId] = &[ShorthandId::All];
        static TRANSFORM: &'static [ShorthandId] = &[ShorthandId::All];
        static TRANSFORM_ORIGIN: &'static [ShorthandId] = &[ShorthandId::All];
        static TRANSITION_DELAY: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Transition];
        static TRANSITION_DURATION: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Transition];
        static TRANSITION_PROPERTY: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Transition];
        static TRANSITION_TIMING_FUNCTION: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Transition];
        static TRANSLATE: &'static [ShorthandId] = &[ShorthandId::All];
        static VERTICAL_ALIGN: &'static [ShorthandId] = &[ShorthandId::All];
        static WORD_SPACING: &'static [ShorthandId] = &[ShorthandId::All];
        static BORDER_IMAGE_SOURCE: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderImage,
        ];
        static LIST_STYLE_IMAGE: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::ListStyle];
        static MAX_BLOCK_SIZE: &'static [ShorthandId] = &[ShorthandId::All];
        static MAX_HEIGHT: &'static [ShorthandId] = &[ShorthandId::All];
        static MAX_INLINE_SIZE: &'static [ShorthandId] = &[ShorthandId::All];
        static MAX_WIDTH: &'static [ShorthandId] = &[ShorthandId::All];
        static BORDER_BOTTOM_LEFT_RADIUS: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::BorderRadius];
        static BORDER_BOTTOM_RIGHT_RADIUS: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::BorderRadius];
        static BORDER_END_END_RADIUS: &'static [ShorthandId] = &[ShorthandId::All];
        static BORDER_END_START_RADIUS: &'static [ShorthandId] = &[ShorthandId::All];
        static BORDER_START_END_RADIUS: &'static [ShorthandId] = &[ShorthandId::All];
        static BORDER_START_START_RADIUS: &'static [ShorthandId] = &[ShorthandId::All];
        static BORDER_TOP_LEFT_RADIUS: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::BorderRadius];
        static BORDER_TOP_RIGHT_RADIUS: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::BorderRadius];
        static PADDING_BLOCK_END: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::PaddingBlock];
        static PADDING_BLOCK_START: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::PaddingBlock];
        static PADDING_BOTTOM: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Padding];
        static PADDING_INLINE_END: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::PaddingInline];
        static PADDING_INLINE_START: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::PaddingInline];
        static PADDING_LEFT: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Padding];
        static PADDING_RIGHT: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Padding];
        static PADDING_TOP: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Padding];
        static BLOCK_SIZE: &'static [ShorthandId] = &[ShorthandId::All];
        static HEIGHT: &'static [ShorthandId] = &[ShorthandId::All];
        static INLINE_SIZE: &'static [ShorthandId] = &[ShorthandId::All];
        static MIN_BLOCK_SIZE: &'static [ShorthandId] = &[ShorthandId::All];
        static MIN_HEIGHT: &'static [ShorthandId] = &[ShorthandId::All];
        static MIN_INLINE_SIZE: &'static [ShorthandId] = &[ShorthandId::All];
        static MIN_WIDTH: &'static [ShorthandId] = &[ShorthandId::All];
        static WIDTH: &'static [ShorthandId] = &[ShorthandId::All];
        static BORDER_BLOCK_END_WIDTH: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::BorderBlock,
            ShorthandId::BorderBlockEnd,
            ShorthandId::BorderBlockWidth,
        ];
        static BORDER_BLOCK_START_WIDTH: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::BorderBlock,
            ShorthandId::BorderBlockStart,
            ShorthandId::BorderBlockWidth,
        ];
        static BORDER_BOTTOM_WIDTH: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderWidth,
            ShorthandId::BorderBottom,
        ];
        static BORDER_INLINE_END_WIDTH: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::BorderInline,
            ShorthandId::BorderInlineEnd,
            ShorthandId::BorderInlineWidth,
        ];
        static BORDER_INLINE_START_WIDTH: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::BorderInline,
            ShorthandId::BorderInlineStart,
            ShorthandId::BorderInlineWidth,
        ];
        static BORDER_LEFT_WIDTH: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderWidth,
            ShorthandId::BorderLeft,
        ];
        static BORDER_RIGHT_WIDTH: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderWidth,
            ShorthandId::BorderRight,
        ];
        static BORDER_TOP_WIDTH: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderWidth,
            ShorthandId::BorderTop,
        ];
        static OUTLINE_WIDTH: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Outline];
        static BACKGROUND_COLOR: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::Background];
        static BORDER_BLOCK_END_COLOR: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::BorderBlock,
            ShorthandId::BorderBlockEnd,
            ShorthandId::BorderBlockColor,
        ];
        static BORDER_BLOCK_START_COLOR: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::BorderBlock,
            ShorthandId::BorderBlockStart,
            ShorthandId::BorderBlockColor,
        ];
        static BORDER_BOTTOM_COLOR: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderColor,
            ShorthandId::BorderBottom,
        ];
        static BORDER_INLINE_END_COLOR: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::BorderInline,
            ShorthandId::BorderInlineEnd,
            ShorthandId::BorderInlineColor,
        ];
        static BORDER_INLINE_START_COLOR: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::BorderInline,
            ShorthandId::BorderInlineStart,
            ShorthandId::BorderInlineColor,
        ];
        static BORDER_LEFT_COLOR: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderColor,
            ShorthandId::BorderLeft,
        ];
        static BORDER_RIGHT_COLOR: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderColor,
            ShorthandId::BorderRight,
        ];
        static BORDER_TOP_COLOR: &'static [ShorthandId] = &[
            ShorthandId::All,
            ShorthandId::Border,
            ShorthandId::BorderColor,
            ShorthandId::BorderTop,
        ];
        static OUTLINE_COLOR: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Outline];
        static BOTTOM: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Inset];
        static INSET_BLOCK_END: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::InsetBlock];
        static INSET_BLOCK_START: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::InsetBlock];
        static INSET_INLINE_END: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::InsetInline];
        static INSET_INLINE_START: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::InsetInline];
        static LEFT: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Inset];
        static MARGIN_BLOCK_END: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::MarginBlock];
        static MARGIN_BLOCK_START: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::MarginBlock];
        static MARGIN_BOTTOM: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Margin];
        static MARGIN_INLINE_END: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::MarginInline];
        static MARGIN_INLINE_START: &'static [ShorthandId] =
            &[ShorthandId::All, ShorthandId::MarginInline];
        static MARGIN_LEFT: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Margin];
        static MARGIN_RIGHT: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Margin];
        static MARGIN_TOP: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Margin];
        static RIGHT: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Inset];
        static TOP: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Inset];

        NonCustomPropertyIterator {
            iter: match *self {
                LonghandId::AlignContent => ALIGN_CONTENT,
                LonghandId::AlignItems => ALIGN_ITEMS,
                LonghandId::AlignSelf => ALIGN_SELF,
                LonghandId::AspectRatio => ASPECT_RATIO,
                LonghandId::BackfaceVisibility => BACKFACE_VISIBILITY,
                LonghandId::BorderCollapse => BORDER_COLLAPSE,
                LonghandId::BorderImageRepeat => BORDER_IMAGE_REPEAT,
                LonghandId::BoxSizing => BOX_SIZING,
                LonghandId::CaptionSide => CAPTION_SIDE,
                LonghandId::Clear => CLEAR,
                LonghandId::ColumnCount => COLUMN_COUNT,
                LonghandId::Direction => DIRECTION,
                LonghandId::Display => DISPLAY,
                LonghandId::EmptyCells => EMPTY_CELLS,
                LonghandId::FlexDirection => FLEX_DIRECTION,
                LonghandId::FlexWrap => FLEX_WRAP,
                LonghandId::Float => FLOAT,
                LonghandId::FontStretch => FONT_STRETCH,
                LonghandId::FontStyle => FONT_STYLE,
                LonghandId::FontVariantCaps => FONT_VARIANT_CAPS,
                LonghandId::FontWeight => FONT_WEIGHT,
                LonghandId::ImageRendering => IMAGE_RENDERING,
                LonghandId::JustifyContent => JUSTIFY_CONTENT,
                LonghandId::ListStylePosition => LIST_STYLE_POSITION,
                LonghandId::ListStyleType => LIST_STYLE_TYPE,
                LonghandId::MixBlendMode => MIX_BLEND_MODE,
                LonghandId::Opacity => OPACITY,
                LonghandId::Order => ORDER,
                LonghandId::OutlineStyle => OUTLINE_STYLE,
                LonghandId::OverflowWrap => OVERFLOW_WRAP,
                LonghandId::PointerEvents => POINTER_EVENTS,
                LonghandId::Position => POSITION,
                LonghandId::ServoOverflowClipBox => _SERVO_OVERFLOW_CLIP_BOX,
                LonghandId::ServoTopLayer => _SERVO_TOP_LAYER,
                LonghandId::TableLayout => TABLE_LAYOUT,
                LonghandId::TextAlign => TEXT_ALIGN,
                LonghandId::TextDecorationLine => TEXT_DECORATION_LINE,
                LonghandId::TextJustify => TEXT_JUSTIFY,
                LonghandId::TextRendering => TEXT_RENDERING,
                LonghandId::TextTransform => TEXT_TRANSFORM,
                LonghandId::TransformStyle => TRANSFORM_STYLE,
                LonghandId::UnicodeBidi => UNICODE_BIDI,
                LonghandId::Visibility => VISIBILITY,
                LonghandId::WhiteSpace => WHITE_SPACE,
                LonghandId::WordBreak => WORD_BREAK,
                LonghandId::WritingMode => WRITING_MODE,
                LonghandId::ZIndex => Z_INDEX,
                LonghandId::FlexGrow => FLEX_GROW,
                LonghandId::FlexShrink => FLEX_SHRINK,
                LonghandId::OverflowBlock => OVERFLOW_BLOCK,
                LonghandId::OverflowInline => OVERFLOW_INLINE,
                LonghandId::OverflowX => OVERFLOW_X,
                LonghandId::OverflowY => OVERFLOW_Y,
                LonghandId::BorderBlockEndStyle => BORDER_BLOCK_END_STYLE,
                LonghandId::BorderBlockStartStyle => BORDER_BLOCK_START_STYLE,
                LonghandId::BorderBottomStyle => BORDER_BOTTOM_STYLE,
                LonghandId::BorderInlineEndStyle => BORDER_INLINE_END_STYLE,
                LonghandId::BorderInlineStartStyle => BORDER_INLINE_START_STYLE,
                LonghandId::BorderLeftStyle => BORDER_LEFT_STYLE,
                LonghandId::BorderRightStyle => BORDER_RIGHT_STYLE,
                LonghandId::BorderTopStyle => BORDER_TOP_STYLE,
                LonghandId::AnimationDelay => ANIMATION_DELAY,
                LonghandId::AnimationDirection => ANIMATION_DIRECTION,
                LonghandId::AnimationDuration => ANIMATION_DURATION,
                LonghandId::AnimationFillMode => ANIMATION_FILL_MODE,
                LonghandId::AnimationIterationCount => ANIMATION_ITERATION_COUNT,
                LonghandId::AnimationName => ANIMATION_NAME,
                LonghandId::AnimationPlayState => ANIMATION_PLAY_STATE,
                LonghandId::AnimationTimingFunction => ANIMATION_TIMING_FUNCTION,
                LonghandId::BackgroundAttachment => BACKGROUND_ATTACHMENT,
                LonghandId::BackgroundClip => BACKGROUND_CLIP,
                LonghandId::BackgroundImage => BACKGROUND_IMAGE,
                LonghandId::BackgroundOrigin => BACKGROUND_ORIGIN,
                LonghandId::BackgroundPositionX => BACKGROUND_POSITION_X,
                LonghandId::BackgroundPositionY => BACKGROUND_POSITION_Y,
                LonghandId::BackgroundRepeat => BACKGROUND_REPEAT,
                LonghandId::BackgroundSize => BACKGROUND_SIZE,
                LonghandId::BorderImageOutset => BORDER_IMAGE_OUTSET,
                LonghandId::BorderImageSlice => BORDER_IMAGE_SLICE,
                LonghandId::BorderImageWidth => BORDER_IMAGE_WIDTH,
                LonghandId::BorderSpacing => BORDER_SPACING,
                LonghandId::BoxShadow => BOX_SHADOW,
                LonghandId::Clip => CLIP,
                LonghandId::Color => COLOR,
                LonghandId::ColumnGap => COLUMN_GAP,
                LonghandId::ColumnWidth => COLUMN_WIDTH,
                LonghandId::Content => CONTENT,
                LonghandId::CounterIncrement => COUNTER_INCREMENT,
                LonghandId::CounterReset => COUNTER_RESET,
                LonghandId::Cursor => CURSOR,
                LonghandId::Filter => FILTER,
                LonghandId::FlexBasis => FLEX_BASIS,
                LonghandId::FontFamily => FONT_FAMILY,
                LonghandId::FontSize => FONT_SIZE,
                LonghandId::LetterSpacing => LETTER_SPACING,
                LonghandId::LineHeight => LINE_HEIGHT,
                LonghandId::OutlineOffset => OUTLINE_OFFSET,
                LonghandId::Perspective => PERSPECTIVE,
                LonghandId::PerspectiveOrigin => PERSPECTIVE_ORIGIN,
                LonghandId::Quotes => QUOTES,
                LonghandId::Rotate => ROTATE,
                LonghandId::Scale => SCALE,
                LonghandId::TextIndent => TEXT_INDENT,
                LonghandId::TextOverflow => TEXT_OVERFLOW,
                LonghandId::TextShadow => TEXT_SHADOW,
                LonghandId::Transform => TRANSFORM,
                LonghandId::TransformOrigin => TRANSFORM_ORIGIN,
                LonghandId::TransitionDelay => TRANSITION_DELAY,
                LonghandId::TransitionDuration => TRANSITION_DURATION,
                LonghandId::TransitionProperty => TRANSITION_PROPERTY,
                LonghandId::TransitionTimingFunction => TRANSITION_TIMING_FUNCTION,
                LonghandId::Translate => TRANSLATE,
                LonghandId::VerticalAlign => VERTICAL_ALIGN,
                LonghandId::WordSpacing => WORD_SPACING,
                LonghandId::BorderImageSource => BORDER_IMAGE_SOURCE,
                LonghandId::ListStyleImage => LIST_STYLE_IMAGE,
                LonghandId::MaxBlockSize => MAX_BLOCK_SIZE,
                LonghandId::MaxHeight => MAX_HEIGHT,
                LonghandId::MaxInlineSize => MAX_INLINE_SIZE,
                LonghandId::MaxWidth => MAX_WIDTH,
                LonghandId::BorderBottomLeftRadius => BORDER_BOTTOM_LEFT_RADIUS,
                LonghandId::BorderBottomRightRadius => BORDER_BOTTOM_RIGHT_RADIUS,
                LonghandId::BorderEndEndRadius => BORDER_END_END_RADIUS,
                LonghandId::BorderEndStartRadius => BORDER_END_START_RADIUS,
                LonghandId::BorderStartEndRadius => BORDER_START_END_RADIUS,
                LonghandId::BorderStartStartRadius => BORDER_START_START_RADIUS,
                LonghandId::BorderTopLeftRadius => BORDER_TOP_LEFT_RADIUS,
                LonghandId::BorderTopRightRadius => BORDER_TOP_RIGHT_RADIUS,
                LonghandId::PaddingBlockEnd => PADDING_BLOCK_END,
                LonghandId::PaddingBlockStart => PADDING_BLOCK_START,
                LonghandId::PaddingBottom => PADDING_BOTTOM,
                LonghandId::PaddingInlineEnd => PADDING_INLINE_END,
                LonghandId::PaddingInlineStart => PADDING_INLINE_START,
                LonghandId::PaddingLeft => PADDING_LEFT,
                LonghandId::PaddingRight => PADDING_RIGHT,
                LonghandId::PaddingTop => PADDING_TOP,
                LonghandId::BlockSize => BLOCK_SIZE,
                LonghandId::Height => HEIGHT,
                LonghandId::InlineSize => INLINE_SIZE,
                LonghandId::MinBlockSize => MIN_BLOCK_SIZE,
                LonghandId::MinHeight => MIN_HEIGHT,
                LonghandId::MinInlineSize => MIN_INLINE_SIZE,
                LonghandId::MinWidth => MIN_WIDTH,
                LonghandId::Width => WIDTH,
                LonghandId::BorderBlockEndWidth => BORDER_BLOCK_END_WIDTH,
                LonghandId::BorderBlockStartWidth => BORDER_BLOCK_START_WIDTH,
                LonghandId::BorderBottomWidth => BORDER_BOTTOM_WIDTH,
                LonghandId::BorderInlineEndWidth => BORDER_INLINE_END_WIDTH,
                LonghandId::BorderInlineStartWidth => BORDER_INLINE_START_WIDTH,
                LonghandId::BorderLeftWidth => BORDER_LEFT_WIDTH,
                LonghandId::BorderRightWidth => BORDER_RIGHT_WIDTH,
                LonghandId::BorderTopWidth => BORDER_TOP_WIDTH,
                LonghandId::OutlineWidth => OUTLINE_WIDTH,
                LonghandId::BackgroundColor => BACKGROUND_COLOR,
                LonghandId::BorderBlockEndColor => BORDER_BLOCK_END_COLOR,
                LonghandId::BorderBlockStartColor => BORDER_BLOCK_START_COLOR,
                LonghandId::BorderBottomColor => BORDER_BOTTOM_COLOR,
                LonghandId::BorderInlineEndColor => BORDER_INLINE_END_COLOR,
                LonghandId::BorderInlineStartColor => BORDER_INLINE_START_COLOR,
                LonghandId::BorderLeftColor => BORDER_LEFT_COLOR,
                LonghandId::BorderRightColor => BORDER_RIGHT_COLOR,
                LonghandId::BorderTopColor => BORDER_TOP_COLOR,
                LonghandId::OutlineColor => OUTLINE_COLOR,
                LonghandId::Bottom => BOTTOM,
                LonghandId::InsetBlockEnd => INSET_BLOCK_END,
                LonghandId::InsetBlockStart => INSET_BLOCK_START,
                LonghandId::InsetInlineEnd => INSET_INLINE_END,
                LonghandId::InsetInlineStart => INSET_INLINE_START,
                LonghandId::Left => LEFT,
                LonghandId::MarginBlockEnd => MARGIN_BLOCK_END,
                LonghandId::MarginBlockStart => MARGIN_BLOCK_START,
                LonghandId::MarginBottom => MARGIN_BOTTOM,
                LonghandId::MarginInlineEnd => MARGIN_INLINE_END,
                LonghandId::MarginInlineStart => MARGIN_INLINE_START,
                LonghandId::MarginLeft => MARGIN_LEFT,
                LonghandId::MarginRight => MARGIN_RIGHT,
                LonghandId::MarginTop => MARGIN_TOP,
                LonghandId::Right => RIGHT,
                LonghandId::Top => TOP,
            }
            .iter(),
        }
    }

    pub fn parse_value<'i, 't>(
        &self,
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<PropertyDeclaration, ParseError<'i>> {
        type ParsePropertyFn = for<'i, 't> fn(
            context: &ParserContext,
            input: &mut Parser<'i, 't>,
        )
            -> Result<PropertyDeclaration, ParseError<'i>>;
        static PARSE_PROPERTY: [ParsePropertyFn; 179] = [
            longhands::align_content::parse_declared,
            longhands::align_items::parse_declared,
            longhands::align_self::parse_declared,
            longhands::aspect_ratio::parse_declared,
            longhands::backface_visibility::parse_declared,
            longhands::border_collapse::parse_declared,
            longhands::border_image_repeat::parse_declared,
            longhands::box_sizing::parse_declared,
            longhands::caption_side::parse_declared,
            longhands::clear::parse_declared,
            longhands::column_count::parse_declared,
            longhands::direction::parse_declared,
            longhands::display::parse_declared,
            longhands::empty_cells::parse_declared,
            longhands::flex_direction::parse_declared,
            longhands::flex_wrap::parse_declared,
            longhands::float::parse_declared,
            longhands::font_stretch::parse_declared,
            longhands::font_style::parse_declared,
            longhands::font_variant_caps::parse_declared,
            longhands::font_weight::parse_declared,
            longhands::image_rendering::parse_declared,
            longhands::justify_content::parse_declared,
            longhands::list_style_position::parse_declared,
            longhands::list_style_type::parse_declared,
            longhands::mix_blend_mode::parse_declared,
            longhands::opacity::parse_declared,
            longhands::order::parse_declared,
            longhands::outline_style::parse_declared,
            longhands::overflow_wrap::parse_declared,
            longhands::pointer_events::parse_declared,
            longhands::position::parse_declared,
            longhands::table_layout::parse_declared,
            longhands::text_align::parse_declared,
            longhands::text_decoration_line::parse_declared,
            longhands::text_justify::parse_declared,
            longhands::text_rendering::parse_declared,
            longhands::text_transform::parse_declared,
            longhands::transform_style::parse_declared,
            longhands::unicode_bidi::parse_declared,
            longhands::visibility::parse_declared,
            longhands::white_space::parse_declared,
            longhands::word_break::parse_declared,
            longhands::writing_mode::parse_declared,
            longhands::z_index::parse_declared,
            longhands::flex_grow::parse_declared,
            longhands::flex_shrink::parse_declared,
            longhands::overflow_block::parse_declared,
            longhands::overflow_inline::parse_declared,
            longhands::overflow_x::parse_declared,
            longhands::overflow_y::parse_declared,
            longhands::border_block_end_style::parse_declared,
            longhands::border_block_start_style::parse_declared,
            longhands::border_bottom_style::parse_declared,
            longhands::border_inline_end_style::parse_declared,
            longhands::border_inline_start_style::parse_declared,
            longhands::border_left_style::parse_declared,
            longhands::border_right_style::parse_declared,
            longhands::border_top_style::parse_declared,
            longhands::animation_delay::parse_declared,
            longhands::animation_direction::parse_declared,
            longhands::animation_duration::parse_declared,
            longhands::animation_fill_mode::parse_declared,
            longhands::animation_iteration_count::parse_declared,
            longhands::animation_name::parse_declared,
            longhands::animation_play_state::parse_declared,
            longhands::animation_timing_function::parse_declared,
            longhands::background_attachment::parse_declared,
            longhands::background_clip::parse_declared,
            longhands::background_image::parse_declared,
            longhands::background_origin::parse_declared,
            longhands::background_position_x::parse_declared,
            longhands::background_position_y::parse_declared,
            longhands::background_repeat::parse_declared,
            longhands::background_size::parse_declared,
            longhands::border_image_outset::parse_declared,
            longhands::border_image_slice::parse_declared,
            longhands::border_image_width::parse_declared,
            longhands::border_spacing::parse_declared,
            longhands::box_shadow::parse_declared,
            longhands::clip::parse_declared,
            longhands::color::parse_declared,
            longhands::column_gap::parse_declared,
            longhands::column_width::parse_declared,
            longhands::content::parse_declared,
            longhands::counter_increment::parse_declared,
            longhands::counter_reset::parse_declared,
            longhands::cursor::parse_declared,
            longhands::filter::parse_declared,
            longhands::flex_basis::parse_declared,
            longhands::font_family::parse_declared,
            longhands::font_size::parse_declared,
            longhands::letter_spacing::parse_declared,
            longhands::line_height::parse_declared,
            longhands::outline_offset::parse_declared,
            longhands::perspective::parse_declared,
            longhands::perspective_origin::parse_declared,
            longhands::quotes::parse_declared,
            longhands::rotate::parse_declared,
            longhands::scale::parse_declared,
            longhands::text_indent::parse_declared,
            longhands::text_overflow::parse_declared,
            longhands::text_shadow::parse_declared,
            longhands::transform::parse_declared,
            longhands::transform_origin::parse_declared,
            longhands::transition_delay::parse_declared,
            longhands::transition_duration::parse_declared,
            longhands::transition_property::parse_declared,
            longhands::transition_timing_function::parse_declared,
            longhands::translate::parse_declared,
            longhands::vertical_align::parse_declared,
            longhands::word_spacing::parse_declared,
            longhands::border_image_source::parse_declared,
            longhands::list_style_image::parse_declared,
            longhands::max_block_size::parse_declared,
            longhands::max_height::parse_declared,
            longhands::max_inline_size::parse_declared,
            longhands::max_width::parse_declared,
            longhands::border_bottom_left_radius::parse_declared,
            longhands::border_bottom_right_radius::parse_declared,
            longhands::border_end_end_radius::parse_declared,
            longhands::border_end_start_radius::parse_declared,
            longhands::border_start_end_radius::parse_declared,
            longhands::border_start_start_radius::parse_declared,
            longhands::border_top_left_radius::parse_declared,
            longhands::border_top_right_radius::parse_declared,
            longhands::padding_block_end::parse_declared,
            longhands::padding_block_start::parse_declared,
            longhands::padding_bottom::parse_declared,
            longhands::padding_inline_end::parse_declared,
            longhands::padding_inline_start::parse_declared,
            longhands::padding_left::parse_declared,
            longhands::padding_right::parse_declared,
            longhands::padding_top::parse_declared,
            longhands::block_size::parse_declared,
            longhands::height::parse_declared,
            longhands::inline_size::parse_declared,
            longhands::min_block_size::parse_declared,
            longhands::min_height::parse_declared,
            longhands::min_inline_size::parse_declared,
            longhands::min_width::parse_declared,
            longhands::width::parse_declared,
            longhands::border_block_end_width::parse_declared,
            longhands::border_block_start_width::parse_declared,
            longhands::border_bottom_width::parse_declared,
            longhands::border_inline_end_width::parse_declared,
            longhands::border_inline_start_width::parse_declared,
            longhands::border_left_width::parse_declared,
            longhands::border_right_width::parse_declared,
            longhands::border_top_width::parse_declared,
            longhands::outline_width::parse_declared,
            longhands::background_color::parse_declared,
            longhands::border_block_end_color::parse_declared,
            longhands::border_block_start_color::parse_declared,
            longhands::border_bottom_color::parse_declared,
            longhands::border_inline_end_color::parse_declared,
            longhands::border_inline_start_color::parse_declared,
            longhands::border_left_color::parse_declared,
            longhands::border_right_color::parse_declared,
            longhands::border_top_color::parse_declared,
            longhands::outline_color::parse_declared,
            longhands::bottom::parse_declared,
            longhands::inset_block_end::parse_declared,
            longhands::inset_block_start::parse_declared,
            longhands::inset_inline_end::parse_declared,
            longhands::inset_inline_start::parse_declared,
            longhands::left::parse_declared,
            longhands::margin_block_end::parse_declared,
            longhands::margin_block_start::parse_declared,
            longhands::margin_bottom::parse_declared,
            longhands::margin_inline_end::parse_declared,
            longhands::margin_inline_start::parse_declared,
            longhands::margin_left::parse_declared,
            longhands::margin_right::parse_declared,
            longhands::margin_top::parse_declared,
            longhands::right::parse_declared,
            longhands::top::parse_declared,
        ];
        (PARSE_PROPERTY[*self as usize])(context, input)
    }
}

/// A set of longhand properties
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LonghandIdSet {
    storage: [u32; (179 - 1 + 32) / 32],
}

impl LonghandIdSet {
    /// Create an empty set
    #[inline]
    pub fn new() -> LonghandIdSet {
        LonghandIdSet {
            storage: [0; (179 - 1 + 32) / 32],
        }
    }
    /// Return whether the given property is in the set
    #[inline]
    pub fn contains(&self, id: LonghandId) -> bool {
        let bit = id as usize;
        (self.storage[bit / 32] & (1 << (bit % 32))) != 0
    }

    /// Add the given property to the set
    #[inline]
    pub fn insert(&mut self, id: LonghandId) {
        let bit = id as usize;
        self.storage[bit / 32] |= 1 << (bit % 32);
    }

    /// Remove the given property from the set
    #[inline]
    pub fn remove(&mut self, id: LonghandId) {
        let bit = id as usize;
        self.storage[bit / 32] &= !(1 << (bit % 32));
    }

    /// Clear all bits
    #[inline]
    pub fn clear(&mut self) {
        for cell in &mut self.storage {
            *cell = 0
        }
    }

    /// Returns whether the set is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.storage.iter().all(|c| *c == 0)
    }
}
