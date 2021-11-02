use cssparser::{Parser, ToCss, Token, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use super::angle::{Angle, AnglePercentage};
use super::color::Color;
use super::layout::Resolution;
use super::length::{LengthPercentage, NonNegativeLengthPercentage};
use super::percentage::Percentage;
use super::position::{HorizontalPosition, Position, VerticalPosition};
use crate::parser::{
	parse_in_any_order, parse_item_if_missing, parse_repeated_with_delimitor, ParseError,
};
use crate::properties::declaration::property_keywords_impl;
use crate::str::convert_options_to_string;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::url::CssUrl;
use crate::values::Ident;

#[derive(Clone, Debug)]
pub enum ImageDirection {
	Ltr,
	Rtl,
}

property_keywords_impl! { ImageDirection,
	ImageDirection::Ltr, "ltr",
	ImageDirection::Rtl, "rtl",
}

#[derive(Clone, Debug)]
pub struct Annotation {
	tag: Option<ImageDirection>,
	src: Option<CssUrl>,
	color: Option<Color>,
}

impl Annotation {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let tag = input
			.try_parse(|input| ImageDirection::parse(input))
			.map_or(None, |v| Some(v));
		let src = input
			.try_parse(|input| CssUrl::parse(context, input))
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| CssUrl::parse_string(context, input))
			})
			.ok();
		let color = input
			.try_parse(|input| {
				if src.is_some() {
					input.expect_comma()?;
				}
				Color::parse(context, input)
			})
			.ok();
		Ok(Annotation { tag, src, color })
	}
}

impl ToCss for Annotation {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let value = convert_options_to_string(
			vec![
				self.src.as_ref().map(|v| v.to_css_string()),
				self.color.as_ref().map(|v| v.to_css_string()),
			],
			", ",
		);
		dest.write_fmt(format_args!(
			"{}{}",
			self.tag
				.as_ref()
				.map_or("".to_string(), |v| std::format!("{} ", v.to_css_string())),
			value
		))
	}
}

#[derive(Clone, Debug)]
pub enum ImageReference {
	Image(Image),
	String(CssUrl),
}

impl ImageReference {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let image = Image::parse(context, input)?;
				Ok(ImageReference::Image(image))
			})
			.or_else(|_err: ParseError<'i>| {
				let url = CssUrl::parse_string(context, input)?;
				Ok(ImageReference::String(url))
			})
	}
}

impl ToCss for ImageReference {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			ImageReference::Image(value) => value.to_css(dest),
			ImageReference::String(value) => value.to_css(dest),
		}
	}
}

#[derive(Clone, Debug)]
pub struct ImageSetOption {
	reference: ImageReference,
	resolution: Resolution,
	mime: Option<String>,
}

impl ImageSetOption {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let reference = ImageReference::parse(context, input)?;
		let mut resolution = None;
		let mut mime = None;
		parse_in_any_order(
			input,
			&mut [
				&mut |input| {
					parse_item_if_missing(input, &mut resolution, &mut |_, input| {
						Resolution::parse(context, input)
					})
				},
				&mut |input| {
					parse_item_if_missing(input, &mut mime, &mut |_, input| {
						input.expect_function_matching("type")?;
						input.parse_nested_block(|input| {
							let value = input.expect_string()?.to_string();
							Ok(value)
						})
					})
				},
			],
		);

		if resolution.is_none() && mime.is_none() {
			return Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError));
		}
		Ok(ImageSetOption {
			reference,
			mime,
			resolution: resolution.map_or("1dppx".into(), |v| v),
		})
	}
}

impl ToCss for ImageSetOption {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!(
			"{} {}{}",
			self.reference.to_css_string(),
			self.resolution.to_css_string(),
			self.mime
				.as_ref()
				.map_or("".to_string(), |v| std::format!(" type(\"{}\")", v))
		))
	}
}

#[derive(Clone, Debug)]
pub enum ImageOrColor {
	Image(Image),
	Color(Color),
}

impl ImageOrColor {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let image = Image::parse(context, input)?;
				Ok(ImageOrColor::Image(image))
			})
			.or_else(|_err: ParseError<'i>| {
				let color = Color::parse(context, input)?;
				Ok(ImageOrColor::Color(color))
			})
	}
}

