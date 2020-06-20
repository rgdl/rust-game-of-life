mod display;
mod game;

static COLUMNS: i32 = 130;
static ROWS: i32 = 80; 
static INITIAL_PROPORTION: f32 = 0.2;
static REFRESH_PERIOD_MS: u64 = 100;


fn main() {
    let mut game = game::Game::new(COLUMNS, ROWS, INITIAL_PROPORTION);
    let display = display::Display::new(COLUMNS, ROWS, REFRESH_PERIOD_MS);
    display.run(move || game.get_next_state());
}
