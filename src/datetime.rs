// https://www.geeksforgeeks.org/convert-unix-timestamp-to-dd-mm-yyyy-hhmmss-format/
use std::time::{SystemTime, UNIX_EPOCH};

use crate::display::TIMEZONE;

fn leap_year(year: i32) -> bool {
	year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

fn current_year(seconds: u64) -> (i32, u64) {
	let mut curr_year = 1970;
	let mut days_till_now = seconds / (24 * 60 * 60);
	loop {
		match leap_year(curr_year) {
			true => {
				if days_till_now < 366 {
					return (curr_year, days_till_now);
				}
				days_till_now -= 366;
			}
			false => {
				if days_till_now < 365 {
					return (curr_year, days_till_now);
				}
				days_till_now -= 365;
			}
		}
		curr_year += 1;
	}
}

fn date_month(leap_year: bool, curr_year_days: i64) -> (i64, usize) {
	let mut extra_days: i64 = curr_year_days + 1;

	let days_of_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
	let mut month = 0;
	let mut i = 0;

	match leap_year {
		true => loop {
			if i == 1 {
				if extra_days - 29 <= 0 {
					break;
				}
				month += 1;
				extra_days -= 29;
			} else {
				if extra_days - days_of_month[i] <= 0 {
					break;
				}
				month += 1;
				extra_days -= days_of_month[i];
			}
			i += 1;
		},
		false => loop {
			if extra_days - days_of_month[i] <= 0 {
				break;
			}
			month += 1;
			extra_days -= days_of_month[i];
			i += 1;
		},
	};

	if extra_days > 0 {
		return (extra_days, month + 1);
	}
	if month == 2 && leap_year {
		return (29, month);
	}

	return (days_of_month[month - 1], month);
}

pub fn date_time_fmt(unix_time: u64) -> String {
	// pub fn date_time_fmt(sys_time: SystemTime) -> String {
	let now = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs()
		+ TIMEZONE;
	// let unix_time = sys_time.duration_since(UNIX_EPOCH).unwrap().as_secs() + TIMEZONE;
	let remaind_time = unix_time % (24 * 60 * 60);

	let (curr_year, curr_year_days) = current_year(unix_time);
	let (date, month) = date_month(leap_year(curr_year), curr_year_days as i64);

	let hours = remaind_time / 3600;
	let minutes = (remaind_time % 3600) / 60;
	// let seconds = (remaind_time % 3600) % 60;

	let month_name = [
		"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
	];
	if unix_time > now - 15768000 {
		return format!(
			"{: >2} {} {:0>2}:{:0>2}",
			date,
			month_name[month - 1],
			hours,
			minutes,
		);
	}
	format!("{: >2} {}  {}", date, month_name[month - 1], curr_year,)
}
