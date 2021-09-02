use image::{
	cornic_gradient_data, cross_fade_data, element_data, image_data, image_set_data, keyword_data,
	linear_gradient_data, radial_gradient_data,
};
use setup::assert_property;
use url::url_data;

#[path = "../values/image.rs"]
mod image;
#[path = "../values/url.rs"]
mod url;
#[macro_use]
#[path = "../setup/mod.rs"]
mod setup;

const TEMPLATE: &str = r#"
.name {{
	list-style-image: {};
}}"#;

test_property!(keyword, keyword_data);

test_property!(url, url_data);

test_property!(image, image_data);

test_property!(image_set, image_set_data);

test_property!(cross_fade, cross_fade_data);

test_property!(element, element_data);

test_property!(linear_gradient, linear_gradient_data);

test_property!(radial_gradient, radial_gradient_data);

test_property!(cornic_gradient, cornic_gradient_data);
