// from https://github.com/redox-os/termion

use libc::{c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ};
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
