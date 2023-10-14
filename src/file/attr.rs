use exacl::AclEntry;
use std::{path::PathBuf, ptr::eq};

use crate::{
	color::WHITE,
	display::tree::{END, LEAF},
	ext::xattr::{Attribute, FileAttributes},
};

#[derive(Clone, Debug)]
pub struct Xattr {
	pub xattr: Option<Vec<Attribute>>,
	pub acl: Option<Vec<AclEntry>>,
}

pub fn ext_attr(path: &PathBuf) -> Option<Xattr> {
	let xattr = match path.attributes() {
		Ok(xa) if !xa.is_empty() => Some(xa),
		_ => None,
	};
	let acl = match exacl::getfacl(path, None) {
		Ok(al) if !al.is_empty() => Some(al),
		_ => None,
	};

	if xattr.is_none() && acl.is_none() {
		None
	} else {
		Some(Xattr { xattr, acl })
	}
}

pub fn ext_attr_fmt<'a>(lists: &'a Option<Xattr>, exist: bool, detail: bool, width: usize) -> (&'a str, String) {
	if !exist {
		return ("", String::new());
	}
	match lists {
		Some(Xattr {
			xattr: Some(x),
			acl: Some(a),
		}) => {
			if !detail {
				return ("@", String::new());
			}
			let mut ext = xattr_fmt(x, width);
			ext.extend(acl_fmt(a, width));
			("@", ext.join(""))
		}
		Some(Xattr {
			xattr: Some(x),
			acl: None,
		}) => {
			if !detail {
				return ("@", String::new());
			}
			("@", xattr_fmt(x, width).join(""))
		}
		Some(Xattr {
			xattr: None,
			acl: Some(a),
		}) => {
			if !detail {
				return ("+", String::new());
			}
			("+", acl_fmt(a, width).join(""))
		}
		_ => (" ", String::new()),
	}
}

fn xattr_fmt(atrs: &Vec<Attribute>, width: usize) -> Vec<String> {
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

fn acl_fmt(atrs: &Vec<AclEntry>, width: usize) -> Vec<String> {
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
