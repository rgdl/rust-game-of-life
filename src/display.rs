use glium::{glutin, Surface};

pub struct Display { columns: i32, rows: i32 }

static REFRESH_PERIOD_MS: u64 = 1000;

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
static WINDOW_HEIGHT: i32 = 500;
static WINDOW_WIDTH: i32 = 800;
static WINDOW_SHAPE: glutin::dpi::LogicalSize<f64> = glutin::dpi::LogicalSize::new(
    WINDOW_WIDTH as f64,
    WINDOW_HEIGHT as f64,
);

fn draw_cell(display: &glium::Display, target: &mut glium::Frame, x_1: f32, y_1: f32, x_2: f32, y_2: f32) {
    // Draw a square by drawing 2 triangles
    let shape_1 = vec![
        Vertex { position: [x_1, y_1] },
        Vertex { position: [x_1, y_2] },
        Vertex { position: [x_2, y_2] },
    ];

    let shape_2 = vec![
        Vertex { position: [x_1, y_1] },
        Vertex { position: [x_2, y_1] },
        Vertex { position: [x_2, y_2] },
    ];

    let vertex_buffer_1 = glium::VertexBuffer::new(display, &shape_1).unwrap();
    let vertex_buffer_2 = glium::VertexBuffer::new(display, &shape_2).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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
    ", CELL.red, CELL.green, CELL.blue);

    let program = glium::Program::from_source(display, vertex_shader_src, &fragment_shader_src, None).unwrap();

    (*target).draw(
        &vertex_buffer_1,
        &indices,
        &program,
        &glium::uniforms::EmptyUniforms,
        &Default::default()
    ).unwrap();

    (*target).draw(
        &vertex_buffer_2,
        &indices,
        &program,
        &glium::uniforms::EmptyUniforms,
        &Default::default()
    ).unwrap();
}

fn draw_gridline(display: &glium::Display, target: &mut glium::Frame, from_x: f32, from_y: f32, to_x: f32, to_y: f32) {
    let shape = vec![
        Vertex { position: [from_x, from_y] },
        Vertex { position: [to_x, to_y] },
    ];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

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

    let program = glium::Program::from_source(display, vertex_shader_src, &fragment_shader_src, None).unwrap();

    (*target).draw( 
        &vertex_buffer,
        &indices,
        &program,
        &glium::uniforms::EmptyUniforms,
        &Default::default()
    ).unwrap();
}

impl Display {
    pub fn new(columns: i32, rows: i32) -> Display {
        Display { columns: columns, rows: rows }
    }

    pub fn draw<F: 'static>(&self, mut get_next_state_func: F)
        where F: FnMut() -> Vec<Vec<bool>>
    {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        display.gl_window().window().set_title(WINDOW_TITLE);
        display.gl_window().window().set_inner_size(WINDOW_SHAPE);

        let columns_range = (0..(self.columns as usize)).collect::<Vec<usize>>();

        let mut vertical_gridline_positions = vec![];
        for x in columns_range.iter() {
            vertical_gridline_positions.push(
                2.0 * *x as f32 / self.columns as f32 - 1.0
            );
        }
        vertical_gridline_positions.push(2.0);

        let rows_range = (0..(self.rows as usize)).collect::<Vec<usize>>();
        let mut horizontal_gridline_positions = vec![];
        for y in rows_range.iter() {
            horizontal_gridline_positions.push(
                2.0 * *y as f32 / self.rows as f32 - 1.0
            );
        }
        horizontal_gridline_positions.push(2.0);

		event_loop.run(move |ev, _, control_flow| {
			let next_frame_time = std::time::Instant::now() + std::time::Duration::from_millis(REFRESH_PERIOD_MS);
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

            // Draw background
            target.clear_color(
                BACKGROUND.red,
                BACKGROUND.green,
                BACKGROUND.blue,
                1.0,
            );

            // Draw cells
            let game_state = get_next_state_func();
            for row in &rows_range {
                for column in &columns_range {
                    if game_state[*row][*column] {
                        // Draw the cell
                        draw_cell(
                            &display,
                            &mut target,
                            vertical_gridline_positions[*column],
                            horizontal_gridline_positions[*row],
                            vertical_gridline_positions[*column + 1],
                            horizontal_gridline_positions[*row + 1],
                        );
                    }
                }
            }

            // Draw gridlines
            for x in &vertical_gridline_positions {
                draw_gridline(&display, &mut target, *x, 1.0, *x, -1.0);
            }
            for y in &horizontal_gridline_positions {
                draw_gridline(&display, &mut target, 1.0, *y, -1.0, *y);
            }
            
            target.finish().unwrap();
		});
    }
}