impl ToCss for ImageOrColor {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			ImageOrColor::Image(value) => value.to_css(dest),
			ImageOrColor::Color(value) => value.to_css(dest),
		}
	}
}

#[derive(Clone, Debug)]
pub struct CFImage {
	percentage: Option<Percentage>,
	fade: ImageOrColor,
}

impl CFImage {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let mut percentage = None;
		let mut fade = None;
		parse_in_any_order(
			input,
			&mut [
				&mut |input| {
					parse_item_if_missing(input, &mut percentage, &mut |_, input| {
						Percentage::parse(context, input)
					})
				},
				&mut |input| {
					parse_item_if_missing(input, &mut fade, &mut |_, input| {
						ImageOrColor::parse(context, input)
					})
				},
			],
		);

		if let Some(fade) = fade {
			Ok(CFImage { fade, percentage })
		} else {
			Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
		}
	}
}

impl ToCss for CFImage {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!(
			"{}{}",
			self.percentage
				.as_ref()
				.map_or("".to_string(), |v| std::format!("{} ", v.to_css_string())),
			self.fade.to_css_string()
		))
	}
}

#[derive(Clone, Debug)]
pub enum Side {
	Left,
	Right,
}

property_keywords_impl! { Side,
	Side::Left, "left",
	Side::Right, "right",
}

#[derive(Clone, Debug)]
pub enum Corner {
	Top,
	Bottom,
}

property_keywords_impl! { Corner,
	Corner::Top, "top",
	Corner::Bottom, "bottom",
}

#[derive(Clone, Debug)]
pub enum LineDirection {
	Angle(Angle),
	Keyword(Option<Side>, Option<Corner>),
}

impl LineDirection {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let angle = Angle::parse(context, input)?;
				Ok(LineDirection::Angle(angle))
			})
			.or_else(|_err: ParseError<'i>| {
				let mut side = None;
				let mut corner = None;
				input.expect_ident_matching("to")?;
				parse_in_any_order(
					input,
					&mut [
						&mut |input| {
							parse_item_if_missing(input, &mut side, &mut |_, input| {
								Side::parse(input)
							})
						},
						&mut |input| {
							parse_item_if_missing(input, &mut corner, &mut |_, input| {
								Corner::parse(input)
							})
						},
					],
				);
				Ok(LineDirection::Keyword(side, corner))
			})
	}
}

impl ToCss for LineDirection {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			LineDirection::Angle(value) => value.to_css(dest),
			LineDirection::Keyword(side, corner) => dest.write_fmt(format_args!(
				"to {}",
				convert_options_to_string(
					vec![
						side.as_ref().map(|v| v.to_css_string()),
						corner.as_ref().map(|v| v.to_css_string()),
					],
					" "
				)
			)),
		}
	}
}

#[derive(Clone, Debug)]
pub struct LinearColorStop<T> {
	color: Color,
	length: Option<T>,
}

impl<T> LinearColorStop<T> {
	pub fn parse<'i, 't, P>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
		item_parser: &mut P,
	) -> Result<Self, ParseError<'i>>
	where
		P: FnMut(&mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
	{
		let mut color = None;
		let mut length = None;
		parse_in_any_order(
			input,
			&mut [
				&mut |input| {
					parse_item_if_missing(input, &mut color, &mut |_, input| {
						Color::parse(context, input)
					})
				},
				&mut |input| {
					parse_item_if_missing(input, &mut length, &mut |_, input| item_parser(input))
				},
			],
		);

		if let Some(color) = color {
			Ok(LinearColorStop { color, length })
		} else {
			Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
		}
	}
}

impl<T: ToCss> ToCss for LinearColorStop<T> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!(
			"{}{}",
			self.color.to_css_string(),
			self.length
				.as_ref()
				.map_or("".to_string(), |v| std::format!(" {}", v.to_css_string()))
		))
	}
}

#[derive(Clone, Debug)]
pub struct LinearColorHint<T> {
	hint: Option<T>,
	color: LinearColorStop<T>,
}

