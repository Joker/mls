// pub static BLACK: &str = "\x1b[0;30m";
pub static RED: &str = "\x1b[0;31m";
pub static GREEN: &str = "\x1b[0;32m";
// pub static YELLOW: &str = "\x1b[0;33m";
// pub static BLUE: &str = "\x1b[0;34m";
// pub static MAGENTA: &str = "\x1b[0;35m";
pub static CYAN: &str = "\x1b[0;36m";
pub static WHITE: &str = "\x1b[0;37m";
// pub static BLACK_H: &str = "\x1b[1;30m";
// pub static RED_H: &str = "\x1b[1;31m";
pub static GREEN_H: &str = "\x1b[1;32m";
// pub static YELLOW_H: &str = "\x1b[1;33m";
// pub static BLUE_H: &str = "\x1b[1;34m";
// pub static MAGENTA_H: &str = "\x1b[1;35m";
// pub static CYAN_H: &str = "\x1b[1;36m";
// pub static WHITE_H: &str = "\x1b[1;37m";
// pub static BLACK_L: &str = "\x1b[0;90m";
// pub static RED_L: &str = "\x1b[0;91m";
pub static GREEN_L: &str = "\x1b[0;92m";
// pub static YELLOW_L: &str = "\x1b[0;93m";
pub static BLUE_L: &str = "\x1b[0;94m";
pub static MAGENTA_L: &str = "\x1b[0;95m";
// pub static CYAN_L: &str = "\x1b[0;96m";
// pub static WHITE_L: &str = "\x1b[0;97m";
pub static XT22: &str = "\x1b[38;5;22m";
pub static XT36: &str = "\x1b[38;5;36m";

pub fn colorise(name: &str, ext: &str, dir: bool, exe: bool, egrp: u8, lnk: bool) -> String {
	if dir {
		return format!("{BLUE_L}{name}");
	}
	if lnk {
		return format!("{CYAN}{name}");
	}
	if exe {
		return format!("{GREEN_H}{name}");
	}

	let color = if egrp > 0 {
		match egrp {
			1 => MAGENTA_L,
			2 => RED,
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

pub fn size_fmt(bytes: u64) -> String {
	match bytes {
		bt if bt >= 1073741824 => {
			let n = bt / 1073741824;
			let m = bt % 1073741824;
			return if m > 0 && n < 100 {
				format!(
					"{GREEN_L}{}.{}{GREEN}G",
					n,
					m.to_string().chars().next().unwrap()
				)
			} else {
				format!("{GREEN_L}{}{GREEN}G", n)
			};
		}
		bt if bt >= 1048576 => {
			let n = bt / 1048576;
			let m = bt % 1048576;
			return if m > 0 && n < 100 {
				format!(
					"{GREEN_L}{}.{}{GREEN}M",
					n,
					m.to_string().chars().next().unwrap()
				)
			} else {
				format!("{GREEN_L}{}{GREEN}M", n)
			};
		}
		bt if bt >= 1024 => {
			let n = bt / 1024;
			let m = bt % 1024;
			return if m > 0 && n < 100 {
				format!(
					"{GREEN_L}{}.{}{GREEN}k",
					n,
					m.to_string().chars().next().unwrap()
				)
			} else {
				format!("{GREEN_L}{}{GREEN}k", n)
			};
		}
		bt if bt >= 1 => format!("{GREEN_L}{}{GREEN}", bt),
		_ => format!("{GREEN_L}0{GREEN}"),
	}
}
