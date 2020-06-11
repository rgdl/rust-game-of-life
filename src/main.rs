mod display;
mod game;

fn main() {
    println!("Hello, world!");
    let display = display::Display::new(15, 8);
    display.draw(vec![vec![true]]);
}
