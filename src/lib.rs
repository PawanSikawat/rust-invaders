pub mod frame;
pub mod invaders;
pub mod player;
pub mod render;
pub mod shot;
pub mod utils;

pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 40;

pub const SHOT_MOVE_SPEED: usize = 50;
pub const SHOT_EXPLOSION_TIME: usize = 250;
pub const SHOT_MAX_COUNT: usize = 2;

pub const INVADER_ROW_COUNT: usize = 9;
pub const INVADER_MOVE_SPEED: usize = 2000;
pub const INVADER_MIN_SPEED: usize = 250;
