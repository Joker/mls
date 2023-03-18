use std::usize;

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

fn size(names: &Vec<usize>) -> (usize, Vec<usize>) {
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

fn spaces(width: usize) -> String {
	(0..width).into_iter().map(|_| ' ').collect()
}

pub fn grid(files: &Vec<File>, indent: usize) -> String {
	let mut name_len = Vec::new();
	for file in files {
		name_len.push(file.name.len() + indent);
	}

	let (stack, column_sizes) = size(&name_len);

	let mut str_vec: Vec<String> = Vec::new();
	for _ in 0..stack {
		str_vec.push("".to_string());
	}

	for (i, nm) in files.iter().enumerate() {
		let j = i % stack;
		let k = i / stack;

		str_vec[j] += &nm.name;

		if column_sizes[k] >= name_len[i] {
			str_vec[j] += &spaces(column_sizes[k] - name_len[i] + indent);
		}
	}

	str_vec.join("\n")
}
