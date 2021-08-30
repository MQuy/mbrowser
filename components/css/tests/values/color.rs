use common::vector::permutate;
use css::values::color::CMYK;
use css::values::number::NumberOrPercentage;
use cssparser::ToCss;

pub fn keyword_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for name in [
		"currentcolor",
		"transparent",
		"canvas",
		"canvastext",
		"linktext",
		"visitedtext",
		"activetext",
		"buttonface",
		"buttontext",
		"buttonborder",
		"field",
		"fieldtext",
		"highlight",
		"highlighttext",
		"mark",
		"marktext",
		"graytext",
	]
	.iter()
	{
		data.push((name.to_string(), name.to_string()));
	}
	data
}

pub fn hue_6digits_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in [("#00ff00", "rgb(0 255 0 / 255)")].iter() {
		data.push((input.to_string(), output.to_string()));
	}
	data
}

pub fn hue_8digits_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in [("#0000ffcc", "rgb(0 0 255 / 204)")].iter() {
		data.push((input.to_string(), output.to_string()));
	}
	data
}

pub fn hue_3digits_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in [("#123", "rgb(17 34 51 / 255)")].iter() {
		data.push((input.to_string(), output.to_string()));
	}
	data
}

pub fn hue_4digits_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in [("#8925", "rgb(136 153 34 / 85)")].iter() {
		data.push((input.to_string(), output.to_string()));
	}
	data
}

pub fn rgb_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (((color, alpha), delimitors), fname) in permutate(
		permutate(
			permutate(
				[("10%", "70%", "0.5%"), ("25.5", "178.5", "1.275")].iter(),
				["25%", "63.75", ""].iter(),
			)
			.iter(),
			[[" ", " ", " / "], [", ", ", ", ", "]].iter(),
		)
		.iter(),
		["rgb", "rgba"].iter(),
	) {
		let input = std::format!(
			"{}({}{}{}{}{}{})",
			fname,
			color.0,
			delimitors[0],
			color.1,
			delimitors[1],
			color.2,
			if alpha.len() > 0 {
				std::format!("{}{}", delimitors[2], alpha)
			} else {
				"".to_string()
			},
		);
		let output = std::format!(
			"rgb(25 178 1 / {})",
			if alpha.len() > 0 { "63" } else { "255" }
		);
		data.push((input, output));
	}
	data
}

pub fn hsl_or_hwb_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (iter, name) in [(["hsl", "hsla"].iter(), "hsl"), (["hwb"].iter(), "hwb")].iter() {
		let angles = permutate(["30", "5.25"].iter(), ["deg", "grad", "rad", "turn"].iter())
			.iter()
			.map(|(value, unit)| std::format!("{}{}", value, unit))
			.collect::<Vec<String>>();
		let hues = [&angles[..], &["5".to_string(), "0.25".to_string()][..]].concat();
		for ((hue, alpha), fname) in permutate(
			permutate(hues.iter(), ["25%", "63.75", ""].iter()).iter(),
			iter.clone(),
		)
		.iter()
		{
			let input = std::format!(
				"{}({} 12% 0.25%{})",
				fname,
				hue,
				if alpha.len() > 0 {
					std::format!(" / {}", alpha)
				} else {
					"".to_string()
				},
			);
			let output = std::format!(
				"{}({} 12% 0.25% / {})",
				name,
				hue,
				if alpha.len() > 0 { "63" } else { "255" },
			);
			data.push((input, output));
		}
	}
	data
}

pub fn lab_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for ((percentage, number), alpha) in permutate(
		permutate(["10%", "0.25%"].iter(), ["5", "1.5"].iter()).iter(),
		["25%", "63.75", ""].iter(),
	)
	.iter()
	{
		let input = std::format!(
			"lab({} {} {}{})",
			percentage,
			number,
			number,
			if alpha.len() > 0 {
				std::format!(" / {}", alpha)
			} else {
				"".to_string()
			},
		);
		let output = std::format!(
			"lab({} {} {} / {})",
			percentage,
			number,
			number,
			if alpha.len() > 0 { "63" } else { "255" },
		);
		data.push((input, output));
	}
	data
}

pub fn lch_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	let angles = permutate(["30", "5.25"].iter(), ["deg", "grad", "rad", "turn"].iter())
		.iter()
		.map(|(value, unit)| std::format!("{}{}", value, unit))
		.collect::<Vec<String>>();
	let hues = [&angles[..], &["5".to_string(), "0.25".to_string()][..]].concat();
	for (((percentage, number), hue), alpha) in permutate(
		permutate(
			permutate(["10%", "0.25%"].iter(), ["5", "1.5"].iter()).iter(),
			hues.iter(),
		)
		.iter(),
		["25%", "63.75", ""].iter(),
	)
	.iter()
	{
		let input = std::format!(
			"lch({} {} {}{})",
			percentage,
			number,
			hue,
			if alpha.len() > 0 {
				std::format!(" / {}", alpha)
			} else {
				"".to_string()
			},
		);
		let output = std::format!(
			"lch({} {} {} / {})",
			percentage,
			number,
			hue,
			if alpha.len() > 0 { "63" } else { "255" },
		);
		data.push((input, output));
	}
	data
}

pub fn color_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for ((ident, value), alpha) in permutate(
		permutate(
			["hello", "--small"].iter(),
			["5", "0.25", "25%", "5.5%", "1 5%", "0.25 5 10%"].iter(),
		)
		.iter(),
		["25%", "63.75", ""].iter(),
	)
	.iter()
	{
		let input = std::format!(
			"color({} {}{})",
			ident,
			value,
			if alpha.len() > 0 {
				std::format!(" / {}", alpha)
			} else {
				"".to_string()
			},
		);
		let output = std::format!(
			"color({} {} / {})",
			ident,
			value,
			if alpha.len() > 0 { "63" } else { "255" },
		);
		data.push((input, output));
	}
	data
}

pub fn device_cmyk_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (((v1, v2), alpha), color) in permutate(
		permutate(
			permutate(["1", "25%"].iter(), ["0.5", "12.5%"].iter()).iter(),
			["25%", "63.75", ""].iter(),
		)
		.iter(),
		["currentcolor", ""].iter(),
	)
	.iter()
	{
		let input = std::format!(
			"device-cmyk({} {} {} {}{}{})",
			v1,
			v2,
			v1,
			v2,
			if alpha.len() > 0 {
				std::format!(" / {}", alpha)
			} else {
				"".to_string()
			},
			if color.len() > 0 {
				std::format!(", {}", color)
			} else {
				"".to_string()
			}
		);
		let output = std::format!(
			"device-cmyk({} {} {} {} / {}{})",
			v1,
			v2,
			v1,
			v2,
			if alpha.len() > 0 {
				std::format!(
					"{}",
					match Into::<NumberOrPercentage>::into(**alpha) {
						NumberOrPercentage::Number(value) => value.get(),
						NumberOrPercentage::Percentage(value) => value.to_value(&(0.0..255.0)),
					} as u8
				)
			} else {
				"255".to_string()
			},
			std::format!(
				", {}",
				if color.len() > 0 {
					color.to_string()
				} else {
					std::format!(
						"rgb({})",
						CMYK {
							cyan: (**v1).into(),
							magenta: (**v2).into(),
							yellow: (**v1).into(),
							black: (**v2).into(),
						}
						.to_rgb()
						.to_css_string()
					)
				}
			)
		);
		data.push((input, output));
	}
	data
}
