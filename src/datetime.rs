// https://www.geeksforgeeks.org/convert-unix-timestamp-to-dd-mm-yyyy-hhmmss-format/
use std::time::{SystemTime, UNIX_EPOCH};

use crate::display::TIMEZONE;

fn leap_year(year: i32) -> bool {
	year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

fn curr_year(seconds: u64) -> (i32, u64) {
	let mut curr_year = 1970;
	let mut days_till_now = seconds / (24 * 60 * 60);
	loop {
		if leap_year(curr_year) {
			if days_till_now < 366 {
				return (curr_year, days_till_now);
			}
			days_till_now -= 366;
		} else {
			if days_till_now < 365 {
				return (curr_year, days_till_now);
			}
			days_till_now -= 365;
		}
		curr_year += 1;
	}
}

fn date_month(leap_year: bool, days_till_now_this_year: i64) -> (i64, usize) {
	let mut extra_days: i64 = days_till_now_this_year + 1;

	let days_of_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
	let mut month = 0;
	let mut index = 0;

	if leap_year {
		loop {
			if index == 1 {
				if extra_days - 29 < 0 {
					break;
				}
				month += 1;
				extra_days -= 29;
			} else {
				if extra_days - days_of_month[index] < 0 {
					break;
				}
				month += 1;
				extra_days -= days_of_month[index];
			}
			index += 1;
		}
	} else {
		loop {
			if extra_days - days_of_month[index] < 0 {
				break;
			}
			month += 1;
			extra_days -= days_of_month[index];
			index += 1;
		}
	};

	let date;
	if extra_days > 0 {
		month += 1;
		date = extra_days;
	} else {
		if month == 2 && leap_year {
			date = 29;
		} else {
			date = days_of_month[month - 1];
		}
	}

	(date, month)
}

pub fn date_time_fmt(sys_time: SystemTime) -> String {
	let seconds = sys_time.duration_since(UNIX_EPOCH).unwrap().as_secs() + TIMEZONE;

	let (curr_year, days_till_now) = curr_year(seconds);
	let (date, month) = date_month(leap_year(curr_year), days_till_now as i64);

	let extra_time = seconds % (24 * 60 * 60);
	let hours = extra_time / 3600;
	let minutes = (extra_time % 3600) / 60;
	let secondss = (extra_time % 3600) % 60;

	let month_name = [
		"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
	];

	format!(
		"{:0>2} {} {}  {:0>2}:{:0>2}:{:0>2}",
		date,
		month_name[month - 1],
		curr_year,
		hours,
		minutes,
		secondss
	)
}
