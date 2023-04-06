use crate::color::{BLACK_H, BLUE_L, CYAN, GREEN, GREEN_L, MAGENTA, RED, RED_L, YELLOW, YELLOW_L};

pub fn kind_fmt(lnk: bool, dir: bool, nlink: u64) -> String {
	if lnk {
		return format!("{CYAN}l");
	}
	if dir {
		return format!("{BLUE_L}d");
	}
	match nlink {
		n if n > 9 => return format!("{MAGENTA}*"),
		n if n > 1 => return format!("{MAGENTA}{n}"),
		_ => return String::from(" "),
	}
}

pub fn permissions_fmt(rwx: u32) -> String {
	let mut out = String::from("");
	let vp = [
		format!("{YELLOW_L}r"),
		format!("{RED_L}w"),
		format!("{GREEN_L}x"),
		format!("{YELLOW}r"),
		format!("{RED}w"),
		format!("{GREEN}x"),
		format!("{YELLOW}r"),
		format!("{RED}w"),
		format!("{GREEN}x"),
		// format!("{GREEN}r"),
		// format!("{YELLOW}w"),
		// format!("{RED}x"),
		// format!("{GREEN}r"),
		// format!("{YELLOW}w"),
		// format!("{RED}x"),
		// format!("{GREEN}r"),
		// format!("{YELLOW}w"),
		// format!("{RED}x"),
	];
	for (i, one) in bits(rwx, 9).iter().enumerate() {
		if *one {
			out.push_str(&vp[i]);
		} else {
			out.push_str(&format!("{BLACK_H}-"));
		}
	}
	out
}

fn bits(rwx: u32, n: u8) -> Vec<bool> {
	let mut v = (0..n)
		.map(|n| if (rwx >> n) & 1 == 1 { true } else { false })
		.collect::<Vec<bool>>();
	v.reverse();
	v
}
