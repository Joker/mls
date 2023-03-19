mod display;
mod info;
mod termsize;

use std::cmp::Reverse;
use std::fs;

use crate::display::grid;
use crate::info::{file_info, File};

use arguably::ArgParser;

pub const INDENT: usize = 3;

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

	let mut file_names = match fs::read_dir(dir) {
		Ok(list) => list
			.filter_map(|x| file_info(&x.unwrap().path(), parser.found("a")))
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

	println!("{}", grid(&file_names, INDENT));
}
