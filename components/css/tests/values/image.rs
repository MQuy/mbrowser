use angle::angle_data;
use color::color_data;
use common::vector::permutate;
use css::str::join_strings;
use layout::resolution_data;
use length::length_percentage_data;

use super::url::url_data;

#[path = "./angle.rs"]
mod angle;
#[path = "./color.rs"]
mod color;
#[path = "./layout.rs"]
mod layout;
#[path = "./length.rs"]
mod length;

fn padding(text: &str) -> String {
	if text.len() == 0 {
		"".to_string()
	} else {
		std::format!(" {}", text)
	}
}

pub fn keyword_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for value in ["none", "none, none"].iter() {
		data.push((value.to_string(), value.to_string()))
	}
	data
}

pub fn image_data() -> Vec<(String, String)> {
	fn standardize(src: &String, color: &String) -> (String, String) {
		let src = padding(src);
		let color = padding(color);
		let color = if src.len() > 0 && color.len() > 0 {
			std::format!(",{}", color)
		} else {
			color
		};
		(src, color)
	}

	let mut url_data = url_data();
	url_data.push(("".to_string(), "".to_string()));
	let mut color_data = color_data();
	color_data.push(("".to_string(), "".to_string()));

	let mut data = Vec::with_capacity(1);
	for ((tag, (src_input, src_output)), (color_input, color_output)) in permutate(
		permutate(["ltr", "rtl"].iter(), url_data.iter()).iter(),
		color_data.iter(),
	)
	.iter()
	{
		if src_input.len() == 0 && color_input.len() == 0 {
			continue;
		}
		let (src_input, color_input) = standardize(src_input, color_input);
		let (src_output, color_output) = standardize(src_output, color_output);
		let input = std::format!("image({}{}{})", tag, src_input, color_input);
		let output = std::format!("image({}{}{})", tag, src_output, color_output);
		data.push((input, output));
	}
	data
}

pub fn image_set_data() -> Vec<(String, String)> {
	let mut resolution_data = resolution_data();
	resolution_data.push(("".to_string(), "".to_string()));

	let mut data = Vec::with_capacity(1);
	for (index, (reference, ((resolution_input, resolution_output), mime))) in permutate(
		["element(#foo)", "\"something\""].iter(),
		permutate(
			resolution_data.iter(),
			["type(\"hello world\")", "type(\"text/html\")", ""].iter(),
		)
		.iter(),
	)
	.iter()
	.enumerate()
	{
		if resolution_input.len() == 0 && mime.len() == 0 {
			continue;
		}
		let resolution_input = padding(resolution_input);
		let mime = padding(mime);
		let resolution_output = padding(resolution_output);
		let input = std::format!("image-set({}{}{})", reference, resolution_input, mime);
		let output = std::format!(
			"image-set({}{}{})",
			reference,
			if resolution_output.len() == 0 {
				" 1dppx".to_string()
			} else {
				resolution_output
			},
			mime
		);
		if index == 0 {
			data.push((
				std::format!("{}, {}", input, input),
				std::format!("{}, {}", output, output),
			))
		}
		data.push((input, output));
	}
	data
}

pub fn cross_fade_data() -> Vec<(String, String)> {
	let mut fade_data = color_data();
	fade_data.push(("currentColor".to_string(), "currentcolor".to_string()));
	fade_data.push(("element(#foo)".to_string(), "element(#foo)".to_string()));
	let mut data = Vec::with_capacity(1);
	for (index, (percentage, (fade_input, fade_output))) in
		permutate(["0%", "5.5%", "100%"].iter(), fade_data.iter())
			.iter()
			.enumerate()
	{
		let input = std::format!("cross-fade({} {})", percentage, fade_input);
		let output = std::format!("cross-fade({} {})", percentage, fade_output);
		if index == 0 {
			data.push((
				std::format!("{}, {}", input, input),
				std::format!("{}, {}", output, output),
			))
		}
		data.push((input, output))
	}
	data
}

pub fn element_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for value in ["element(#foo)", "element(#class123)"].iter() {
		data.push((value.to_string(), value.to_string()))
	}
	data
}

fn side_or_corner() -> Vec<String> {
	let mut data = Vec::with_capacity(1);
	for (side, corner) in
		permutate(["left", "right", ""].iter(), ["top", "bottom", ""].iter()).iter()
	{
		if side.len() == 0 && corner.len() == 0 {
			continue;
		}
		let value = join_strings(vec![side, corner], " ");
		data.push(value);
	}
	data
}

fn color_stop_list() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in [
		(
			"currentColor, 10px, green 10%",
			"currentcolor, 10px, rgb(0 128 0 / 1) 10%",
		),
		("transparent, transparent", "transparent, transparent"),
		(
			"currentColor 1.5px, 2px, red, red 10%, blue",
			"currentcolor 1.5px, 2px, rgb(255 0 0 / 1), rgb(255 0 0 / 1) 10%, rgb(0 0 255 / 1)",
		),
	]
	.iter()
	{
		data.push((input.to_string(), output.to_string()));
	}
	data
}

