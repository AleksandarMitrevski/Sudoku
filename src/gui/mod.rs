// holds our own GUI drawing logic
mod logic;

pub use self::logic::initialize;

/// GUI initialization and event loop
pub fn start() {
	use self::logic;
	use conrod::{
		self,
		backend::glium::glium::{self, Surface}
	};

	logic::check_initialized("gui::start() called when gui::initialize() has not been called before");
	
	// resulting window is resizable, this is preferred size
	const WINDOW_WIDTH: u32 = 560;
	const WINDOW_HEIGHT: u32 = 570;

	// Build the window.
	let mut events_loop = glium::glutin::EventsLoop::new();
	let window = glium::glutin::WindowBuilder::new()
		.with_title("Sudoku")
		.with_dimensions((WINDOW_WIDTH, WINDOW_HEIGHT).into());
	let context = glium::glutin::ContextBuilder::new()
		.with_vsync(true)
		.with_multisampling(4);
	let display = glium::Display::new(window, context, &events_loop).unwrap();

	// construct our `Ui`.
	let mut ui = conrod::UiBuilder::new([WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64]).build();
	
	let ids = logic::generate_widget_ids(&mut ui);

	// Add a `Font` to the `Ui`'s `font::Map` from file.
	ui.fonts.insert_from_file("./resources/fonts/NotoSans/NotoSans-Regular.ttf").expect("can not load font resource");

	// A type used for converting `conrod::render::Primitives` into `Command`s that can be used for drawing to the glium `Surface`.
	let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

	// The image map describing each of our widget->image mappings (in our case, none).
	let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

	let mut events = Vec::new();

	'render: loop {
		events.clear();

		// Get all the new events since the last frame.
		events_loop.poll_events(|event| { events.push(event); });

		// If there are no new events, wait for one.
		if events.is_empty() {
			events_loop.run_forever(|event| {
				events.push(event);
				glium::glutin::ControlFlow::Break
			});
		}

		// Process the events.
		for event in events.drain(..) {
			// Break from the loop upon `Escape` or closed window.
			match event.clone() {
				glium::glutin::Event::WindowEvent { event, .. } => {
					match event {
						glium::glutin::WindowEvent::CloseRequested |
						glium::glutin::WindowEvent::KeyboardInput {
							input: glium::glutin::KeyboardInput {
								virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
								..
							},
							..
						} => {
							logic::on_exit_event();
							break 'render
						},
						_ => (),
					}
				}
				_ => (),
			};

			// Use the `winit` backend feature to convert the winit event to a conrod input.
			let input = match ::conrod::backend::winit::convert_event(event, &display) {
				None => continue,
				Some(input) => input,
			};

			// Handle the input with the `Ui`.
			ui.handle_event(input);

			// Set the widgets.
			let ui = &mut ui.set_widgets();
			
			logic::draw_ui(ui, &ids);
		}

		// Draw the `Ui` if it has changed.
		if let Some(primitives) = ui.draw_if_changed() {
			renderer.fill(&display, primitives, &image_map);
			let mut target = display.draw();
			target.clear_color(0.9418, 0.9418, 0.9418, 1.0);
			renderer.draw(&display, &mut target, &image_map).unwrap();
			target.finish().unwrap();
		}
	}
}