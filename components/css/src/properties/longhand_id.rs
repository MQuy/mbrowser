use core::fmt;
use std::convert::TryFrom;
use std::fmt::Write;

use cssparser::Parser;
use num_enum::TryFromPrimitive;

use super::property_id::{NonCustomPropertyId, NonCustomPropertyIterator};
use crate::computed_values::StyleContext;
use crate::css_writer::{CssWriter, ToCss};
use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::longhands;
use crate::properties::shorthand_id::ShorthandId;
use crate::stylesheets::stylesheet::ParserContext;

/// An identifier for a given longhand property.
#[derive(Clone, Copy, Eq, Hash, PartialEq, TryFromPrimitive)]
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
	/// table-layout
	TableLayout = 32,
	/// text-align
	TextAlign = 33,
	/// text-decoration-line
	TextDecorationLine = 34,
	/// text-justify
	TextJustify = 35,
	/// text-rendering
	TextRendering = 36,
	/// text-transform
	TextTransform = 37,
	/// transform-style
	TransformStyle = 38,
	/// unicode-bidi
	UnicodeBidi = 39,
	/// visibility
	Visibility = 40,
	/// white-space
	WhiteSpace = 41,
	/// word-break
	WordBreak = 42,
	/// writing-mode
	WritingMode = 43,
	/// z-index
	ZIndex = 44,
	/// flex-grow
	FlexGrow = 45,
	/// flex-shrink
	FlexShrink = 46,
	/// overflow-block
	OverflowBlock = 47,
	/// overflow-inline
	OverflowInline = 48,
	/// overflow-x
	OverflowX = 49,
	/// overflow-y
	OverflowY = 50,
	/// border-block-end-style
	BorderBlockEndStyle = 51,
	/// border-block-start-style
	BorderBlockStartStyle = 52,
	/// border-bottom-style
	BorderBottomStyle = 53,
	/// border-inline-end-style
	BorderInlineEndStyle = 54,
	/// border-inline-start-style
	BorderInlineStartStyle = 55,
	/// border-left-style
	BorderLeftStyle = 56,
	/// border-right-style
	BorderRightStyle = 57,
	/// border-top-style
	BorderTopStyle = 58,
	/// animation-delay
	AnimationDelay = 59,
	/// animation-direction
	AnimationDirection = 60,
	/// animation-duration
	AnimationDuration = 61,
	/// animation-fill-mode
	AnimationFillMode = 62,
	/// animation-iteration-count
	AnimationIterationCount = 63,
	/// animation-name
	AnimationName = 64,
	/// animation-play-state
	AnimationPlayState = 65,
	/// animation-timing-function
	AnimationTimingFunction = 66,
	/// background-attachment
	BackgroundAttachment = 67,
	/// background-clip
	BackgroundClip = 68,
	/// background-image
	BackgroundImage = 69,
	/// background-origin
	BackgroundOrigin = 70,
	/// background-position-x
	BackgroundPositionX = 71,
	/// background-position-y
	BackgroundPositionY = 72,
	/// background-repeat
	BackgroundRepeat = 73,
	/// background-size
	BackgroundSize = 74,
	/// border-image-outset
	BorderImageOutset = 75,
	/// border-image-slice
	BorderImageSlice = 76,
	/// border-image-width
	BorderImageWidth = 77,
	/// border-spacing
	BorderSpacing = 78,
	/// box-shadow
	BoxShadow = 79,
	/// clip
	Clip = 80,
	/// color
	Color = 81,
	/// column-gap
	ColumnGap = 82,
	/// column-width
	ColumnWidth = 83,
	/// content
	Content = 84,
	/// counter-increment
	CounterIncrement = 85,
	/// counter-reset
	CounterReset = 86,
	/// cursor
	Cursor = 87,
	/// filter
	Filter = 88,
	/// flex-basis
	FlexBasis = 89,
	/// font-family
	FontFamily = 90,
	/// font-size
	FontSize = 91,
	/// letter-spacing
	LetterSpacing = 92,
	/// line-height
	LineHeight = 93,
	/// outline-offset
	OutlineOffset = 94,
	/// perspective
	Perspective = 95,
	/// perspective-origin
	PerspectiveOrigin = 96,
	/// quotes
	Quotes = 97,
	/// rotate
	Rotate = 98,
	/// scale
	Scale = 99,
	/// text-indent
	TextIndent = 100,
	/// text-overflow
	TextOverflow = 101,
	/// text-shadow
	TextShadow = 102,
	/// transform
	Transform = 103,
	/// transform-origin
	TransformOrigin = 104,
	/// transition-delay
	TransitionDelay = 105,
	/// transition-duration
	TransitionDuration = 106,
	/// transition-property
	TransitionProperty = 107,
	/// transition-timing-function
	TransitionTimingFunction = 108,
	/// translate
	Translate = 109,
	/// vertical-align
	VerticalAlign = 110,
	/// word-spacing
	WordSpacing = 111,
	/// border-image-source
	BorderImageSource = 112,
	/// list-style-image
	ListStyleImage = 113,
	/// max-block-size
	MaxBlockSize = 114,
	/// max-height
	MaxHeight = 115,
	/// max-inline-size
	MaxInlineSize = 116,
	/// max-width
	MaxWidth = 117,
	/// border-bottom-left-radius
	BorderBottomLeftRadius = 118,
	/// border-bottom-right-radius
	BorderBottomRightRadius = 119,
	/// border-end-end-radius
	BorderEndEndRadius = 120,
	/// border-end-start-radius
	BorderEndStartRadius = 121,
	/// border-start-end-radius
	BorderStartEndRadius = 122,
	/// border-start-start-radius
	BorderStartStartRadius = 123,
	/// border-top-left-radius
	BorderTopLeftRadius = 124,
	/// border-top-right-radius
	BorderTopRightRadius = 125,
	/// padding-block-end
	PaddingBlockEnd = 126,
	/// padding-block-start
	PaddingBlockStart = 127,
	/// padding-bottom
	PaddingBottom = 128,
	/// padding-inline-end
	PaddingInlineEnd = 129,
	/// padding-inline-start
	PaddingInlineStart = 130,
	/// padding-left
	PaddingLeft = 131,
	/// padding-right
	PaddingRight = 132,
	/// padding-top
	PaddingTop = 133,
	/// block-size
	BlockSize = 134,
	/// height
	Height = 135,
	/// inline-size
	InlineSize = 136,
	/// min-block-size
	MinBlockSize = 137,
	/// min-height
	MinHeight = 138,
	/// min-inline-size
	MinInlineSize = 139,
	/// min-width
	MinWidth = 140,
	/// width
	Width = 141,
	/// border-block-end-width
	BorderBlockEndWidth = 142,
	/// border-block-start-width
	BorderBlockStartWidth = 143,
	/// border-bottom-width
	BorderBottomWidth = 144,
	/// border-inline-end-width
	BorderInlineEndWidth = 145,
	/// border-inline-start-width
	BorderInlineStartWidth = 146,
	/// border-left-width
	BorderLeftWidth = 147,
	/// border-right-width
	BorderRightWidth = 148,
	/// border-top-width
	BorderTopWidth = 149,
	/// outline-width
	OutlineWidth = 150,
	/// background-color
	BackgroundColor = 151,
	/// border-block-end-color
	BorderBlockEndColor = 152,
	/// border-block-start-color
	BorderBlockStartColor = 153,
	/// border-bottom-color
	BorderBottomColor = 154,
	/// border-inline-end-color
	BorderInlineEndColor = 155,
	/// border-inline-start-color
	BorderInlineStartColor = 156,
	/// border-left-color
	BorderLeftColor = 157,
	/// border-right-color
	BorderRightColor = 158,
	/// border-top-color
	BorderTopColor = 159,
	/// outline-color
	OutlineColor = 160,
	/// bottom
	Bottom = 161,
	/// inset-block-end
	InsetBlockEnd = 162,
	/// inset-block-start
	InsetBlockStart = 163,
	/// inset-inline-end
	InsetInlineEnd = 164,
	/// inset-inline-start
	InsetInlineStart = 165,
	/// left
	Left = 166,
	/// margin-block-end
	MarginBlockEnd = 167,
	/// margin-block-start
	MarginBlockStart = 168,
	/// margin-bottom
	MarginBottom = 169,
	/// margin-inline-end
	MarginInlineEnd = 170,
	/// margin-inline-start
	MarginInlineStart = 171,
	/// margin-left
	MarginLeft = 172,
	/// margin-right
	MarginRight = 173,
	/// margin-top
	MarginTop = 174,
	/// right
	Right = 175,
	/// top
	Top = 176,
	/// ---- additional

	/// counter-set
	CounterSet = 177,
	/// object-fit
	ObjectFit = 178,
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

	pub fn shorthands(&self) -> NonCustomPropertyIterator<ShorthandId> {
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
		static BORDER_IMAGE_REPEAT: &'static [ShorthandId] =
			&[ShorthandId::All, ShorthandId::Border, ShorthandId::BorderImage];
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
		static FONT_VARIANT_CAPS: &'static [ShorthandId] =
			&[ShorthandId::All, ShorthandId::Font, ShorthandId::FontVariant];
		static FONT_WEIGHT: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Font];
		static IMAGE_RENDERING: &'static [ShorthandId] = &[ShorthandId::All];
		static JUSTIFY_CONTENT: &'static [ShorthandId] = &[ShorthandId::All];
		static LIST_STYLE_POSITION: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::ListStyle];
		static LIST_STYLE_TYPE: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::ListStyle];
		static MIX_BLEND_MODE: &'static [ShorthandId] = &[ShorthandId::All];
		static OPACITY: &'static [ShorthandId] = &[ShorthandId::All];
		static OBJECTFIT: &'static [ShorthandId] = &[ShorthandId::All];
		static ORDER: &'static [ShorthandId] = &[ShorthandId::All];
		static OUTLINE_STYLE: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Outline];
		static OVERFLOW_WRAP: &'static [ShorthandId] = &[ShorthandId::All];
		static POINTER_EVENTS: &'static [ShorthandId] = &[ShorthandId::All];
		static POSITION: &'static [ShorthandId] = &[ShorthandId::All];
		static TABLE_LAYOUT: &'static [ShorthandId] = &[ShorthandId::All];
		static TEXT_ALIGN: &'static [ShorthandId] = &[ShorthandId::All];
		static TEXT_DECORATION_LINE: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::TextDecoration];
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
		static ANIMATION_DELAY: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Animation];
		static ANIMATION_DIRECTION: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Animation];
		static ANIMATION_DURATION: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Animation];
		static ANIMATION_FILL_MODE: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Animation];
		static ANIMATION_ITERATION_COUNT: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Animation];
		static ANIMATION_NAME: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Animation];
		static ANIMATION_PLAY_STATE: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Animation];
		static ANIMATION_TIMING_FUNCTION: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Animation];
		static BACKGROUND_ATTACHMENT: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Background];
		static BACKGROUND_CLIP: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Background];
		static BACKGROUND_IMAGE: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Background];
		static BACKGROUND_ORIGIN: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Background];
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
		static BACKGROUND_REPEAT: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Background];
		static BACKGROUND_SIZE: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Background];
		static BORDER_IMAGE_OUTSET: &'static [ShorthandId] =
			&[ShorthandId::All, ShorthandId::Border, ShorthandId::BorderImage];
		static BORDER_IMAGE_SLICE: &'static [ShorthandId] =
			&[ShorthandId::All, ShorthandId::Border, ShorthandId::BorderImage];
		static BORDER_IMAGE_WIDTH: &'static [ShorthandId] =
			&[ShorthandId::All, ShorthandId::Border, ShorthandId::BorderImage];
		static BORDER_SPACING: &'static [ShorthandId] = &[ShorthandId::All];
		static BOX_SHADOW: &'static [ShorthandId] = &[ShorthandId::All];
		static CLIP: &'static [ShorthandId] = &[ShorthandId::All];
		static COLOR: &'static [ShorthandId] = &[ShorthandId::All];
		static COLUMN_GAP: &'static [ShorthandId] = &[ShorthandId::All];
		static COLUMN_WIDTH: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Columns];
		static CONTENT: &'static [ShorthandId] = &[ShorthandId::All];
		static COUNTER_INCREMENT: &'static [ShorthandId] = &[ShorthandId::All];
		static COUNTER_RESET: &'static [ShorthandId] = &[ShorthandId::All];
		static COUNTER_SET: &'static [ShorthandId] = &[ShorthandId::All];
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
		static TRANSITION_DELAY: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Transition];
		static TRANSITION_DURATION: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Transition];
		static TRANSITION_PROPERTY: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Transition];
		static TRANSITION_TIMING_FUNCTION: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Transition];
		static TRANSLATE: &'static [ShorthandId] = &[ShorthandId::All];
		static VERTICAL_ALIGN: &'static [ShorthandId] = &[ShorthandId::All];
		static WORD_SPACING: &'static [ShorthandId] = &[ShorthandId::All];
		static BORDER_IMAGE_SOURCE: &'static [ShorthandId] =
			&[ShorthandId::All, ShorthandId::Border, ShorthandId::BorderImage];
		static LIST_STYLE_IMAGE: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::ListStyle];
		static MAX_BLOCK_SIZE: &'static [ShorthandId] = &[ShorthandId::All];
		static MAX_HEIGHT: &'static [ShorthandId] = &[ShorthandId::All];
		static MAX_INLINE_SIZE: &'static [ShorthandId] = &[ShorthandId::All];
		static MAX_WIDTH: &'static [ShorthandId] = &[ShorthandId::All];
		static BORDER_BOTTOM_LEFT_RADIUS: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::BorderRadius];
		static BORDER_BOTTOM_RIGHT_RADIUS: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::BorderRadius];
		static BORDER_END_END_RADIUS: &'static [ShorthandId] = &[ShorthandId::All];
		static BORDER_END_START_RADIUS: &'static [ShorthandId] = &[ShorthandId::All];
		static BORDER_START_END_RADIUS: &'static [ShorthandId] = &[ShorthandId::All];
		static BORDER_START_START_RADIUS: &'static [ShorthandId] = &[ShorthandId::All];
		static BORDER_TOP_LEFT_RADIUS: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::BorderRadius];
		static BORDER_TOP_RIGHT_RADIUS: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::BorderRadius];
		static PADDING_BLOCK_END: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::PaddingBlock];
		static PADDING_BLOCK_START: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::PaddingBlock];
		static PADDING_BOTTOM: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Padding];
		static PADDING_INLINE_END: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::PaddingInline];
		static PADDING_INLINE_START: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::PaddingInline];
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
		static BACKGROUND_COLOR: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Background];
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
		static INSET_BLOCK_END: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::InsetBlock];
		static INSET_BLOCK_START: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::InsetBlock];
		static INSET_INLINE_END: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::InsetInline];
		static INSET_INLINE_START: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::InsetInline];
		static LEFT: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Inset];
		static MARGIN_BLOCK_END: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::MarginBlock];
		static MARGIN_BLOCK_START: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::MarginBlock];
		static MARGIN_BOTTOM: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::Margin];
		static MARGIN_INLINE_END: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::MarginInline];
		static MARGIN_INLINE_START: &'static [ShorthandId] = &[ShorthandId::All, ShorthandId::MarginInline];
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
				LonghandId::ObjectFit => OBJECTFIT,
				LonghandId::Opacity => OPACITY,
				LonghandId::Order => ORDER,
				LonghandId::OutlineStyle => OUTLINE_STYLE,
				LonghandId::OverflowWrap => OVERFLOW_WRAP,
				LonghandId::PointerEvents => POINTER_EVENTS,
				LonghandId::Position => POSITION,
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
				LonghandId::CounterSet => COUNTER_SET,
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
		let parser_func = match self {
			LonghandId::AlignContent => longhands::align_content::parse_declared,
			LonghandId::AlignItems => longhands::align_items::parse_declared,
			LonghandId::AlignSelf => longhands::align_self::parse_declared,
			LonghandId::AspectRatio => longhands::aspect_ratio::parse_declared,
			LonghandId::BackfaceVisibility => longhands::backface_visibility::parse_declared,
			LonghandId::BorderCollapse => longhands::border_collapse::parse_declared,
			LonghandId::BorderImageRepeat => longhands::border_image_repeat::parse_declared,
			LonghandId::BoxSizing => longhands::box_sizing::parse_declared,
			LonghandId::CaptionSide => longhands::caption_side::parse_declared,
			LonghandId::Clear => longhands::clear::parse_declared,
			LonghandId::ColumnCount => longhands::column_count::parse_declared,
			LonghandId::Direction => longhands::direction::parse_declared,
			LonghandId::Display => longhands::display::parse_declared,
			LonghandId::EmptyCells => longhands::empty_cells::parse_declared,
			LonghandId::FlexDirection => longhands::flex_direction::parse_declared,
			LonghandId::FlexWrap => longhands::flex_wrap::parse_declared,
			LonghandId::Float => longhands::float::parse_declared,
			LonghandId::FontStretch => longhands::font_stretch::parse_declared,
			LonghandId::FontStyle => longhands::font_style::parse_declared,
			LonghandId::FontVariantCaps => longhands::font_variant_caps::parse_declared,
			LonghandId::FontWeight => longhands::font_weight::parse_declared,
			LonghandId::ImageRendering => longhands::image_rendering::parse_declared,
			LonghandId::JustifyContent => longhands::justify_content::parse_declared,
			LonghandId::ListStylePosition => longhands::list_style_position::parse_declared,
			LonghandId::ListStyleType => longhands::list_style_type::parse_declared,
			LonghandId::MixBlendMode => longhands::mix_blend_mode::parse_declared,
			LonghandId::ObjectFit => longhands::object_fit::parse_declared,
			LonghandId::Opacity => longhands::opacity::parse_declared,
			LonghandId::Order => longhands::order::parse_declared,
			LonghandId::OutlineStyle => longhands::outline_style::parse_declared,
			LonghandId::OverflowWrap => longhands::overflow_wrap::parse_declared,
			LonghandId::PointerEvents => longhands::pointer_events::parse_declared,
			LonghandId::Position => longhands::position::parse_declared,
			LonghandId::TableLayout => longhands::table_layout::parse_declared,
			LonghandId::TextAlign => longhands::text_align::parse_declared,
			LonghandId::TextDecorationLine => longhands::text_decoration_line::parse_declared,
			LonghandId::TextJustify => longhands::text_justify::parse_declared,
			LonghandId::TextRendering => longhands::text_rendering::parse_declared,
			LonghandId::TextTransform => longhands::text_transform::parse_declared,
			LonghandId::TransformStyle => longhands::transform_style::parse_declared,
			LonghandId::UnicodeBidi => longhands::unicode_bidi::parse_declared,
			LonghandId::Visibility => longhands::visibility::parse_declared,
			LonghandId::WhiteSpace => longhands::white_space::parse_declared,
			LonghandId::WordBreak => longhands::word_break::parse_declared,
			LonghandId::WritingMode => longhands::writing_mode::parse_declared,
			LonghandId::ZIndex => longhands::z_index::parse_declared,
			LonghandId::FlexGrow => longhands::flex_grow::parse_declared,
			LonghandId::FlexShrink => longhands::flex_shrink::parse_declared,
			LonghandId::OverflowBlock => longhands::overflow_block::parse_declared,
			LonghandId::OverflowInline => longhands::overflow_inline::parse_declared,
			LonghandId::OverflowX => longhands::overflow_x::parse_declared,
			LonghandId::OverflowY => longhands::overflow_y::parse_declared,
			LonghandId::BorderBlockEndStyle => longhands::border_block_end_style::parse_declared,
			LonghandId::BorderBlockStartStyle => longhands::border_block_start_style::parse_declared,
			LonghandId::BorderBottomStyle => longhands::border_bottom_style::parse_declared,
			LonghandId::BorderInlineEndStyle => longhands::border_inline_end_style::parse_declared,
			LonghandId::BorderInlineStartStyle => longhands::border_inline_start_style::parse_declared,
			LonghandId::BorderLeftStyle => longhands::border_left_style::parse_declared,
			LonghandId::BorderRightStyle => longhands::border_right_style::parse_declared,
			LonghandId::BorderTopStyle => longhands::border_top_style::parse_declared,
			LonghandId::AnimationDelay => longhands::animation_delay::parse_declared,
			LonghandId::AnimationDirection => longhands::animation_direction::parse_declared,
			LonghandId::AnimationDuration => longhands::animation_duration::parse_declared,
			LonghandId::AnimationFillMode => longhands::animation_fill_mode::parse_declared,
			LonghandId::AnimationIterationCount => longhands::animation_iteration_count::parse_declared,
			LonghandId::AnimationName => longhands::animation_name::parse_declared,
			LonghandId::AnimationPlayState => longhands::animation_play_state::parse_declared,
			LonghandId::AnimationTimingFunction => longhands::animation_timing_function::parse_declared,
			LonghandId::BackgroundAttachment => longhands::background_attachment::parse_declared,
			LonghandId::BackgroundClip => longhands::background_clip::parse_declared,
			LonghandId::BackgroundImage => longhands::background_image::parse_declared,
			LonghandId::BackgroundOrigin => longhands::background_origin::parse_declared,
			LonghandId::BackgroundPositionX => longhands::background_position_x::parse_declared,
			LonghandId::BackgroundPositionY => longhands::background_position_y::parse_declared,
			LonghandId::BackgroundRepeat => longhands::background_repeat::parse_declared,
			LonghandId::BackgroundSize => longhands::background_size::parse_declared,
			LonghandId::BorderImageOutset => longhands::border_image_outset::parse_declared,
			LonghandId::BorderImageSlice => longhands::border_image_slice::parse_declared,
			LonghandId::BorderImageWidth => longhands::border_image_width::parse_declared,
			LonghandId::BorderSpacing => longhands::border_spacing::parse_declared,
			LonghandId::BoxShadow => longhands::box_shadow::parse_declared,
			LonghandId::Clip => longhands::clip::parse_declared,
			LonghandId::Color => longhands::color::parse_declared,
			LonghandId::ColumnGap => longhands::column_gap::parse_declared,
			LonghandId::ColumnWidth => longhands::column_width::parse_declared,
			LonghandId::Content => longhands::content::parse_declared,
			LonghandId::CounterIncrement => longhands::counter_increment::parse_declared,
			LonghandId::CounterReset => longhands::counter_reset::parse_declared,
			LonghandId::Cursor => longhands::cursor::parse_declared,
			LonghandId::Filter => longhands::filter::parse_declared,
			LonghandId::FlexBasis => longhands::flex_basis::parse_declared,
			LonghandId::FontFamily => longhands::font_family::parse_declared,
			LonghandId::FontSize => longhands::font_size::parse_declared,
			LonghandId::LetterSpacing => longhands::letter_spacing::parse_declared,
			LonghandId::LineHeight => longhands::line_height::parse_declared,
			LonghandId::OutlineOffset => longhands::outline_offset::parse_declared,
			LonghandId::Perspective => longhands::perspective::parse_declared,
			LonghandId::PerspectiveOrigin => longhands::perspective_origin::parse_declared,
			LonghandId::Quotes => longhands::quotes::parse_declared,
			LonghandId::Rotate => longhands::rotate::parse_declared,
			LonghandId::Scale => longhands::scale::parse_declared,
			LonghandId::TextIndent => longhands::text_indent::parse_declared,
			LonghandId::TextOverflow => longhands::text_overflow::parse_declared,
			LonghandId::TextShadow => longhands::text_shadow::parse_declared,
			LonghandId::Transform => longhands::transform::parse_declared,
			LonghandId::TransformOrigin => longhands::transform_origin::parse_declared,
			LonghandId::TransitionDelay => longhands::transition_delay::parse_declared,
			LonghandId::TransitionDuration => longhands::transition_duration::parse_declared,
			LonghandId::TransitionProperty => longhands::transition_property::parse_declared,
			LonghandId::TransitionTimingFunction => longhands::transition_timing_function::parse_declared,
			LonghandId::Translate => longhands::translate::parse_declared,
			LonghandId::VerticalAlign => longhands::vertical_align::parse_declared,
			LonghandId::WordSpacing => longhands::word_spacing::parse_declared,
			LonghandId::BorderImageSource => longhands::border_image_source::parse_declared,
			LonghandId::ListStyleImage => longhands::list_style_image::parse_declared,
			LonghandId::MaxBlockSize => longhands::max_block_size::parse_declared,
			LonghandId::MaxHeight => longhands::max_height::parse_declared,
			LonghandId::MaxInlineSize => longhands::max_inline_size::parse_declared,
			LonghandId::MaxWidth => longhands::max_width::parse_declared,
			LonghandId::BorderBottomLeftRadius => longhands::border_bottom_left_radius::parse_declared,
			LonghandId::BorderBottomRightRadius => longhands::border_bottom_right_radius::parse_declared,
			LonghandId::BorderEndEndRadius => longhands::border_end_end_radius::parse_declared,
			LonghandId::BorderEndStartRadius => longhands::border_end_start_radius::parse_declared,
			LonghandId::BorderStartEndRadius => longhands::border_start_end_radius::parse_declared,
			LonghandId::BorderStartStartRadius => longhands::border_start_start_radius::parse_declared,
			LonghandId::BorderTopLeftRadius => longhands::border_top_left_radius::parse_declared,
			LonghandId::BorderTopRightRadius => longhands::border_top_right_radius::parse_declared,
			LonghandId::PaddingBlockEnd => longhands::padding_block_end::parse_declared,
			LonghandId::PaddingBlockStart => longhands::padding_block_start::parse_declared,
			LonghandId::PaddingBottom => longhands::padding_bottom::parse_declared,
			LonghandId::PaddingInlineEnd => longhands::padding_inline_end::parse_declared,
			LonghandId::PaddingInlineStart => longhands::padding_inline_start::parse_declared,
			LonghandId::PaddingLeft => longhands::padding_left::parse_declared,
			LonghandId::PaddingRight => longhands::padding_right::parse_declared,
			LonghandId::PaddingTop => longhands::padding_top::parse_declared,
			LonghandId::BlockSize => longhands::block_size::parse_declared,
			LonghandId::Height => longhands::height::parse_declared,
			LonghandId::InlineSize => longhands::inline_size::parse_declared,
			LonghandId::MinBlockSize => longhands::min_block_size::parse_declared,
			LonghandId::MinHeight => longhands::min_height::parse_declared,
			LonghandId::MinInlineSize => longhands::min_inline_size::parse_declared,
			LonghandId::MinWidth => longhands::min_width::parse_declared,
			LonghandId::Width => longhands::width::parse_declared,
			LonghandId::BorderBlockEndWidth => longhands::border_block_end_width::parse_declared,
			LonghandId::BorderBlockStartWidth => longhands::border_block_start_width::parse_declared,
			LonghandId::BorderBottomWidth => longhands::border_bottom_width::parse_declared,
			LonghandId::BorderInlineEndWidth => longhands::border_inline_end_width::parse_declared,
			LonghandId::BorderInlineStartWidth => longhands::border_inline_start_width::parse_declared,
			LonghandId::BorderLeftWidth => longhands::border_left_width::parse_declared,
			LonghandId::BorderRightWidth => longhands::border_right_width::parse_declared,
			LonghandId::BorderTopWidth => longhands::border_top_width::parse_declared,
			LonghandId::OutlineWidth => longhands::outline_width::parse_declared,
			LonghandId::BackgroundColor => longhands::background_color::parse_declared,
			LonghandId::BorderBlockEndColor => longhands::border_block_end_color::parse_declared,
			LonghandId::BorderBlockStartColor => longhands::border_block_start_color::parse_declared,
			LonghandId::BorderBottomColor => longhands::border_bottom_color::parse_declared,
			LonghandId::BorderInlineEndColor => longhands::border_inline_end_color::parse_declared,
			LonghandId::BorderInlineStartColor => longhands::border_inline_start_color::parse_declared,
			LonghandId::BorderLeftColor => longhands::border_left_color::parse_declared,
			LonghandId::BorderRightColor => longhands::border_right_color::parse_declared,
			LonghandId::BorderTopColor => longhands::border_top_color::parse_declared,
			LonghandId::OutlineColor => longhands::outline_color::parse_declared,
			LonghandId::Bottom => longhands::bottom::parse_declared,
			LonghandId::InsetBlockEnd => longhands::inset_block_end::parse_declared,
			LonghandId::InsetBlockStart => longhands::inset_block_start::parse_declared,
			LonghandId::InsetInlineEnd => longhands::inset_inline_end::parse_declared,
			LonghandId::InsetInlineStart => longhands::inset_inline_start::parse_declared,
			LonghandId::Left => longhands::left::parse_declared,
			LonghandId::MarginBlockEnd => longhands::margin_block_end::parse_declared,
			LonghandId::MarginBlockStart => longhands::margin_block_start::parse_declared,
			LonghandId::MarginBottom => longhands::margin_bottom::parse_declared,
			LonghandId::MarginInlineEnd => longhands::margin_inline_end::parse_declared,
			LonghandId::MarginInlineStart => longhands::margin_inline_start::parse_declared,
			LonghandId::MarginLeft => longhands::margin_left::parse_declared,
			LonghandId::MarginRight => longhands::margin_right::parse_declared,
			LonghandId::MarginTop => longhands::margin_top::parse_declared,
			LonghandId::Right => longhands::right::parse_declared,
			LonghandId::Top => longhands::top::parse_declared,
			LonghandId::CounterSet => longhands::counter_set::parse_declared,
		};
		parser_func(context, input)
	}

	pub fn cascade<'a>(&self, declaration: Option<&PropertyDeclaration>, context: &'a mut StyleContext) {
		let cascade_func = match self {
			LonghandId::BackgroundColor => longhands::background_color::cascade_property,
			LonghandId::Color => longhands::color::cascade_property,
			LonghandId::Display => longhands::display::cascade_property,
			LonghandId::FontFamily => longhands::font_family::cascade_property,
			LonghandId::FontSize => longhands::font_size::cascade_property,
			LonghandId::Height => longhands::height::cascade_property,
			LonghandId::MarginBottom => longhands::margin_bottom::cascade_property,
			LonghandId::MarginLeft => longhands::margin_left::cascade_property,
			LonghandId::MarginRight => longhands::margin_right::cascade_property,
			LonghandId::MarginTop => longhands::margin_top::cascade_property,
			LonghandId::MinWidth => longhands::min_width::cascade_property,
			LonghandId::MinHeight => longhands::min_height::cascade_property,
			LonghandId::MaxWidth => longhands::max_width::cascade_property,
			LonghandId::MaxHeight => longhands::max_height::cascade_property,
			LonghandId::PaddingBottom => longhands::padding_bottom::cascade_property,
			LonghandId::PaddingLeft => longhands::padding_left::cascade_property,
			LonghandId::PaddingRight => longhands::padding_right::cascade_property,
			LonghandId::PaddingTop => longhands::padding_top::cascade_property,
			LonghandId::Width => longhands::width::cascade_property,
			_ => return,
		};
		cascade_func(declaration, context);
	}

	pub fn is_early_property(&self) -> bool {
		matches!(
			*self,
			// Needed to compute the first available font, in order to
			// compute font-relative units correctly.
			LonghandId::FontSize |
            LonghandId::FontWeight |
            LonghandId::FontStretch |
            LonghandId::FontStyle |
            LonghandId::FontFamily |

            // Needed to properly compute the writing mode, to resolve logical
            // properties, and similar stuff.
            LonghandId::WritingMode |
            LonghandId::Direction
		)
	}

	pub fn ids(phase: PhaseOrder) -> LonghandIdPhaseIterator {
		LonghandIdPhaseIterator { index: 0, phase }
	}
}

#[derive(PartialEq, Eq)]
pub enum PhaseOrder {
	Early,
	Other,
	All,
}

pub struct LonghandIdPhaseIterator {
	index: u16,
	phase: PhaseOrder,
}

impl LonghandIdPhaseIterator {
	pub fn take(&mut self) -> Option<LonghandId> {
		let mut id: Option<LonghandId> = None;
		loop {
			id = LonghandId::try_from(self.index).ok();
			if let Some(id) = id {
				if self.phase == PhaseOrder::All {
					break;
				} else if self.phase == PhaseOrder::Early && id.is_early_property() {
					break;
				} else if self.phase == PhaseOrder::Other && !id.is_early_property() {
					break;
				}
			} else {
				break;
			}
			self.index += 1;
		}
		id
	}
}

impl Iterator for LonghandIdPhaseIterator {
	type Item = LonghandId;

	fn next(&mut self) -> Option<Self::Item> {
		let current = self.take();
		self.index += 1;
		current
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
