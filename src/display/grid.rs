use std::usize;

use super::{spaces, INDENT};
use crate::{termsize::terminal_size, File};

fn width_sizes(names: &Vec<File>, stack_size: &usize) -> (usize, Vec<usize>) {
	let mut col_sizes = Vec::new();
	let mut count = 0;
	let mut maximum = 0;

	for n in names {
		if count == *stack_size {
			col_sizes.push(maximum);
			maximum = 0;
			count = 0;
		}
		maximum = if n.len > maximum { n.len } else { maximum };
		count += 1;
	}
	if maximum > 0 {
		col_sizes.push(maximum)
	}

	(col_sizes.iter().sum::<usize>(), col_sizes)
}

fn grid_size(names: &Vec<File>) -> (usize, Vec<usize>) {
	let (mw, _) = terminal_size().unwrap();
	let term_width = mw as usize;

	let mut stack =
		names.len() / (term_width / names.iter().map(|f| f.len).max().unwrap_or(term_width / 2));

	let (mut width, mut col_sizes) = width_sizes(names, &stack);
	if term_width < width {
		loop {
			stack += 1;
			(width, col_sizes) = width_sizes(names, &stack);
			if term_width >= width {
				return (stack, col_sizes);
			}
		}
	} else {
		let mut column_out = col_sizes;
		loop {
			stack -= 1;
			(width, col_sizes) = width_sizes(names, &stack);
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

pub fn to_string(files: &Vec<File>) -> String {
	let (stack, column_sizes) = grid_size(files);

	if stack == 1 {
		return files
			.iter()
			.map(|x| x.name.clone())
			.collect::<Vec<String>>()
			.join(&spaces(INDENT));
	}

	let mut str_vec: Vec<String> = Vec::with_capacity(stack);
	str_vec.resize(stack, String::from(""));

	for (i, nm) in files.iter().enumerate() {
		let row_num = i % stack;
		let col_num = i / stack;

		str_vec[row_num].push_str(&nm.name);

		if column_sizes[col_num] >= files[i].len {
			str_vec[row_num].push_str(&spaces(column_sizes[col_num] - files[i].len + INDENT));
		}
	}

	str_vec.join("\n")
}
