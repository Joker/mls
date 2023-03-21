use std::usize;

use super::{spaces, INDENT};
use crate::{termsize::terminal_size, File};

fn width_sizes(names: &Vec<usize>, stack_size: &usize) -> (usize, Vec<usize>) {
	let mut out = Vec::new();
	let mut count = 0;
	let mut maximum = 0;

	for n in names {
		if count == *stack_size {
			out.push(maximum);
			maximum = 0;
			count = 0;
		}
		maximum = if *n > maximum { *n } else { maximum };
		count += 1;
	}
	if maximum > 0 {
		out.push(maximum)
	}

	(out.iter().sum::<usize>(), out)
}

fn grid_size(names: &Vec<usize>) -> (usize, Vec<usize>) {
	let (mw, _) = terminal_size().unwrap();
	let term_width = mw as usize;

	let mut stack = names.len() / (term_width / names.iter().max().unwrap());
	let mut column_vec = Vec::new();

	loop {
		let (sum, row_sizes) = width_sizes(names, &stack);

		if term_width >= sum && stack != 0 {
			column_vec = row_sizes;
			stack -= 1;
		} else {
			return (stack + 1, column_vec);
		}
	}
}

pub fn to_string(files: &Vec<File>) -> String {
	let mut name_len = Vec::new();
	for file in files {
		name_len.push(file.name.chars().count() + INDENT);
	}

	let (stack, column_sizes) = grid_size(&name_len);

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
		let j = i % stack;
		let k = i / stack;

		str_vec[j].push_str(&nm.name);

		if column_sizes[k] >= name_len[i] {
			str_vec[j].push_str(&spaces(column_sizes[k] - name_len[i] + INDENT));
		}
	}

	str_vec.join("\n")
}
