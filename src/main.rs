use std::cmp::Reverse;
use std::fs;
use std::path::{Path, PathBuf};

use crate::display::grid;

use arguably::ArgParser;

mod display;
mod termsize;

fn filename(path: &Path) -> String {
	match path.file_name() {
		Some(name) => name.to_string_lossy().to_string(),
		_ => path.display().to_string(),
	}
}
fn ext(path: &Path) -> String {
	match path.extension() {
		Some(ext) => ext.to_string_lossy().to_string(),
		_ => "".to_string(),
	}
}

#[derive(Clone, Debug)]
pub struct File {
	pub path: PathBuf,
	pub name: String,
	pub ext: String,
	pub dir: bool,
	pub dot: bool,
}

fn main() {
	let mut parser = ArgParser::new()
		.helptext("Usage: mls")
		.version("1.0")
		.flag("a")
		.flag("l");
	if let Err(err) = parser.parse() {
		err.exit();
	}
	if parser.found("l") {
		println!("Flag -l found.");
	}
	let dir = if parser.args.len() > 0 {
		parser.args[0].clone()
	} else {
		".".to_string()
	};

	//

	let hide = if parser.found("a") { true } else { false };

	let mut file_names = match fs::read_dir(dir) {
		Ok(list) => list
			.filter_map(|x| {
				let path = &x.unwrap().path();
				let fname = filename(path);
				let dot = fname.chars().next().unwrap() == '.';
				let md = std::fs::symlink_metadata(path).unwrap();

				if dot && hide || !dot {
					return Some(File {
						path: path.to_path_buf(),
						name: fname.clone(),
						ext: ext(path),
						dir: md.is_dir(),
						dot: dot,
					});
				} else {
					return None;
				}
			})
			.collect::<Vec<File>>(),

		Err(e) => {
			println!("{}", e);
			return;
		}
	};
	if file_names.len() == 0 {
		println!(".   ..");
		return;
	}

	file_names.sort_by_key(|f| (Reverse(f.dir), f.ext.clone(), f.name.clone()));
	println!("{}", grid(&file_names, 3));
}
