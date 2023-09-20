use std::usize;

use crate::{color::RESET, ext::unlibc::terminal_size, File};

use super::GRID_GAP;

fn columns_width(files: &Vec<File>, lines: usize) -> (usize, Vec<usize>) {
	let mut col_sizes = Vec::new();
	let mut count = 0;
	let mut maximum = 0;

	for f in files {
		if count == lines {
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

	let mut lines = names.len() / (term_width / names.iter().map(|f| f.len).max().unwrap_or(term_width / 2));

	if lines < 3 && names.iter().fold(0, |acc, fname| acc + fname.len) <= term_width {
		return (1, Vec::new());
	}

	let (mut col_sum, mut column_vec) = columns_width(names, lines);
	if term_width < col_sum {
		loop {
			lines += 1;
			(col_sum, column_vec) = columns_width(names, lines);
			if term_width >= col_sum {
				return (lines, column_vec);
			}
		}
	}
	if term_width > col_sum && lines > 2 {
		let mut column_bcup = column_vec;
		loop {
			lines -= 1;
			if lines < 2 {
				return (2, column_bcup);
			}
			(col_sum, column_vec) = columns_width(names, lines);

			if term_width == col_sum {
				return (lines, column_vec);
			}
			if term_width < col_sum {
				return (lines + 1, column_bcup);
			}
			column_bcup = column_vec;
		}
	}
	return (lines, column_vec);
}

pub fn print(files: &Vec<File>) {
	let (lines, column_sizes) = grid_size(files);

	if lines == 1 {
		return println!(
			"{}{RESET}",
			files
				.iter()
				.map(|x| x.name.as_str())
				.collect::<Vec<_>>()
				.join(&spaces(GRID_GAP))
		);
	}

	let mut str_vec: Vec<String> = Vec::with_capacity(lines);
	str_vec.resize(lines, String::from(""));

	for (i, nm) in files.iter().enumerate() {
		let row_num = i % lines;
		let col_num = i / lines;

		str_vec[row_num].push_str(&nm.name);

		if column_sizes[col_num] >= files[i].len {
			str_vec[row_num].push_str(&spaces(column_sizes[col_num] - files[i].len + GRID_GAP));
		}
	}

	println!("{}{RESET}", str_vec.join("\n"))
}

fn spaces(width: usize) -> String {
	(0..width).map(|_| ' ').collect()
}
