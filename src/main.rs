mod display;
mod game;

static COLUMNS: i32 = 20;
static ROWS: i32 = 13; 

fn main() {
    let mut game = game::Game::new(COLUMNS, ROWS);
    let display = display::Display::new(COLUMNS, ROWS);
    display.draw(move || game.get_next_state());
}
