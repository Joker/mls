mod display;
mod info;
mod termsize;

use std::cmp::Reverse;
use std::fs;

use crate::info::{file_info, File};

use arguably::ArgParser;

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
	let a = parser.found("a");

	//

	let mut file_list = match fs::read_dir(dir) {
		Ok(list) => list
			.filter_map(|x| file_info(&x.unwrap().path(), a))
			.collect::<Vec<File>>(),
		Err(e) => {
			println!("{}", e);
			return;
		}
	};
	if file_list.len() == 0 {
		println!(".   ..");
		return;
	}
	file_list.sort_by_key(|f| (Reverse(f.dir), f.ext.clone(), f.name.clone()));

	println!("{}", display::grid::to_string(&file_list)); 
}
