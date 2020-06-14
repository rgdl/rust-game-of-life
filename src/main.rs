mod display;
mod game;

static COLUMNS: i32 = 80;
static ROWS: i32 = 50; 

fn main() {
    let mut game = game::Game::new(COLUMNS, ROWS);
    let display = display::Display::new(COLUMNS, ROWS);
    display.run(move || game.get_next_state());
}
