mod color;
mod display;
mod info;
mod unsafelibc;
mod datetime;

use std::cmp::Reverse;
use std::fs;

use crate::info::{file_info, File};

use arguably::ArgParser;

fn main() {
	let mut parser = ArgParser::new()
		.helptext("Usage: mls")
		.version("1.0")
		.flag("a")
		.flag("l")
		.flag("S")
		.flag("f")
		.flag("h");
	if let Err(err) = parser.parse() {
		err.exit();
	}
	let dir = if parser.args.len() > 0 {
		parser.args[0].as_str()
	} else {
		"."
	};
	let l = parser.found("l");
	let a = parser.found("a");
	let h = parser.found("h");
	let f = parser.found("f");

	//

	let mut file_list = match fs::read_dir(dir) {
		Ok(list) => list
			.filter_map(|x| file_info(&x.unwrap().path(), a, l, f))
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
	if parser.found("S") {
		file_list.sort_by_key(|f| (f.size, Reverse(f.dir)));
	} else {
		file_list.sort_by_key(|f| (Reverse(f.dir), f.ext.clone(), f.sname.clone()));
	}

	//

	if l {
		println!("{}", display::list::to_string(&file_list, h));
		return;
	}
	println!("{}", display::grid::to_string(&file_list));
}
