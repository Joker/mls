#![allow(non_snake_case)]
mod color;
mod datetime;
mod display;
mod info;
mod unsafelibc;

use std::cmp::Reverse;
use std::fs;

use crate::info::{file_info, File};

use arguably::ArgParser;

pub struct Flag {
	pub all: bool,
	pub long: bool,
	pub Size_sort: bool,
	pub time_sort: bool,
	pub full: bool,
	pub human: bool,
	pub ctime: bool,
	pub u_access: bool,
	pub U_create: bool,
}
fn main() {
	let mut parser = ArgParser::new()
		.helptext("Usage: mls")
		.version("1.0")
		.flag("a")
		.flag("l")
		.flag("S")
		.flag("t")
		.flag("f")
		.flag("h")
		.flag("c")
		.flag("u")
		.flag("U");
	if let Err(err) = parser.parse() {
		err.exit();
	}
	let fl = Flag {
		all: parser.found("a"),
		long: parser.found("l"),
		Size_sort: parser.found("S"),
		time_sort: parser.found("t"),
		full: parser.found("f"),
		human: parser.found("h"),
		ctime: parser.found("c"),
		u_access: parser.found("u"),
		U_create: parser.found("U"),
	};
	let dir = if parser.args.len() > 0 {
		parser.args[0].as_str()
	} else {
		"."
	};

	//

	let mut name_max_width: usize = 0;
	let mut file_list = match fs::read_dir(dir) {
		Ok(list) => list
			.filter_map(|x| file_info(&x.unwrap().path(), &fl, &mut name_max_width))
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
	if fl.Size_sort {
		file_list.sort_by_key(|f| (Reverse(f.dir), f.size));
	} else if fl.time_sort {
		file_list.sort_by_key(|f| (f.time));
	} else {
		file_list.sort_by_key(|f| (Reverse(f.dir), f.ext.clone(), f.sname.clone()));
	}

	//

	if fl.long {
		println!(
			"{}",
			display::list::to_string(&file_list, fl.human, name_max_width)
		);
		return;
	}
	println!("{}", display::grid::to_string(&file_list));
}
