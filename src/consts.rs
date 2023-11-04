pub const ROWS: i32 = SCREEN_HEIGHT / CELL_HEIGHT;
pub const COLS: i32 = SCREEN_WIDTH / CELL_WIDTH;
pub const SCREEN_HEIGHT: i32 = 900;
pub const SCREEN_WIDTH: i32 = 1500;
pub const CELL_WIDTH: i32 = 5;
pub const CELL_HEIGHT: i32 = 5;

// time speed
pub const REFRESH_MILLIS: u64 = 50;

// for terminal ui
pub const ALIVE_SYM: &str = "•";
pub const DEAD_SYM: &str = ".";

// raylib ui colors
pub const GRID_COLOR: &str = "212120";
pub const ALIVE_COLOR: &str = "2ff768";
pub const DEAD_COLOR: &str = "000000";
