pub const ROWS: i32 = 100;
pub const COLS: i32 = 100;
pub const SCREEN_HEIGHT: i32 = 1000;
pub const SCREEN_WIDTH: i32 = 1000;
pub const CELL_WIDTH: i32 = SCREEN_WIDTH / COLS;
pub const CELL_HEIGHT: i32 = SCREEN_HEIGHT / ROWS;

// for terminal ui
pub const ALIVE_SYM: &str = "•";
pub const DEAD_SYM: &str = ".";

// raylib ui colors
pub const GRID_COLOR: &str = "212120";
pub const ALIVE_COLOR: &str = "2ff768";
pub const DEAD_COLOR: &str = "000000";
