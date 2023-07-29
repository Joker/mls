mod args;
mod color;
mod display;
mod ext;
mod file;

use std::{cmp::Reverse, path::Path};

use args::{args_init, Flags};
use color::{BLUE_L, RED, WHITE};
use display::{tree, GRID_GAP};
use file::{name::parent_path, File};

#[derive(Debug)]
pub struct Width {
	pub uid: usize,
	pub gid: usize,
	pub szn: usize,
	pub xattr: bool,
}

fn main() {
	let (flags, args) = args_init();

	let mut file_list = Vec::new();
	let mut folders = Vec::new();
	let mut width = Width {
		uid: 0,
		gid: 0,
		szn: 0,
		xattr: false,
	};

	for string_path in args {
		match Path::new(&string_path) {
			path if path.is_dir() => {
				let list = if flags.tree_format {
					tree::list(path, &flags, &mut width, 0, String::new())
				} else {
					file::list(path, &flags, &mut width)
				};
				folders.push((Some(string_path), list));
			}
			path if path.exists() || path.is_symlink() => {
				if let Some(mut f) = file::info(&path.to_path_buf(), &flags, &mut width) {
					let base = parent_path(path);
					f.name = format!("{WHITE}{}{}", base, f.name);
					f.len = format!("{}{}", base, f.sname).chars().count() + GRID_GAP;
					file_list.push(f)
				}
			}
			_ => println!("{RED}{string_path}{WHITE}: No such file or directory\n"),
		}
	}

	let fl_len = file_list.len();
	if fl_len > 0 {
		file_vec_print(None, file_list, &flags, &width)
	}

	if folders.len() == 1 && fl_len == 0 {
		folders[0].0 = None; // hide the title for a single folder
	}

	for (title, folder) in folders {
		match flags.tree_format {
			true => file_vec_print(None, folder, &flags, &width),
			false => file_vec_print(title, folder, &flags, &width),
		}
	}
}

fn file_vec_print(title: Option<String>, mut file_list: Vec<File>, fl: &Flags, w: &Width) {
	if let Some(title_string) = title {
		println!("\n{WHITE}{title_string}:")
	}

	let fl_len = file_list.len();
	match fl_len {
		0 if !fl.list_format && !fl.tree_format => return println!("{BLUE_L}.   .."),
		0 => return,
		_ => (),
	}

	if !fl.tree_format && fl_len > 1 {
		if fl.size_sort {
			file_list.sort_by_key(|f| (Reverse(f.dir), f.line.as_ref().unwrap().size));
			return display::list::print(&file_list, fl, w);
		}
		if fl.time_sort {
			file_list.sort_by_key(|f| (f.line.as_ref().unwrap().time));
			return display::list::print(&file_list, fl, w);
		}
		if fl.name_sort {
			file_list.sort_by_key(|f| (f.sname.clone()));
		} else {
			file_list.sort_by_key(|f| (Reverse(f.dir), f.ext.clone(), f.sname.clone()));
		}
	}

	if fl.list_format {
		return display::list::print(&file_list, fl, w);
	}
	if fl.tree_format {
		return display::tree::print(&file_list);
	}
	display::grid::print(&file_list);
}
