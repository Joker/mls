use std::fs::Metadata;
use std::os::unix::prelude::MetadataExt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
	args::Flags,
	color::{DAY1, DAY2, WEK1, WEK4, YEAR},
	ext::datetime::seconds_to_datetime,
};

pub const TIMEZONE: u64 = 10800;

pub fn unix(md: &Metadata, fl: &Flags) -> u64 {
	if fl.create {
		match md.created().ok() {
			Some(t) => match t.duration_since(UNIX_EPOCH) {
				Ok(s) => s.as_secs(),
				Err(_) => 0,
			},
			None => 0,
		}
	} else if fl.access {
		md.atime() as u64
	} else if fl.ctime {
		md.ctime() as u64
	} else {
		md.mtime() as u64
	}
}

pub fn date_time_fmt(unix_time: u64) -> String {
	let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + TIMEZONE;

	let (date, month, year, hours, minutes, _) = seconds_to_datetime(unix_time);

	let month_name = [
		"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
	];
	let cl = match unix_time {
		ut if ut > now - 86400 => DAY1,
		ut if ut > now - 172800 => DAY2,
		ut if ut > now - 604800 => WEK1,
		ut if ut > now - 2419200 => WEK4,
		_ => YEAR,
	};
	if unix_time > now - 15768000 {
		return format!(
			"{}{: >2} {} {:0>2}:{:0>2}",
			cl, date, month_name[month], hours, minutes
		);
	}

	format!("{}{: >2} {}  {}", cl, date, month_name[month], year,)
}
