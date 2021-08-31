use common::vector::permutate;
use dyn_fmt::AsStrFormatExt;

pub fn url_data() -> Vec<(String, String)> {
	let mut data = Vec::with_capacity(1);
	for (input, output) in [(
		"url(http://www.example.org/style/basic.css)",
		r#"url("http://www.example.org/style/basic.css")"#,
	)]
	.iter()
	{
		data.push((input.to_string(), output.to_string()))
	}

	for ((name, value), modifier) in permutate(
		permutate(
			["url({}{})", "src({}{})"].iter(),
			[r#""http://www.example.com/pinkish.gif""#].iter(),
		)
		.iter(),
		[
			"",
			" prefetch",
			" var(--foo)",
			" defer toggle(italic, normal)",
		]
		.iter(),
	)
	.iter()
	{
		let input = name.format(&[value, modifier]);
		data.push((input.clone(), input.clone()));
	}
	data
}
