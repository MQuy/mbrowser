use std::cell::RefCell;
use std::rc::Rc;

use css::error_reporting::{ContextualParseError, ParseErrorReporter};
use css::media_queries::media_list::MediaList;
use css::stylesheets::origin::Origin;
use css::stylesheets::stylesheet::Stylesheet;
use cssparser::SourceLocation;
use dom::global_scope::{GlobalScope, NodeRef};
use dom::inheritance::Castable;
use dom::parser::DomParser;
use dom::window::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use html5ever::driver;
use html5ever::tendril::{StrTendril, TendrilSink};
use iced_wgpu::{wgpu, Backend, Color, Primitive, Renderer, Settings, Viewport};
use iced_winit::futures::task::SpawnExt;
use iced_winit::winit::dpi::LogicalSize;
use iced_winit::winit::event::Event;
use iced_winit::winit::event_loop::{ControlFlow, EventLoop};
use iced_winit::{futures, mouse, winit, Background, Debug, Font, Point, Rectangle, Size};
use layout::display_list::builder::DisplayListBuilder;
use layout::display_list::display_item::{DisplayItem, LayoutRect};
use layout::flow::tree::BoxTree;
use layout::fonts;
use layout::style_tree::StyleTree;
use selectors::context::QuirksMode;

#[derive(Debug)]
pub struct CSSError {
	pub line: u32,
	pub column: u32,
	pub message: String,
}

pub struct TestingErrorReporter {
	errors: RefCell<Vec<CSSError>>,
}

impl TestingErrorReporter {
	pub fn new() -> Self {
		TestingErrorReporter {
			errors: RefCell::new(Vec::new()),
		}
	}
}

impl ParseErrorReporter for TestingErrorReporter {
	fn report_error(&self, location: SourceLocation, error: ContextualParseError) {
		self.errors.borrow_mut().push(CSSError {
			line: location.line,
			column: location.column,
			message: error.to_string(),
		})
	}
}

fn to_rectangle(bounds: &LayoutRect) -> Rectangle {
	Rectangle::new(
		Point::new(bounds.min_x().get(), bounds.min_y().get()),
		Size::new(bounds.size.width.get(), bounds.size.height.get()),
	)
}

