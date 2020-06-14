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

Don't worry about unit tests. I think I've learned what I need to learn about that.


BUGS:
- awkward resize on startup. Can we start fullscreen?