impl<T> LinearColorHint<T> {
	pub fn parse<'i, 't, P>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
		item_parser: &mut P,
	) -> Result<Self, ParseError<'i>>
	where
		P: FnMut(&mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
	{
		let hint = input.try_parse(|input| item_parser(input)).ok();
		if hint.is_some() {
			input.expect_comma()?;
		}
		let color = LinearColorStop::parse(context, input, item_parser)?;
		Ok(LinearColorHint { hint, color })
	}
}

impl<T: ToCss> ToCss for LinearColorHint<T> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!(
			"{}{}",
			self.hint
				.as_ref()
				.map_or("".to_string(), |v| std::format!("{}, ", v.to_css_string())),
			self.color.to_css_string()
		))
	}
}

/// https://drafts.csswg.org/css-images-3/#color-stop-syntax
#[derive(Clone, Debug)]
pub struct ColorStopList<T> {
	starting: LinearColorStop<T>,
	ending: Vec<LinearColorHint<T>>,
}

impl<T> ColorStopList<T> {
	pub fn parse<'i, 't, P>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
		item_parser: &mut P,
	) -> Result<Self, ParseError<'i>>
	where
		P: for<'tt> Fn(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i>>,
	{
		let starting = LinearColorStop::parse(context, input, item_parser)?;
		input.expect_comma()?;
		let ending = parse_repeated_with_delimitor(
			input,
			&mut |input| LinearColorHint::parse(context, input, item_parser),
			&mut |input| {
				input.expect_comma()?;
				Ok(())
			},
			1,
		)?;
		Ok(ColorStopList { starting, ending })
	}
}

impl<T: ToCss> ToCss for ColorStopList<T> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_fmt(format_args!(
			"{}, {}",
			self.starting.to_css_string(),
			self.ending
				.iter()
				.map(|v| v.to_css_string())
				.collect::<Vec<String>>()
				.join(", ")
		))
	}
}

#[derive(Clone, Debug)]
pub struct LinearGradient {
	direction: LineDirection,
	color_stop: ColorStopList<LengthPercentage>,
	repeating: bool,
}

impl LinearGradient {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let direction = input.try_parse(|input| LineDirection::parse(context, input));
		let direction = if direction.is_ok() {
			input.expect_comma()?;
			direction?
		} else {
			LineDirection::Keyword(None, Some(Corner::Bottom))
		};
		let color_stop = ColorStopList::parse(context, input, &mut |input| {
			LengthPercentage::parse(context, input)
		})?;
		Ok(LinearGradient {
			direction,
			color_stop,
			repeating: false,
		})
	}
}

impl ToCss for LinearGradient {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let name = if self.repeating {
			"repeating-linear-gradient"
		} else {
			"linear-gradient"
		};
		dest.write_fmt(format_args!(
			"{}({}, {})",
			name,
			self.direction.to_css_string(),
			self.color_stop.to_css_string()
		))
	}
}

#[derive(Clone, Debug)]
pub enum EndingShape {
	Circle,
	Ellipse,
}

property_keywords_impl! { EndingShape,
	EndingShape::Circle, "circle",
	EndingShape::Ellipse, "ellipse",
}

#[derive(Clone, Debug)]
pub enum RadialSize {
	ClosestSide,
	FarthestSide,
	ClosestCorner,
	FarthestCorner,
	Length(
		NonNegativeLengthPercentage,
		Option<NonNegativeLengthPercentage>,
	),
}

impl RadialSize {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let horizontal = NonNegativeLengthPercentage::parse(context, input)?;
				let veritical = input
					.try_parse(|input| NonNegativeLengthPercentage::parse(context, input))
					.ok();
				Ok(RadialSize::Length(horizontal, veritical))
			})
			.or_else(|_err: ParseError<'i>| {
				let location = input.current_source_location();
				let ident = input.expect_ident()?;
				Ok(match_ignore_ascii_case! { ident,
					"closest-side" => RadialSize::ClosestSide,
					"farthest-side" => RadialSize::FarthestSide,
					"closest-corner" => RadialSize::ClosestCorner,
					"farthest-corner" => RadialSize::FarthestCorner,
					_ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone()))),
				})
			})
	}
}