fn main() {
	GlobalScope::clear();
	let sink = DomParser::new();

	let mut parser = driver::parse_document(sink, Default::default());
	parser.process(StrTendril::from(
		r#"
<div class="a">
  <div class="b">
    <div class="c">
      <div class="d">
        <div class="e">
          <div class="f">
            <div class="g">
                <div style="color: black; background-color: white; width: 150px">
                    Hello darkness my old friend
                    <div id="hello" style="display: inline-block; height: 40px; padding-left: 4px">
                        <div>bla bla bla bla</div>
                    </div>
                    <p><span>mBrowser here</span></p>
                </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
"#,
	));

	let output = parser.finish();

	let error_reporter = TestingErrorReporter::new();
	let media = Rc::new(MediaList::empty());
	let stylesheet = Stylesheet::from_str(
		r#"
.a, .b, .c, .d, .e, .f, .g { padding: 12px; }
.a { background-color: #ff0000; }
.b { background-color: #ffa500; }
.c { background-color: #ffff00; }
.d { background-color: #008000; }
.e { background-color: #0000ff; }
.f { background-color: #4b0082; }
.g { background-color: #800080; }
        "#,
		Origin::UserAgent,
		media,
		Some(&error_reporter),
		QuirksMode::NoQuirks,
		0,
	);
	let root = output.document.upcast().first_child().unwrap();

	let style_tree = Rc::new(StyleTree::new(NodeRef(root.clone()), QuirksMode::NoQuirks));
	style_tree.import_user_agent();
	style_tree.add_stylesheet(&stylesheet);
	style_tree.match_rules();
	style_tree.cascade();

	let box_tree = Rc::new(BoxTree::construct(style_tree));
	box_tree.compute_layout();

	let display_list = DisplayListBuilder::construct(box_tree);

	let event_loop = EventLoop::new();
	let window = winit::window::Window::new(&event_loop).unwrap();
	window.set_inner_size(LogicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT));

	let physical_size = window.inner_size();
	let viewport = Viewport::with_physical_size(
		Size::new(physical_size.width, physical_size.height),
		window.scale_factor(),
	);

	// Initialize wgpu
	let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
	let surface = unsafe { instance.create_surface(&window) };

	let (format, (mut device, queue)) = futures::executor::block_on(async {
		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::HighPerformance,
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			})
			.await
			.expect("Request adapter");

		(
			surface.get_preferred_format(&adapter).expect("Get preferred format"),
			adapter
				.request_device(
					&wgpu::DeviceDescriptor {
						label: None,
						features: wgpu::Features::empty(),
						limits: wgpu::Limits::default(),
					},
					None,
				)
				.await
				.expect("Request device"),
		)
	});

	{
		let size = window.inner_size();

		surface.configure(
			&device,
			&wgpu::SurfaceConfiguration {
				usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
				format,
				width: size.width,
				height: size.height,
				present_mode: wgpu::PresentMode::Mailbox,
			},
		)
	};
	let mut resized = false;

	// Initialize staging belt and local pool
	let mut staging_belt = wgpu::util::StagingBelt::new(5 * 1024);
	let mut local_pool = futures::executor::LocalPool::new();

	// Initialize iced
	let debug = Debug::new();
	let mut renderer = Renderer::new(Backend::new(&mut device, Settings::default(), format));

	// Run event loop
	event_loop.run(move |event, _, control_flow| {
		// You should change this if you want to render continuosly
		*control_flow = ControlFlow::Wait;

		match event {
			Event::WindowEvent { event, .. } => match event {
				winit::event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
				_ => {},
			},
			Event::MainEventsCleared => {
				// If there are events pending
			},
			Event::RedrawRequested(_) => {
				if resized {
					let size = window.inner_size();

					surface.configure(
						&device,
						&wgpu::SurfaceConfiguration {
							usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
							format,
							width: size.width,
							height: size.height,
							present_mode: wgpu::PresentMode::Mailbox,
						},
					);

					resized = false;
				}

				match surface.get_current_texture() {
					Ok(frame) => {
						let mut encoder =
							device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

						let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

						// And then iced on top
						let backend = renderer.backend_mut();
						for item in display_list.items.iter() {
							let primitive = match item {
								DisplayItem::Rectangle(rectangle) => Primitive::Quad {
									bounds: to_rectangle(&rectangle.bounds),
									background: Background::Color(Color::from_rgba8(
										rectangle.color.red,
										rectangle.color.green,
										rectangle.color.blue,
										rectangle.color.alpha,
									)),
									border_radius: 0.0,
									border_width: 0.0,
									border_color: Color::TRANSPARENT,
								},
								DisplayItem::Text(text) => {
									let (font_name, font_bytes) = fonts::load_cached_font(&text.font_families);
									Primitive::Text {
										content: text.content.clone(),
										bounds: to_rectangle(&text.bounds),
										color: Color::from_rgba8(
											text.color.red,
											text.color.green,
											text.color.blue,
											text.color.alpha,
										),
										font: Font::External {
											name: font_name,
											bytes: font_bytes,
										},
										horizontal_alignment: iced_winit::alignment::Horizontal::Left,
										size: text.font_size,
										vertical_alignment: iced_winit::alignment::Vertical::Top,
									}
								},
							};
							backend.draw(
								&mut device,
								&mut staging_belt,
								&mut encoder,
								&view,
								&viewport,
								&(primitive, mouse::Interaction::Idle),
								&debug.overlay(),
							);
						}

						// Then we submit the work
						staging_belt.finish();
						queue.submit(Some(encoder.finish()));
						frame.present();

						// And recall staging buffers
						local_pool
							.spawner()
							.spawn(staging_belt.recall())
							.expect("Recall staging buffers");

						local_pool.run_until_stalled();
					},
					Err(error) => match error {
						wgpu::SurfaceError::OutOfMemory => {
							panic!("Swapchain error: {}. Rendering cannot continue.", error)
						},
						_ => {
							// Try rendering again next frame.
							window.request_redraw();
						},
					},
				}
			},
			_ => {},
		}
	})
}
