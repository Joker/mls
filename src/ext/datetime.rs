// https://stackoverflow.com/questions/11188621/how-can-i-convert-seconds-since-the-epoch-to-hours-minutes-seconds-in-java/11197532#11197532
pub fn seconds_to_datetime(unux_time: u64) -> (u64, usize, u64, u64, u64, u64) {
	let days_since_jan1st = [
		[0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365], // 365 days, non-leap
		[0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366], // 366 days, leap
	];
	let mut sec = unux_time + 11644473600;

	// let wday = (sec / 86400 + 1) % 7; // day of week

	let q = sec / 12622780800; // 400*365.2425*24*3600
	sec %= 12622780800;

	let mut c = sec / 3155673600; // 100*(365+24/100)*24*3600
	if c > 3 {
		c = 3;
	}
	sec -= c * 3155673600;

	let mut d = sec / 126230400; // 4*(365+1/4)*24*3600
	if d > 24 {
		d = 24;
	}
	sec -= d * 126230400;

	let mut a = sec / 31536000; // 365*24*3600
	if a > 3 {
		a = 3;
	}
	sec -= a * 31536000;

	// Calculate the year and find out if it's leap
	let year = 1601 + q * 400 + c * 100 + d * 4 + a;
	let leap = if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) { 1 } else { 0 };

	// Calculate the day of the year and the time
	let yday = sec / 86400;
	sec %= 86400;
	let hour = sec / 3600;
	sec %= 3600;
	let min = sec / 60;
	sec %= 60;

	// Calculate the month
	let mut mday = 1;
	let mut month = 1;
	for m in 1..13 {
		month = m;
		if yday < days_since_jan1st[leap][m] {
			mday += yday - days_since_jan1st[leap][m - 1];
			break;
		}
	}
	// 	wday [0,6]   (day of week)
	// 	yday [0,365] (day of year)
	(mday, month - 1, year, hour, min, sec)
}
