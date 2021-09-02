use url::url_data;

#[path = "./url.rs"]
mod url;

pub fn filter_list_data() -> Vec<(String, String)> {
	let mut data = url_data();
	for (input, output) in [
		("blur()", "blur(0px)"),
		("blur(5.5px)", "blur(5.5px)"),
		("brightness()", "brightness(1)"),
		("brightness(.25)", "brightness(0.25)"),
		("brightness(100%)", "brightness(100%)"),
		("contrast()", "contrast(1)"),
		("contrast(5)", "contrast(5)"),
		("contrast(15.5%)", "contrast(15.5%)"),
		(
			"drop-shadow(10px 0.5px)",
			"drop-shadow(transparent 10px 0.5px 0px)",
		),
		(
			"drop-shadow(currentColor 12px 2.25px 0px)",
			"drop-shadow(currentcolor 12px 2.25px 0px)",
		),
		("grayscale()", "grayscale(1)"),
		("grayscale(10)", "grayscale(10)"),
		("grayscale(25%)", "grayscale(25%)"),
		("hue-rotate()", "hue-rotate(0deg)"),
		("hue-rotate(5.25deg)", "hue-rotate(5.25deg)"),
		("hue-rotate(0)", "hue-rotate(0deg)"),
		("invert()", "invert(1)"),
		("invert(2.5)", "invert(2.5)"),
		("invert(86%)", "invert(86%)"),
		("opacity()", "opacity(1)"),
		("opacity(100)", "opacity(100)"),
		("opacity(90%)", "opacity(90%)"),
		("saturate()", "saturate(1)"),
		("saturate(10.0)", "saturate(10)"),
		("saturate(9.5%)", "saturate(9.5%)"),
		("sepia()", "sepia(1)"),
		("sepia(1)", "sepia(1)"),
		("sepia(12%)", "sepia(12%)"),
	]
	.iter()
	{
		data.push((input.to_string(), output.to_string()));
	}
	data
}