impl ToCss for RadialSize {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			RadialSize::ClosestSide => dest.write_str("closest-side"),
			RadialSize::FarthestSide => dest.write_str("farthest-side"),
			RadialSize::ClosestCorner => dest.write_str("closest-corner"),
			RadialSize::FarthestCorner => dest.write_str("farthest-corner"),
			RadialSize::Length(horizontal, vertical) => dest.write_str(&convert_options_to_string(
				vec![
					Some(horizontal.to_css_string()),
					vertical.as_ref().map(|v| v.to_css_string()),
				],
				" ",
			)),
		}
	}
}

#[derive(Clone, Debug)]
pub struct RadialGradient {
	end_shape: EndingShape,
	size: RadialSize,
	position: Position,
	color_stop: ColorStopList<LengthPercentage>,
	repeating: bool,
}

impl RadialGradient {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let mut end_shape = None;
		let mut size = None;
		parse_in_any_order(
			input,
			&mut [
				&mut |input| {
					parse_item_if_missing(input, &mut end_shape, &mut |_, input| {
						EndingShape::parse(input)
					})
				},
				&mut |input| {
					parse_item_if_missing(input, &mut size, &mut |_, input| {
						RadialSize::parse(context, input)
					})
				},
			],
		);
		let position = input
			.try_parse(|input| {
				input.expect_ident_matching("at")?;
				Position::parse(context, input)
			})
			.ok();
		if end_shape.is_some() || size.is_some() || position.is_some() {
			input.expect_comma()?;
		}
		let color_stop = ColorStopList::parse(context, input, &mut |input| {
			LengthPercentage::parse(context, input)
		})?;
		let size = size.map_or(RadialSize::FarthestCorner, |v| v);
		let end_shape = if let Some(end_shape) = end_shape {
			end_shape
		} else {
			match &size {
				RadialSize::Length(_, vertical) if vertical.is_none() => EndingShape::Circle,
				_ => EndingShape::Ellipse,
			}
		};
		Ok(RadialGradient {
			end_shape,
			size,
			color_stop,
			repeating: false,
			position: position.map_or(
				Position::new(HorizontalPosition::Center, VerticalPosition::Center),
				|v| v,
			),
		})
	}
}

impl ToCss for RadialGradient {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let name = if self.repeating {
			"repeating-radial-gradient"
		} else {
			"radial-gradient"
		};
		dest.write_fmt(format_args!(
			"{}({} {} at {}, {})",
			name,
			self.end_shape.to_css_string(),
			self.size.to_css_string(),
			self.position.to_css_string(),
			self.color_stop.to_css_string()
		))
	}
}

#[derive(Clone, Debug)]
pub struct ConicRadient {
	angle: Angle,
	position: Position,
	color_stop: ColorStopList<AnglePercentage>,
	repeating: bool,
}

impl ConicRadient {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let angle = input
			.try_parse(|input| {
				input.expect_ident_matching("from")?;
				Angle::parse(context, input)
			})
			.ok();
		let position = input
			.try_parse(|input| {
				input.expect_ident_matching("at")?;
				Position::parse(context, input)
			})
			.ok();

		if angle.is_some() || position.is_some() {
			input.expect_comma()?;
		}
		let color_stop = ColorStopList::parse(context, input, &mut |input| {
			AnglePercentage::parse(context, input)
		})?;
		Ok(ConicRadient {
			color_stop,
			angle: angle.map_or("0deg".into(), |v| v),
			position: position.map_or(
				Position::new(HorizontalPosition::Center, VerticalPosition::Center),
				|v| v,
			),
			repeating: false,
		})
	}
}

impl ToCss for ConicRadient {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let name = if self.repeating {
			"repeating-conic-gradient"
		} else {
			"conic-gradient"
		};
		dest.write_fmt(format_args!(
			"{}(from {} at {}, {})",
			name,
			self.angle.to_css_string(),
			self.position.to_css_string(),
			self.color_stop.to_css_string()
		))
	}
}

#[derive(Clone, Debug)]
pub enum Gradient {
	Linear(LinearGradient),
	Radial(RadialGradient),
	Conic(ConicRadient),
}

impl ToCss for Gradient {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			Gradient::Linear(value) => value.to_css(dest),
			Gradient::Radial(value) => value.to_css(dest),
			Gradient::Conic(value) => value.to_css(dest),
		}
	}
}

