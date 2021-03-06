use glium::{glutin, Surface};

pub struct Display { columns: i32, rows: i32, refresh_period_ms: u64 }

struct Colour {
    red: f32,
    green: f32,
    blue: f32,
}

struct Square {
    x_1: f32,
    y_1: f32,
    x_2: f32,
    y_2: f32,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

glium::implement_vertex!(Vertex, position);

static BACKGROUND: Colour = Colour { red: 0.1, green: 0.0, blue: 0.6 };
static CELL: Colour = Colour { red: 0.0, green: 0.7, blue: 0.0 };
static GRIDLINES: Colour = Colour { red: 0.7, green: 0.0, blue: 0.0 };

static WINDOW_TITLE: &str = "Game of Life";
static WINDOW_HEIGHT: i32 = 500;
static WINDOW_WIDTH: i32 = 800;
static WINDOW_SHAPE: glutin::dpi::LogicalSize<f64> = glutin::dpi::LogicalSize::new(
    WINDOW_WIDTH as f64,
    WINDOW_HEIGHT as f64,
);

// Constants for OpenGL calls
static VERTEX_SHADER_SRC: &str = "
    #version 140
    in vec2 position;
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
";

fn draw_all_cells(display: &glium::Display, target: &mut glium::Frame, squares: Vec<Square>) {
    let mut vertices = vec![];
    for square in squares {
        vertices.push(Vertex { position: [square.x_1, square.y_1] });
        vertices.push(Vertex { position: [square.x_1, square.y_2] });
        vertices.push(Vertex { position: [square.x_2, square.y_2] });

        vertices.push(Vertex { position: [square.x_1, square.y_1] });
        vertices.push(Vertex { position: [square.x_2, square.y_1] });
        vertices.push(Vertex { position: [square.x_2, square.y_2] });
    }

    let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let fragment_shader_src = format!("
        #version 140
        out vec4 color;
        void main() {{
            color = vec4({}, {}, {}, 1.0);
        }}
    ", CELL.red, CELL.green, CELL.blue);

    let program = glium::Program::from_source(display, VERTEX_SHADER_SRC, &fragment_shader_src, None).unwrap();

    (*target).draw(
        &vertex_buffer,
        &indices,
        &program,
        &glium::uniforms::EmptyUniforms,
        &Default::default()
    ).unwrap();
}

fn draw_all_gridlines(display: &glium::Display, target: &mut glium::Frame, all_x: &Vec<f32>, all_y: &Vec<f32>) {
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

    let fragment_shader_src = format!("
        #version 140
        out vec4 color;
        void main() {{
            color = vec4({}, {}, {}, 1.0);
        }}
    ", GRIDLINES.red, GRIDLINES.green, GRIDLINES.blue);

    let program = glium::Program::from_source(display, VERTEX_SHADER_SRC, &fragment_shader_src, None).unwrap();

    let mut vertices = vec![];
    for y in all_y {
        vertices.push(Vertex { position: [-1.0, *y] });
        vertices.push(Vertex { position: [1.0, *y] });
    }
    for x in all_x {
        vertices.push(Vertex { position: [*x, -1.0] });
        vertices.push(Vertex { position: [*x, 1.0] });
    }
    let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
    (*target).draw( 
        &vertex_buffer,
        &indices,
        &program,
        &glium::uniforms::EmptyUniforms,
        &Default::default()
    ).unwrap();
}

impl Display {
    pub fn new(columns: i32, rows: i32, refresh_period_ms: u64) -> Display {
        Display { columns: columns, rows: rows, refresh_period_ms: refresh_period_ms }
    }

    pub fn run<F: 'static>(&self, mut get_next_state_func: F)
        where F: FnMut() -> Vec<Vec<bool>>
    {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new().with_inner_size::<glutin::dpi::LogicalSize<f64>>(WINDOW_SHAPE.into());
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        display.gl_window().window().set_title(WINDOW_TITLE);

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
		let refresh_period_ms = self.refresh_period_ms; 
		event_loop.run(move |ev, _, control_flow| {
            std::thread::sleep(std::time::Duration::from_millis(refresh_period_ms));
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
            let mut squares = vec![];
            for row in &rows_range {
                for column in &columns_range {
                    if game_state[*row][*column] {
                        squares.push(Square {
                            x_1: vertical_gridline_positions[*column],
                            y_1: horizontal_gridline_positions[*row],
                            x_2: vertical_gridline_positions[*column + 1],
                            y_2: horizontal_gridline_positions[*row + 1],
                        });
                    }
                }
            }
            draw_all_cells(&display, &mut target, squares);

            // Draw gridlines
            draw_all_gridlines(
                &display,
                &mut target,
                &vertical_gridline_positions,
                &horizontal_gridline_positions,
            );
 
            target.finish().unwrap();
		});
    }
}

