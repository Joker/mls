use std::path::Path;

use crate::color::{AVI, BLUE_L, CFG, CYAN, DOC, GREEN_H, IMG, MP3, SRC, TMP, WHITE, ZIP};

pub fn parent_path(path: &Path) -> String {
	match path.parent() {
		Some(p) => {
			let mut path = p.to_string_lossy().to_string();
			if !path.is_empty() {
				path += "/"
			}
			path
		}
		_ => String::new(),
	}
}

pub fn ext(path: &Path) -> String {
	match path.extension() {
		Some(ext) => ext.to_string_lossy().to_lowercase(),
		_ => String::new(),
	}
}

pub fn ext_group(ext: String) -> (String, u8) {
	match ext.as_str() {
		"png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "ico" | "bmp" | "tiff" | "fig" | "psd"
		| "sketch" => (format!("1_{ext}"), 1),

		"7z" | "zip" | "xz" | "gz" | "rar" | "bz2" | "tar" | "tgz" | "tbz" | "txz" => (format!("2_{ext}"), 2),

		"djvu" | "doc" | "docx" | "dotx" | "odp" | "odt" | "pdf" | "ppt" | "pptx" | "rtf" | "xls"
		| "xlsx" | "fb2" => (format!("3_{ext}"), 3),

		"html" | "css" | "scss" | "sass" | "js" | "jsx" | "ts" | "tsx" | "go" | "rs" | "java" | "sql"
		| "py" | "c" | "cpp" | "hs" | "dart" | "mjs" | "cjs" | "sh" => (format!("4a_{ext}"), 7),

		"json" | "txt" | "md" | "csv" | "yaml" | "toml" | "cfg" | "conf" | "ini" | "xml" | "reg" => {
			(format!("4b_{ext}"), 4)
		}
		"avi" | "flv" | "mkv" | "mov" | "mp4" | "mpeg" | "mpg" | "vob" | "wmv" | "webm" => {
			(format!("5a_{ext}"), 5)
		}
		"aac" | "mp3" | "ogg" | "opus" | "wma" | "flac" | "wav" => (format!("5b_{ext}"), 6),

		"tmp" | "swp" | "swo" | "swn" | "bak" | "bkp" | "bk" | "parts" | "lock" => (format!("zzz_{ext}"), 9),
		_ => (ext, 0),
	}
}

pub fn filename(path: &Path) -> String {
	match path.file_name() {
		Some(name) => name.to_string_lossy().to_string(),
		_ => match path.canonicalize().unwrap().as_path().file_name() {
			Some(name) => name.to_string_lossy().to_string(),
			_ => path.display().to_string(),
		},
	}
}

pub fn filename_fmt(name: &str, ext: &str, egrp: u8, dir: bool, exe: bool, lnk: bool) -> String {
	if lnk {
		return format!("{CYAN}{name}");
	}
	if dir {
		return format!("{BLUE_L}{name}");
	}
	if exe {
		return format!("{GREEN_H}{name}");
	}

	let color = if egrp > 0 {
		match egrp {
			1 => IMG,
			2 => ZIP,
			3 => DOC,
			4 => CFG,
			5 => AVI,
			6 => MP3,
			7 => SRC,
			9 => TMP,
			_ => WHITE,
		}
	} else {
		match ext {
			"" => WHITE,
			"exe" => GREEN_H,
			_ => WHITE,
		}
	};

	format!("{color}{name}")
}
