use std::{os::unix::prelude::PermissionsExt, path::PathBuf};

use crate::color::{CYAN, RED, WHITE};

use super::name::{ext, ext_group, filename, filename_fmt, parent_path};
use super::USER_EXE;

pub fn info(pb: &PathBuf) -> (PathBuf, PathBuf, bool, bool, bool) {
	let mut read_link_path = PathBuf::new();
	let mut full_path = PathBuf::new();
	let mut error = false;
	let mut dir = false;
	let mut exe = false;
	match std::fs::read_link(pb) {
		Ok(p) => {
			full_path = if p.is_relative() {
				PathBuf::from(format!("{}/{}", parent_path(pb), p.to_string_lossy()))
			} else {
				p.clone()
			};
			read_link_path = p;
			match std::fs::metadata(&full_path) {
				Ok(metadata) => {
					dir = metadata.is_dir();
					exe = metadata.permissions().mode() & USER_EXE == USER_EXE;
				}
				Err(_) => error = true,
			}
		}
		Err(_) => error = true,
	};
	(read_link_path, full_path, exe, dir, error)
}

pub fn ref_fmt(pb: &PathBuf, abs: bool) -> (String, bool) {
	let (path, pb_path, exe, dir, error) = info(pb);

	if error {
		return (format!("{RED} -> {}", path.to_string_lossy()), false);
	}

	let (ext, egrp) = ext_group(ext(&path));
	let name = filename(&path);

	let path_to = if abs {
		match std::fs::canonicalize(pb_path) {
			Ok(s) => parent_path(s.as_path()),
			Err(_) => parent_path(path.as_path()),
		}
	} else {
		parent_path(path.as_path())
	};
	(
		format!(
			"{WHITE} -> {CYAN}{path_to}{}",
			filename_fmt(&name, &ext, egrp, dir, exe, false)
		),
		dir,
	)
}
