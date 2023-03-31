#![allow(non_snake_case)]
mod color;
mod datetime;
mod display;
mod info;
mod unsafelibc;

use std::{cmp::Reverse, env, fs, path::Path};

use crate::color::{BLUE_L, RED, WHITE};
use crate::info::{basepath, file_info, File};

use arguably::ArgParser;

pub struct Flags {
	pub all: bool,
	pub long: bool,
	pub Size_sort: bool,
	pub time_sort: bool,
	pub full: bool,
	pub human: bool,
	pub ctime: bool,
	pub u_access: bool,
	pub U_create: bool,
	pub dir_only: bool,
}

fn args_init() -> (Flags, Vec<String>) {
	let args: Vec<String> = env::args().collect();

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
		.flag("U")
		.flag("d");
	if let Err(err) = parser.parse() {
		err.exit();
	}
	let mut fl = Flags {
		all: parser.found("a"),
		long: parser.found("l"),
		Size_sort: parser.found("S"),
		time_sort: parser.found("t"),
		full: parser.found("f"),
		human: parser.found("h"),
		ctime: parser.found("c"),
		u_access: parser.found("u"),
		U_create: parser.found("U"),
		dir_only: parser.found("d"),
	};
	match args[0].rsplit("/").next() {
		Some(p) => match p {
			"la" => fl.all = true,
			"lla" | "lal" => {
				fl.long = true;
				fl.all = true;
			}
			"ll" => fl.long = true,
			"lsd" => fl.dir_only = true,
			_ => (),
		},
		None => (),
	};

	let dirs = if parser.args.len() > 0 {
		parser.args
	} else {
		vec![".".to_string()]
	};
	(fl, dirs)
}

fn main() {
	let (fl, args) = args_init();

	let mut standalone = Vec::new();
	let mut folders = Vec::new();

	let mut standalone_width: usize = 0;
	let mut name_max_width: usize = 0;

	for st in args {
		match Path::new(&st) {
			path if path.is_file() => {
				match file_info(&path.to_path_buf(), &fl, &mut standalone_width) {
					Some(mut f) => {
						f.name = format!("{WHITE}{}/{}", basepath(path), f.name);
						standalone.push(f)
					}
					None => (),
				}
			}
			path if path.is_dir() => {
				let file_list = match fs::read_dir(path) {
					Ok(list) => list
						.filter_map(|x| file_info(&x.unwrap().path(), &fl, &mut name_max_width))
						.collect::<Vec<File>>(),
					Err(e) => {
						return println!("{}", e);
					}
				};
				folders.push((Some(st), file_list));
			}
			_ => println!("{RED}{st}{WHITE}: No such file or directory\n"),
		}
	}

	let sl = standalone.len();
	if sl > 0 {
		file_vec_print(None, standalone, &fl, name_max_width)
	}

	if folders.len() == 1 && sl == 0 {
		folders[0].0 = None;
	}

	for (title, folder) in folders {
		file_vec_print(title, folder, &fl, name_max_width)
	}
}

fn file_vec_print(title: Option<String>, mut file_list: Vec<File>, fl: &Flags, width: usize) {
	if let Some(pt) = title {
		println!("\n{WHITE}{pt}:")
	}
	if file_list.len() == 0 {
		return println!("{BLUE_L}.   ..");
	}

	if fl.Size_sort {
		file_list.sort_by_key(|f| (Reverse(f.dir), f.size));
	} else if fl.time_sort {
		file_list.sort_by_key(|f| (f.time));
	} else {
		file_list.sort_by_key(|f| (Reverse(f.dir), f.ext.clone(), f.sname.clone()));
	}

	if fl.long {
		return display::list::print(&file_list, fl.human, width);
	}
	display::grid::print(&file_list);
}
