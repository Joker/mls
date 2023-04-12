use std::path::Path;

use crate::color::{BLUE_L, CYAN, GREEN_H, MAGENTA_L, WHITE, XT22, XT36, YELLOW};

pub fn basepath(path: &Path) -> String {
	match path.parent() {
		Some(p) => {
			let mut path = p.to_string_lossy().to_string();
			if path.len() > 0 {
				path += "/"
			}
			path
		}
		_ => "".to_string(),
	}
}

pub fn ext(path: &Path) -> String {
	match path.extension() {
		Some(ext) => ext.to_string_lossy().to_lowercase(),
		_ => "".to_string(),
	}
}

pub fn ext_group(ext: String) -> (String, u8) {
	match ext.as_str() {
		"png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "ico" | "bmp" | "tiff" => (format!("1_{ext}"), 1),

		"7z" | "zip" | "tar" | "gz" | "bz2" | "rar" | "tgz" | "xz" | "txz" => (format!("2_{ext}"), 2),

		"djvu" | "doc" | "docx" | "dotx" | "odp" | "odt" | "pdf" | "ppt" | "pptx" | "rtf" | "xls"
		| "xlsx" => (format!("3_{ext}"), 3),

		"html" | "css" | "scss" | "sass" | "js" | "jsx" | "ts" | "tsx" | "go" | "rs" | "java" => {
			(format!("4a_{ext}"), 4)
		}
		"json" | "txt" | "md" | "csv" | "yaml" => (format!("4b_{ext}"), 4),

		"avi" | "flv" | "mkv" | "mov" | "mp4" | "mpeg" | "mpg" | "vob" | "wmv" | "webm" => {
			(format!("5a_{ext}"), 5)
		}
		"aac" | "mp3" | "ogg" | "opus" | "wma" | "flac" | "wav" => (format!("5b_{ext}"), 6),

		"tmp" | "swp" | "swo" | "swn" | "bak" | "bkp" | "bk" | "parts" => (format!("zzz_{ext}"), 9),
		_ => (ext, 0),
	}
}

pub fn filename(path: &Path) -> String {
	match path.file_name() {
		Some(name) => name.to_string_lossy().to_string(),
		_ => path.display().to_string(),
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
			1 => MAGENTA_L,
			2 => YELLOW,
			3 => XT22,
			4 => XT36,
			_ => WHITE,
		}
	} else {
		match ext {
			"" => WHITE,
			"exe" => GREEN_H,
			_ => WHITE,
		}
	};

	return format!("{color}{name}");
}
