// pub static BLACK: &str = "\x1b[0;30m";
pub static RED: &str = "\x1b[0;31m";
// pub static GREEN: &str = "\x1b[0;32m";
// pub static YELLOW: &str = "\x1b[0;33m";
// pub static BLUE: &str = "\x1b[0;34m";
// pub static MAGENTA: &str = "\x1b[0;35m";
// pub static CYAN: &str = "\x1b[0;36m";
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
// pub static GREEN_L: &str = "\x1b[0;92m";
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
	if exe {
		return format!("{GREEN_H}{name}");
	}

	if lnk {
		name.to_string().push('@');
	}
	let color = if egrp > 0 {
		match egrp {
			1 => MAGENTA_L,
			2 => RED,
			3 => XT22,
			4 => XT36,
			_ => WHITE,
		}
	} else if ext.len() > 0 {
		match ext {
			"exe" => GREEN_H,
			_ => WHITE,
		}
	} else {
		WHITE
	};

	return format!("{color}{name}");
}
