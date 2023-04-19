// use crate::color::{BLACK_H, BLUE_L, CYAN, GREEN, GREEN_L, MAGENTA, RED, RED_L, YELLOW, YELLOW_L};
use crate::{
	args::Flags,
	color::{BLACK_H, BLUE_L, CYAN, GREEN, GREEN_H, GREEN_L, MAGENTA, MAGENTA_H, OCT, RED, RED_L, YELLOW},
};

const BLOCK_DEV: u32 = 0o060000;
const DIR: u32 = 0o040000;
const CHAR_DEV: u32 = 0o020000;
const PIPE: u32 = 0o010000; // S_IFIFO
const FILE: u32 = 0o100000; // S_IFREG

const STICKY_UID: u32 = 0o004000;
const STICKY_GID: u32 = 0o002000;
const STICKY_DIR: u32 = 0o001000;

pub const USER_EXE: u32 = 0o000100; // S_IXUSR

// const LNK:  u32 = 0o120000;
// const SOCK: u32 = 0o140000;

fn kind(rwx: u32, nlink: u64) -> String {
	let has_bit = |bit| rwx & bit == bit;

	if has_bit(BLOCK_DEV) {
		return format!("{GREEN_H}b");
	}
	if has_bit(DIR) {
		return match has_bit(FILE) {
			true => format!("{RED_L}s"),
			false => format!("{BLUE_L}d"),
		};
	}
	if has_bit(CHAR_DEV) {
		return match has_bit(FILE) {
			true => format!("{CYAN}l"),
			false => format!("{MAGENTA_H}c"),
		};
	}
	if has_bit(PIPE) {
		return format!("{GREEN_L}│"); // p
	}

	return match nlink {
		n if n > 9 => format!("{MAGENTA}*"),
		n if n > 1 => format!("{MAGENTA}{n}"),
		_ => String::from(" "),
	};
}

pub fn permissions_fmt(rwx: u32, nlink: u64, fl: &Flags) -> String {
	let mut vp = [
		kind(rwx, nlink),
		// format!("{YELLOW_L}r"),
		// format!("{YELLOW_L}r"),
		// format!("{RED_L}w"),
		// format!("{GREEN_L}x"),
		// format!("{YELLOW}r"),
		// format!("{RED}w"),
		// format!("{GREEN}x"),
		// format!("{YELLOW}r"),
		// format!("{RED}w"),
		// format!("{GREEN}x"),
		format!("{GREEN}r"),
		format!("{YELLOW}w"),
		format!("{RED}x"),
		format!("{GREEN}r"),
		format!("{YELLOW}w"),
		format!("{RED}x"),
		format!("{GREEN}r"),
		format!("{YELLOW}w"),
		format!("{RED}x"),
	];
	for (i, one) in bits(rwx, 9).iter().enumerate() {
		if !one {
			vp[i + 1] = format!("{BLACK_H}-");
		}
	}

	if has_bit(rwx, STICKY_UID) {
		vp[3] = format!("{MAGENTA_H}S");
	}
	if has_bit(rwx, STICKY_GID) {
		vp[6] = format!("{MAGENTA_H}S");
	}
	if has_bit(rwx, STICKY_DIR) {
		vp[9] = format!("{MAGENTA_H}t");
	}

	if fl.octal {
		let o = format!("{: >6o}", rwx);
		let spl = o.split_at(3);
		return format!("{BLACK_H}{}{OCT}{}  {}", spl.0, spl.1, vp.join(""));
	}
	vp.join("")
}

fn bits(rwx: u32, n: u8) -> Vec<bool> {
	let mut v = (0..n)
		.map(|n| if (rwx >> n) & 1 == 1 { true } else { false })
		.collect::<Vec<bool>>();
	v.reverse();
	v
}

fn has_bit(rwx: u32, bit: u32) -> bool {
	rwx & bit == bit
}
