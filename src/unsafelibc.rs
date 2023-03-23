// from https://github.com/redox-os/termion

use libc::{c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ};
use std::ffi::CStr;
use std::{io, mem};

// Support functions for converting libc return values to io errors {
trait IsMinusOne {
	fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
        ($($t:ident)*) => ($(impl IsMinusOne for $t {
            fn is_minus_one(&self) -> bool {
                *self == -1
            }
        })*)
    }

impl_is_minus_one! { i8 i16 i32 i64 isize }

fn cvt<T: IsMinusOne>(t: T) -> io::Result<T> {
	if t.is_minus_one() {
		Err(io::Error::last_os_error())
	} else {
		Ok(t)
	}
}
// } End of support functions

#[repr(C)]
struct TermSize {
	row: c_ushort,
	col: c_ushort,
	x: c_ushort,
	y: c_ushort,
}

pub fn terminal_size() -> io::Result<(usize, usize)> {
	unsafe {
		let mut size: TermSize = mem::zeroed();
		cvt(ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut size as *mut _))?;
		Ok((size.col as usize, size.row as usize))
	}
}

pub fn username_group(uid: u32, gid: u32) -> (String, String) {
	let grp = unsafe {
		let g = libc::getgrgid(gid);
		if g as usize > 0 {
			CStr::from_ptr((*g).gr_name)
		} else {
			CStr::from_bytes_with_nul(b"\0").unwrap()
		}
	};
	let usr = unsafe {
		let u = libc::getpwuid(uid);
		if u as usize > 0 {
			CStr::from_ptr((*u).pw_name)
		} else {
			CStr::from_bytes_with_nul(b"\0").unwrap()
		}
	};
	return (
		String::from_utf8_lossy(usr.to_bytes()).to_string(),
		String::from_utf8_lossy(grp.to_bytes()).to_string(),
	);
}
