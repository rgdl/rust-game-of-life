pub struct Display {
    columns: i32,
    rows: i32,
}

impl Display {
    pub fn new(columns: i32, rows: i32) -> Display {
        Display { columns: columns, rows: rows }
    }

    pub fn draw(&self, state: Vec<Vec<bool>>) {
        // use this as a guide: https://github.com/glium/glium/blob/master/book/tuto-01-getting-started.md
        
        use glium::glutin;

        let mut event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

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
		});
    }
}



