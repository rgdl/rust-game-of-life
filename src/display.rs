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
        println!("Hi bitch");
    }
}
