use crate::{
	args::Flags,
	color::{RESET, WHITE},
	file::{
		attr::exattr_fmt,
		size::size_fmt,
		time::{date_time_fmt, TIMEZONE},
		File,
	},
	Width,
};

fn line_fmt(f: &File, fl: &Flags, w: &Width) -> String {
	match &f.line {
		Some(l) => {
			let usr_width = w.uid + 1;
			let grp_width = if fl.group { w.gid + 1 } else { 0 };
			let x_width = 32 + usr_width + grp_width + w.szn + if fl.octal { 8 } else { 0 };

			let (attr, exattr) = exattr_fmt(&l.xattr, &l.acl, w.xattr, fl.xattr && !fl.tree_format, x_width);

			format!(
				"{: >ind$}{}{WHITE}{}{: >ncu$}{: >ncg$}  {}  {}  {}{}",
				l.inode,
				l.perm,
				attr,
				l.user,
				l.group,
				date_time_fmt(f.line.as_ref().unwrap().time + TIMEZONE),
				size_fmt(f, w, fl.bytes),
				f.name,
				exattr,
				ind = w.inode,
				ncu = usr_width,
				ncg = grp_width,
			)
		}
		_ => String::new(),
	}
}

pub fn print(files: &[File], fl: &Flags, w: &Width) {
	println!(
		"{}{RESET}",
		files
			.iter()
			.map(|f| line_fmt(f, fl, w))
			.collect::<Vec<_>>()
			.join("\n")
	)
}
