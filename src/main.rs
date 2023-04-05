#![allow(non_snake_case)]
mod color;
mod datetime;
mod display;
mod fileinfo;
mod info;
mod unsafelibc;
mod xattr;

use std::{cmp::Reverse, env, fs, path::Path};

use crate::color::{BLUE_L, RED, WHITE};
use crate::fileinfo::basepath;

use arguably::ArgParser;
use display::GRID_GAP;
use info::{file_info, File};

#[derive(Debug)]
pub struct Flags {
	pub all: bool,
	pub long: bool,
	pub Size_sort: bool,
	pub time_sort: bool,
	pub full: bool,
	pub bytes: bool,
	pub ctime: bool,
	pub u_access: bool,
	pub U_create: bool,
	pub dir_only: bool,
	pub group: bool,
	pub tree2: bool,
	pub tree3: bool,
	pub tree0: bool,
}

#[derive(Debug)]
pub struct Width {
	pub uid: usize,
	pub gid: usize,
	pub szn: usize,
	pub xattr: bool,
}

fn args_init() -> (Flags, Vec<String>) {
	let args: Vec<String> = env::args().collect();

	let mut parser = ArgParser::new()
		.helptext(
			r#"USAGE:
	ls [-alStfbcuUgd] [file ...]
OPTIONS:
	-a   Include directory entries whose names begin with a dot (`.`).
	-l   List files in the long format.
	-S   Sort by size.
	-t   Sort by time.
	-f   Absolute path for symbolic link in the list.
	-b   List file sizes in bytes.
	-c   Use time when file status was last changed.
	-u   Use time of last access, instead of time of last modification of the file.
	-U   Use time when file was created.
	-g   Display the group name.
	-d   List of directories only.
	"#,
			// -2   Recurse into directories as a tree. Limit the depth 2.
			// -3   Recurse into directories as a tree. Limit the depth 3.
			// -0   Recurse into directories as a tree.
			// -L   Limit the depth of recursion.
		)
		.flag("a")
		.flag("l")
		.flag("S")
		.flag("t")
		.flag("f")
		.flag("h")
		.flag("b")
		.flag("c")
		.flag("u")
		.flag("U")
		.flag("g")
		.flag("d")
		.flag("2")
		.flag("3")
		.flag("0");

	if let Err(err) = parser.parse() {
		err.exit();
	}
	let mut fl = Flags {
		all: parser.found("a"),
		long: parser.found("l"),
		Size_sort: parser.found("S"),
		time_sort: parser.found("t"),
		full: parser.found("f"),
		bytes: parser.found("b"),
		ctime: parser.found("c"),
		u_access: parser.found("u"),
		U_create: parser.found("U"),
		dir_only: parser.found("d"),
		group: parser.found("g"),
		tree2: parser.found("2"),
		tree3: parser.found("3"),
		tree0: parser.found("0"),
	};
	if parser.found("h") {
		fl.bytes = true
	}
	match args[0].rsplit("/").next() {
		Some(p) => match p {
			"la" => fl.all = true,
			"lla" | "lal" => {
				fl.long = true;
				fl.all = true;
			}
			"ll" => fl.long = true,
			"lt" => fl.tree2 = true,
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

	let mut f_width = Width {
		uid: 0,
		gid: 0,
		szn: 0,
		xattr: false,
	};

	for st in args {
		match Path::new(&st) {
			path if path.is_file() => match file_info(&path.to_path_buf(), &fl, &mut f_width) {
				Some(mut f) => {
					f.name = format!("{WHITE}{}/{}", basepath(path), f.name);
					f.len = format!("{}/{}", basepath(path), f.sname).chars().count() + GRID_GAP;
					standalone.push(f)
				}
				None => (),
			},
			path if path.is_dir() => {
				let file_list = match fs::read_dir(path) {
					Ok(list) => list
						.filter_map(|x| file_info(&x.unwrap().path(), &fl, &mut f_width))
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
		file_vec_print(None, standalone, &fl, &f_width)
	}

	if folders.len() == 1 && sl == 0 {
		folders[0].0 = None;
	}

	for (title, folder) in folders {
		file_vec_print(title, folder, &fl, &f_width)
	}
}

fn file_vec_print(title: Option<String>, mut file_list: Vec<File>, fl: &Flags, w: &Width) {
	if let Some(pt) = title {
		println!("\n{WHITE}{pt}:")
	}

	match file_list.len() {
		0 => return println!("{BLUE_L}.   .."),
		f if f > 1 => {
			if fl.Size_sort {
				file_list.sort_by_key(|f| (Reverse(f.dir), f.long.as_ref().unwrap().size));
				return display::list::print(&file_list, fl, w);
			}

			if fl.time_sort {
				file_list.sort_by_key(|f| (f.long.as_ref().unwrap().time));
				return display::list::print(&file_list, fl, w);
			}

			file_list.sort_by_key(|f| (Reverse(f.dir), f.ext.clone(), f.sname.clone()));
		}
		_ => (),
	}

	if fl.long || fl.group {
		return display::list::print(&file_list, fl, w);
	}
	display::grid::print(&file_list);
}
