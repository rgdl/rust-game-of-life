# First toy Rust project. Create Conway's Game of Life

This will be an application in a window that plays out the game's rules from a randomised initial state.

Display class:
- creates an OS window on initialisation
- draws a grid
- fills in some grid squares, according to a 2D array passed to it by game state
- has a restart button (randomise game state)

Game class:
- maintains a 2D array of alive/dead cells (doesn't actually need to be 2D in practice)
- applies the rules at each time step
- has a clock of some kind

I want to write unit tests, to get the hang of `cargo test`. NOTE these should be inside modules. The separate tests directory is for integration tests
Write both unit tests (inside modules) and integration tests (tests directory), even if this is overkill
