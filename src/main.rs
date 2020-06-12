mod display;
mod game;

static COLUMNS: i32 = 15;
static ROWS: i32 = 8; 

fn main() {
    println!("Hello, world!");
    let game = game::Game::new(COLUMNS, ROWS);
    let display = display::Display::new(COLUMNS, ROWS);
    display.draw(vec![vec![true]]);
}
