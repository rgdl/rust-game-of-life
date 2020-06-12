use glium::{glutin, Surface};

pub struct Display { columns: i32, rows: i32, }

struct Colour {
    red: f32,
    green: f32,
    blue: f32,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

glium::implement_vertex!(Vertex, position);

static BACKGROUND: Colour = Colour { red: 0.1, green: 0.0, blue: 0.6 };
static GRIDLINES: Colour = Colour { red: 0.0, green: 1.0, blue: 0.0 };
static CELL: Colour = Colour { red: 1.0, green: 0.0, blue: 0.0 };

static WINDOW_TITLE: &str = "Game of Life";
static WINDOW_HEIGHT: u32 = 500;
static WINDOW_WIDTH: u32 = 800;
static WINDOW_SHAPE: glutin::dpi::LogicalSize<f64> = glutin::dpi::LogicalSize::new(
    WINDOW_WIDTH as f64,
    WINDOW_HEIGHT as f64,
);

impl Display {
    pub fn new(columns: i32, rows: i32) -> Display {
        Display { columns: columns, rows: rows }
    }

    pub fn draw(&self, state: Vec<Vec<bool>>) {
        // use this as a guide: https://github.com/glium/glium/blob/master/book/tuto-01-getting-started.md
        
        let mut event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        display.gl_window().window().set_title(WINDOW_TITLE);
        display.gl_window().window().set_inner_size(WINDOW_SHAPE);

        let shape = vec![
            Vertex { position: [0.0, 1.0] },
            // Vertex { position: [one_pixel_width(), 1.0] },
            // Vertex { position: [one_pixel_width(), -1.0] },
            Vertex { position: [0.0, -1.0] },
        ];

        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
        // Need to do some nasty-looking openGL stuff. Abstract away once it's working

		let vertex_shader_src = "
			#version 140
			in vec2 position;
			void main() {
				gl_Position = vec4(position, 0.0, 1.0);
			}
		";

		let fragment_shader_src = format!("
			#version 140
			out vec4 color;
			void main() {{
				color = vec4({}, {}, {}, 1.0);
			}}
		", GRIDLINES.red, GRIDLINES.green, GRIDLINES.blue);

        let program = glium::Program::from_source(&display, vertex_shader_src, &fragment_shader_src, None).unwrap();

		event_loop.run(move |ev, _, control_flow| {
			let next_frame_time = std::time::Instant::now() +
				std::time::Duration::from_nanos(16_666_667);
			*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
			match ev {
				glutin::event::Event::WindowEvent { event, .. } => match event {
					glutin::event::WindowEvent::CloseRequested => {
						*control_flow = glutin::event_loop::ControlFlow::Exit;
						return;
					},
					_ => return,
				},
				_ => (),
			}

            let mut target = display.draw();
            
            target.clear_color(
                BACKGROUND.red,
                BACKGROUND.green,
                BACKGROUND.blue,
                1.0,
            );

            target.draw( 
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default()
            ).unwrap();

            target.finish().unwrap();
		});
    }
}



