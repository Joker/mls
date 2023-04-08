pub mod grid;
pub mod list;
pub mod tree;

pub const GRID_GAP: usize = 3;

pub fn spaces(width: usize) -> String {
	(0..width).into_iter().map(|_| ' ').collect()
}