/// https://drafts.csswg.org/css-images-4/#image-values
#[derive(Clone, Debug)]
pub enum Image {
	Url(CssUrl),
	Image(Annotation),
	Set(Vec<ImageSetOption>),
	CrossFade(Vec<CFImage>),
	Element(Ident),
	Gradient(Gradient),
}

impl Image {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| {
				let url = CssUrl::parse(context, input)?;
				Ok(Image::Url(url))
			})
			.or_else(|_err: ParseError<'i>| {
				let location = input.current_source_location();
				let name = input.expect_function()?.clone();
				input.parse_nested_block(|input| {
					match_ignore_ascii_case! { &name,
						"image" => Image::parse_image(context, input),
						"image-set" => Image::parse_set(context, input),
						"cross-fade" => Image::parse_cross_fade(context, input),
						"element" => Image::parse_element(context, input),
						"linear-gradient" => Image::parse_linear_gradient(context, input, false),
						"repeating-linear-gradient" => Image::parse_linear_gradient(context, input, true),
						"radial-gradient" => Image::parse_radial_gradient(context, input, false),
						"repeating-radial-gradient" => Image::parse_radial_gradient(context, input, true),
						"conic-gradient" => Image::parse_conic_gradient(context, input, false),
						"repeating-conic-gradient" => Image::parse_conic_gradient(context, input, true),
						_ => return Err(location.new_custom_error(StyleParseErrorKind::UnspecifiedError))
					}
				})
			})
	}

	pub fn parse_image<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let value = Annotation::parse(context, input)?;
		Ok(Image::Image(value))
	}

	pub fn parse_set<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let value = input.parse_comma_separated(|input| ImageSetOption::parse(context, input))?;
		if value.len() == 0 {
			Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
		} else {
			Ok(Image::Set(value))
		}
	}

	pub fn parse_cross_fade<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let value = input.parse_comma_separated(|input| CFImage::parse(context, input))?;
		if value.len() == 0 {
			Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
		} else {
			Ok(Image::CrossFade(value))
		}
	}

	pub fn parse_element<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let location = input.current_source_location();
		let token = input.next()?;
		let id = match token {
			Token::IDHash(value) | Token::Hash(value) => Ident(value.to_string()),
			_ => {
				return Err(
					location.new_custom_error(StyleParseErrorKind::UnexpectedToken(token.clone()))
				)
			},
		};
		Ok(Image::Element(id))
	}

	pub fn parse_linear_gradient<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
		repeating: bool,
	) -> Result<Self, ParseError<'i>> {
		let mut value = LinearGradient::parse(context, input)?;
		value.repeating = repeating;
		Ok(Image::Gradient(Gradient::Linear(value)))
	}

	pub fn parse_radial_gradient<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
		repeating: bool,
	) -> Result<Self, ParseError<'i>> {
		let mut value = RadialGradient::parse(context, input)?;
		value.repeating = repeating;
		Ok(Image::Gradient(Gradient::Radial(value)))
	}

	pub fn parse_conic_gradient<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
		repeating: bool,
	) -> Result<Self, ParseError<'i>> {
		let mut value = ConicRadient::parse(context, input)?;
		value.repeating = repeating;
		Ok(Image::Gradient(Gradient::Conic(value)))
	}
}

impl ToCss for Image {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			Image::Url(value) => value.to_css(dest),
			Image::Image(value) => dest.write_fmt(format_args!("image({})", value.to_css_string())),
			Image::Set(value) => dest.write_fmt(format_args!(
				"image-set({})",
				value
					.iter()
					.map(|v| v.to_css_string())
					.collect::<Vec<String>>()
					.join(", ")
			)),
			Image::CrossFade(value) => dest.write_fmt(format_args!(
				"cross-fade({})",
				value
					.iter()
					.map(|v| v.to_css_string())
					.collect::<Vec<String>>()
					.join(", ")
			)),
			Image::Element(value) => {
				dest.write_fmt(format_args!("element(#{})", value.to_css_string()))
			},
			Image::Gradient(value) => value.to_css(dest),
		}
	}
}
