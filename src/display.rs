pub mod grid;
pub mod list;

pub const INDENT: usize = 3;

pub fn spaces(width: usize) -> String {
	(0..width).into_iter().map(|_| ' ').collect()
}
