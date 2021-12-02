use color::{
	color_data, device_cmyk_data, hsl_or_hwb_data, hue_3digits_data, hue_4digits_data, hue_6digits_data,
	hue_8digits_data, keyword_data, lab_data, lch_data, rgb_data,
};
use setup::assert_property;

#[path = "../values/color.rs"]
mod color;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	border-right-color: {};
}}"#;

test_property!(keyword, keyword_data);

test_property!(hue_6digits, hue_6digits_data);

test_property!(hue_8digits, hue_8digits_data);

test_property!(hue_3digits, hue_3digits_data);

test_property!(hue_4digits, hue_4digits_data);

test_property!(rgb, rgb_data);

test_property!(hsl_or_hwb, hsl_or_hwb_data);

test_property!(lab, lab_data);

test_property!(lch, lch_data);

test_property!(color, color_data);

test_property!(device_cmyk, device_cmyk_data);
