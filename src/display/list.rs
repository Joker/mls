use std::ptr::eq;

use crate::{
	args::Flags,
	color::WHITE,
	ext::xattr::Attribute,
	file::{size::size_fmt, time::date_time_fmt, time::TIMEZONE, File},
	Width,
};

use super::tree::{END, LEAF};

fn xattr_fmt(wx: bool, lx: &Option<Vec<Attribute>>, detail: bool, width: usize) -> (&str, String) {
	match wx {
		true => match lx {
			Some(_) if !detail => ("@", "".into()),
			Some(atrs) => {
				let last = atrs.iter().last().unwrap();
				(
					"@",
					atrs.iter()
						.map(|a| {
							format!(
								"\n{WHITE}{: >nsp$} {}",
								if eq(a, last) { END } else { LEAF },
								a.name,
								nsp = width
							)
						})
						.collect::<Vec<String>>()
						.join(""),
				)
			}
			_ => (" ", "".into()),
		},
		false => ("", "".into()),
	}
}

fn line_fmt(f: &File, fl: &Flags, w: &Width) -> String {
	match &f.line {
		Some(l) => {
			let usr_width = w.uid + 1;
			let grp_width = if fl.group { w.gid + 1 } else { 0 };
			let x_width = 32 + usr_width + grp_width + w.szn + if fl.octal { 8 } else { 0 };

			let (xsign, xattr) = xattr_fmt(w.xattr, &l.xattr, fl.xattr && !fl.tree_format, x_width);

			format!(
				"{}{WHITE}{}{: >ncu$}{: >ncg$}  {}  {}  {}{}",
				l.perm,
				xsign,
				l.user,
				l.group,
				date_time_fmt(f.line.as_ref().unwrap().time + TIMEZONE),
				size_fmt(f, w, fl.bytes),
				f.name,
				xattr,
				ncu = usr_width,
				ncg = grp_width,
			)
		}
		_ => "".to_string(),
	}
}

pub fn print(files: &[File], fl: &Flags, w: &Width) {
	println!(
		"{}",
		files
			.iter()
			.map(|f| line_fmt(f, fl, w))
			.collect::<Vec<_>>()
			.join("\n")
	)
}
