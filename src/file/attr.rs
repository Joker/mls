use std::{io, path::Path, ptr::eq};
use exacl::AclEntry;

use crate::{
	color::WHITE,
	display::tree::{END, LEAF},
	ext::xattr::Attribute,
};

pub trait AclAttributes {
	fn access_lists(&self) -> io::Result<Vec<AclEntry>>;
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
impl AclAttributes for Path {
	fn access_lists(&self) -> io::Result<Vec<AclEntry>> {
		exacl::getfacl(self, None)
	}
}

pub fn exattr_fmt<'a>(
	lx: &'a Option<Vec<Attribute>>, la: &'a Option<Vec<AclEntry>>, wx: bool, detail: bool, width: usize,
) -> (&'a str, String) {
	if !wx {
		return ("", String::new());
	}
	match (lx, la) {
		(Some(x), Some(a)) => {
			if !detail {
				return ("@", String::new());
			}
			let mut qwe = xattr(x, width);
			qwe.extend(acl(a, width));
			("@", qwe.join(""))
		}
		(Some(x), None) => {
			if !detail {
				return ("@", String::new());
			}
			("@", xattr(x, width).join(""))
		}
		(None, Some(a)) => {
			if !detail {
				return ("+", String::new());
			}
			("+", acl(a, width).join(""))
		}
		_ => (" ", String::new()),
	}
}

fn xattr(atrs: &Vec<Attribute>, width: usize) -> Vec<String> {
	let last = atrs.iter().last().unwrap();
	atrs.iter()
		.map(|a| {
			format!(
				"\n{WHITE}{: >nsp$} {}",
				if eq(a, last) { END } else { LEAF },
				a.name,
				nsp = width,
			)
		})
		.collect::<Vec<String>>()
}

fn acl(atrs: &Vec<AclEntry>, width: usize) -> Vec<String> {
	atrs.iter()
		.map(|a| {
			format!(
				"\n{WHITE}{: >nsp$} {}:{} {} {} {}",
				"+",
				a.kind,
				a.name,
				if a.allow { "allow" } else { "deny" },
				a.perms,
				a.flags,
				nsp = width
			)
		})
		.collect::<Vec<String>>()
}
