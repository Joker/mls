use crate::color::{BLACK_H, BLACK_L, GREEN, GREEN_L, WHITE};
use crate::datetime::date_time_fmt;
use crate::display::TIMEZONE;
use crate::info::File;
use crate::{Flags, Width};

pub fn print(files: &Vec<File>, fl: &Flags, w: &Width) {
	println!(
		"{}",
		files
			.iter()
			.map(|f| line_fmt(f, fl, w))
			.collect::<Vec<_>>()
			.join("\n")
	)
}

fn line_fmt(f: &File, fl: &Flags, w: &Width) -> String {
	match &f.long {
		Some(l) => format!(
			"{WHITE}{}{WHITE}{: >ncu$}{: >ncg$}  {}  {}  {}",
			l.perm,
			l.user,
			l.group,
			date_time_fmt(f.time + TIMEZONE),
			size_fmt(f, w, fl.human),
			f.name,
			ncu = w.uid + 1,
			ncg = if fl.group { w.gid + 1 } else { 0 },
		),
		_ => "".to_string(),
	}
}

pub fn size_fmt(f: &File, w: &Width, bitsize: bool) -> String {
	let line = f.long.as_ref().unwrap();
	if f.dir && (line.lnk || f.size == 0) {
		return format!("{BLACK_H}{: >nsz$}", "-", nsz = w.szn);
	}
	if f.dir {
		return format!("{WHITE}{: >nsz$}{BLACK_L}f", f.size, nsz = w.szn - 1);
	}
	if bitsize {
		return format!("{WHITE}{: >nsz$}", f.size, nsz = w.szn);
	}
	if line.suf.len() > 0 {
		format!(
			"{GREEN}{: >nsz$}{GREEN_L}{}",
			line.size,
			line.suf,
			nsz = w.szn - 1
		)
	} else {
		format!("{GREEN}{: >nsz$}", line.size, nsz = w.szn)
	}
}

pub fn size_to_string(bytes: u64) -> (String, String) {
	match bytes {
		bt if bt >= 1073741824 => (short_size(bt as f64), "G".to_string()),
		bt if bt >= 1048576 => (short_size(bt as f64), "M".to_string()),
		bt if bt >= 1024 => (short_size(bt as f64), "k".to_string()),
		bt if bt >= 1 => (bt.to_string(), "".to_string()),
		_ => ("0".to_string(), "".to_string()),
		// _ => format!("{GREEN_L}{}0{GREEN}", spaces(FSIZE_WIDTH - 1)),
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
