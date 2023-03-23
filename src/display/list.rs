use crate::{color::size_fmt, info::File};

use super::spaces;

pub fn to_string(files: &Vec<File>) -> String {
	let out = files.iter().map(|f| line_fmt(f)).collect::<Vec<_>>();

	out.join("\n")
}

fn line_fmt(f: &File) -> String {
	let size = if !f.dir { size_fmt(f.size) } else { spaces(7) };
	let sp = if !f.dir {
		spaces(6 - (size.len() - 14))
	} else {
		"".to_string()
	};

	format!("{}{} {}", sp, size, f.name)
}
