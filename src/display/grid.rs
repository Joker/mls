use std::usize;

use crate::{ext::unlibc::terminal_size, File};

use super::GRID_GAP;

fn columns_width(files: &Vec<File>, stack_size: usize) -> (usize, Vec<usize>) {
	let mut col_sizes = Vec::new();
	let mut count = 0;
	let mut maximum = 0;

	for f in files {
		if count == stack_size {
			col_sizes.push(maximum);
			maximum = 0;
			count = 0;
		}
		maximum = if f.len > maximum { f.len } else { maximum };
		count += 1;
	}
	if maximum > 0 {
		col_sizes.push(maximum)
	}

	(col_sizes.iter().sum::<usize>(), col_sizes)
}

fn grid_size(names: &Vec<File>) -> (usize, Vec<usize>) {
	let (term_width, _) = terminal_size().unwrap();

	let mut stack = names.len() / (term_width / names.iter().map(|f| f.len).max().unwrap_or(term_width / 2));

	if stack < 2 && names.iter().fold(0, |acc, e| acc + e.len) <= term_width {
		return (1, Vec::new());
	}

	let (mut width, mut col_sizes) = columns_width(names, stack);
	if term_width < width {
		loop {
			stack += 1;
			(width, col_sizes) = columns_width(names, stack);
			if term_width >= width {
				return (stack, col_sizes);
			}
		}
	} else {
		let mut column_out = col_sizes;
		loop {
			stack -= 1;
			(width, col_sizes) = columns_width(names, stack);
			if term_width == width {
				return (stack, col_sizes);
			}
			if term_width < width {
				return (stack + 1, column_out);
			}
			column_out = col_sizes;
		}
	}
}

pub fn print(files: &Vec<File>) {
	let (stack, column_sizes) = grid_size(files);

	if stack == 1 {
		return println!(
			"{}",
			files
				.iter()
				.map(|x| x.name.as_str())
				.collect::<Vec<_>>()
				.join(&spaces(GRID_GAP))
		);
	}

	let mut str_vec: Vec<String> = Vec::with_capacity(stack);
	str_vec.resize(stack, String::from(""));

	for (i, nm) in files.iter().enumerate() {
		let row_num = i % stack;
		let col_num = i / stack;

		str_vec[row_num].push_str(&nm.name);

		if column_sizes[col_num] >= files[i].len {
			str_vec[row_num].push_str(&spaces(column_sizes[col_num] - files[i].len + GRID_GAP));
		}
	}

	println!("{}", str_vec.join("\n"))
}

fn spaces(width: usize) -> String {
	(0..width).map(|_| ' ').collect()
}