pub fn linear_gradient_data() -> Vec<(String, String)> {
	let mut direction = vec!["".to_string()];
	side_or_corner()
		.iter()
		.for_each(|value| direction.push(std::format!("to {}", value)));
	angle_data()
		.iter()
		.for_each(|(angle, _)| direction.push(angle.to_string()));
	let mut data = Vec::with_capacity(1);
	for (name, (direction, (color_input, color_output))) in permutate(
		["linear-gradient", "repeating-linear-gradient"].iter(),
		permutate(direction.iter(), color_stop_list().iter()).iter(),
	)
	.iter()
	{
		let input = std::format!(
			"{}({})",
			name,
			join_strings(vec![direction, color_input], ", ")
		);
		let output = std::format!(
			"{}({})",
			name,
			join_strings(
				vec![
					if direction.len() == 0 {
						"to bottom"
					} else {
						direction
					},
					color_output
				],
				", "
			)
		);
		data.push((input, output));
	}
	data
}

pub fn radial_gradient_data() -> Vec<(String, String)> {
	let mut radial = Vec::with_capacity(1);
	for ((shape_input, shape_output), (position_input, position_output)) in permutate(
		[
			("circle", "circle farthest-corner"),
			("ellipse", "ellipse farthest-corner"),
			("circle closest-side", "circle closest-side"),
			("circle farthest-side", "circle farthest-side"),
			("circle closest-corner", "circle closest-corner"),
			("circle farthest-corner", "circle farthest-corner"),
			("", "ellipse farthest-corner"),
		]
		.iter(),
		[
			("at left top", "at left top"),
			("", "at center center"),
			("at top", "at center top"),
		]
		.iter(),
	)
	.iter()
	{
		let input = join_strings(vec![shape_input, position_input], " ");
		let output = join_strings(vec![shape_output, position_output], " ");
		radial.push((input, output));
	}

	let mut data = Vec::with_capacity(1);
	for (name, ((radial_input, radial_output), (color_input, color_output))) in permutate(
		["radial-gradient", "repeating-radial-gradient"].iter(),
		permutate(radial.iter(), color_stop_list().iter()).iter(),
	)
	.iter()
	{
		let input = std::format!(
			"{}({})",
			name,
			join_strings(vec![radial_input, color_input], ", ")
		);
		let output = std::format!(
			"{}({})",
			name,
			join_strings(
				vec![
					if radial_output.len() == 0 {
						"to bottom"
					} else {
						radial_output
					},
					color_output
				],
				", "
			)
		);
		data.push((input, output));
	}
	data
}

fn angular_stop_list() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in [
		(
			"currentColor, 10deg, green 10%",
			"currentcolor, 10deg, rgb(0 128 0 / 1) 10%",
		),
		("transparent, transparent", "transparent, transparent"),
		(
			"currentColor 1.5deg, 10%, red, red 10%, blue",
			"currentcolor 1.5deg, 10%, rgb(255 0 0 / 1), rgb(255 0 0 / 1) 10%, rgb(0 0 255 / 1)",
		),
	]
	.iter()
	{
		data.push((input.to_string(), output.to_string()));
	}
	data
}

pub fn cornic_gradient_data() -> Vec<(String, String)> {
	let mut radial = Vec::with_capacity(1);
	let mut angle_data = angle_data();
	angle_data.push(("".to_string(), "0deg".to_string()));
	for ((angle_input, angle_output), (position_input, position_output)) in permutate(
		angle_data.iter(),
		[
			("at left top", "at left top"),
			("", "at center center"),
			("at top", "at center top"),
		]
		.iter(),
	)
	.iter()
	{
		let angle_input = if angle_input.len() == 0 {
			angle_input.to_string()
		} else {
			std::format!("from {}", angle_input)
		};
		let angle_output = if angle_output.len() == 0 {
			angle_output.to_string()
		} else {
			std::format!("from {}", angle_output)
		};
		let input = join_strings(vec![&angle_input, position_input], " ");
		let output = join_strings(vec![&angle_output, position_output], " ");
		radial.push((input, output));
	}

	let mut data = Vec::with_capacity(1);
	for (name, ((cornic_input, cornic_output), (color_input, color_output))) in permutate(
		["conic-gradient", "repeating-conic-gradient"].iter(),
		permutate(radial.iter(), angular_stop_list().iter()).iter(),
	)
	.iter()
	{
		let input = std::format!(
			"{}({})",
			name,
			join_strings(vec![cornic_input, color_input], ", ")
		);
		let output = std::format!(
			"{}({})",
			name,
			join_strings(
				vec![
					if cornic_output.len() == 0 {
						"to bottom"
					} else {
						cornic_output
					},
					color_output
				],
				", "
			)
		);
		data.push((input, output));
	}
	data
}
