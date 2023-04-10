use crate::{
	args::Flags,
	color::WHITE,
	file::{size::size_fmt, time::date_time_fmt, time::TIMEZONE, File},
	Width,
};

fn line_fmt(f: &File, fl: &Flags, w: &Width) -> String {
	match &f.line {
		Some(l) => format!(
			"{WHITE}{}{WHITE}{}{: >ncu$}{: >ncg$}  {}  {}  {}",
			l.perm,
			match w.xattr {
				true if l.xattr => "@",
				true => " ",
				false => "",
			},
			l.user,
			l.group,
			date_time_fmt(f.line.as_ref().unwrap().time + TIMEZONE),
			size_fmt(f, w, fl.bytes),
			f.name,
			ncu = w.uid + 1,
			ncg = if fl.group { w.gid + 1 } else { 0 },
		),
		_ => "".to_string(),
	}
}

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
