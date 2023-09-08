use crate::{
	color::{BLACK_H, GREEN, SIZE, SUFFIX, WHITE},
	Width,
};

use super::File;

pub fn size_fmt(f: &File, w: &Width, bitsize: bool) -> String {
	let line = f.line.as_ref().unwrap();
	if f.dir && (line.lnk || line.size == 0) {
		return format!("{BLACK_H}{: >nsz$}", "-", nsz = w.szn);
	}
	if f.dir {
		return format!("{WHITE}{: >nsz$}{BLACK_H}f", line.size, nsz = w.szn - 1);
	}
	if bitsize {
		return format!("{WHITE}{: >nsz$}", line.size, nsz = w.szn);
	}
	match line.size_suf.as_str() {
		"M" | "G" => format!(
			"{SIZE}{: >nsz$}{SUFFIX}{}",
			line.size_str,
			line.size_suf,
			nsz = w.szn - 1
		),
		"k" => format!(
			"{GREEN}{: >nsz$}{SUFFIX}{}",
			line.size_str,
			line.size_suf,
			nsz = w.szn - 1
		),
		_ => format!("{GREEN}{: >nsz$}", line.size_str, nsz = w.szn),
	}
}

pub fn size_to_string(bytes: u64) -> (String, String) {
	match bytes {
		bt if bt >= 1073741824 => (short_size(bt as f64), "G".to_string()),
		bt if bt >= 1048576 => (short_size(bt as f64), "M".to_string()),
		bt if bt >= 1024 => (short_size(bt as f64), "k".to_string()),
		bt if bt >= 1 => (bt.to_string(), String::new()),
		_ => ("0".to_string(), String::new()),
		// _ => format!("{GREEN_H}{}0{GREEN}", spaces(FSIZE_WIDTH - 1)),
	}
}

const KB: f64 = 1024.0;
fn short_size(bytes: f64) -> String {
	let base = bytes.log10() / KB.log10();
	let ans = KB.powf(base - base.floor());
	if ans < 100.0 {
		return format!("{:.1}", ans).trim_end_matches(".0").to_owned();
	}
	format!("{:.0}", ans)
}
