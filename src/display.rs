pub mod grid;
pub mod list;

pub const SIZE_WIDTH: usize = 5;
pub const GRID_GAP: usize = 3;
pub const TIMEZONE: u64 = 10800;

pub fn spaces(width: usize) -> String {
	(0..width).into_iter().map(|_| ' ').collect()
}
