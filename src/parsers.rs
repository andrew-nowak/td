use time::macros::time;
use time::{Duration, Month, OffsetDateTime, Weekday};

pub struct ParseOptions {
	pub(crate) now: OffsetDateTime,
}

pub fn try_parse(ts: &str, opts: &ParseOptions) -> Option<OffsetDateTime> {
	(if ts.is_empty() {
		opts.now.replace_nanosecond(0).ok()
	} else {
		None
	})
	.or_else(|| as_year(ts))
	.or_else(|| as_unix_timestamp(ts))
	.or_else(|| as_weekday(ts, opts))
	.or_else(|| as_month(ts, opts))
}

fn as_unix_timestamp(ts: &str) -> Option<OffsetDateTime> {
	let number = ts.parse::<i128>().ok()?;

	let as_secs: Option<i64> = number.try_into().ok();
	let as_secs: Option<OffsetDateTime> = as_secs
		.and_then(|secs| OffsetDateTime::from_unix_timestamp(secs).ok())
		.and_then(|sts| if sts.year() > 9999 { None } else { Some(sts) });

	as_secs.or_else(|| OffsetDateTime::from_unix_timestamp_nanos(number * 1_000_000).ok())
}

fn as_month(ts: &str, opts: &ParseOptions) -> Option<OffsetDateTime> {
	let month: Option<Month> = match ts.to_lowercase().as_str() {
		"january" => Some(Month::January),
		"february" => Some(Month::February),
		"march" => Some(Month::March),
		"april" => Some(Month::April),
		"may" => Some(Month::May),
		"june" => Some(Month::June),
		"july" => Some(Month::July),
		"august" => Some(Month::August),
		"september" => Some(Month::September),
		"october" => Some(Month::October),
		"november" => Some(Month::November),
		"december" => Some(Month::December),
		_ => None,
	};

	month.map(|mo| {
		let month_number: u8 = mo.into();
		let now_month_number: u8 = opts.now.month().into();
		(if month_number <= now_month_number {
			opts.now
				.replace_year(opts.now.year() + 1)
				.unwrap()
				.replace_month(mo)
				.unwrap()
		} else {
			opts.now.replace_month(mo).unwrap()
		})
		.replace_time(time!(00:00))
		.replace_day(1)
		.unwrap()
	})
}

fn as_year(ts: &str) -> Option<OffsetDateTime> {
	let number = ts.parse::<i32>().ok()?;

	if number > 9999 || number <= 0 {
		None
	} else {
		OffsetDateTime::UNIX_EPOCH
			.to_owned()
			.replace_year(number)
			.ok()
	}
}

fn as_weekday(ts: &str, opts: &ParseOptions) -> Option<OffsetDateTime> {
	let d = match ts {
		"Monday" | "monday" => Some(Weekday::Monday),
		"Tuesday" | "tuesday" => Some(Weekday::Tuesday),
		"Wednesday" | "wednesday" => Some(Weekday::Wednesday),
		"Thursday" | "thursday" => Some(Weekday::Thursday),
		"Friday" | "friday" => Some(Weekday::Friday),
		"Saturday" | "saturday" => Some(Weekday::Saturday),
		"Sunday" | "sunday" => Some(Weekday::Sunday),
		_ => None,
	};

	d.map(|wd| {
		let wd: i16 = wd.number_days_from_monday().into();
		let now_wd: i16 = opts.now.weekday().number_days_from_monday().into();

		let mut diff = wd - now_wd;
		if diff <= 0 {
			diff += 7;
		}

		let diff: i64 = diff.into();

		opts.now
			.saturating_add(Duration::days(diff))
			.replace_time(time!(00:00)) // reset to start of day
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	use time::macros::datetime;

	#[test]
	fn test_as_unix_timestamp() {
		assert_eq!(
			as_unix_timestamp("1500000000"),
			Some(datetime!(2017-07-14 02:40:00 UTC))
		);
		assert_eq!(as_unix_timestamp("notunix"), None);
	}

	#[test]
	fn test_as_unix_timestamp_millis() {
		assert_eq!(
			as_unix_timestamp("1500000000000"),
			Some(datetime!(2017-07-14 02:40:00 UTC))
		);
	}

	#[test]
	fn test_as_year() {
		assert_eq!(as_year("2020"), Some(datetime!(2020-01-01 00:00:00 UTC)));
	}

	#[test]
	fn test_as_month() {
		let opt = ParseOptions {
			now: datetime!(2017-07-14 02:40:00 UTC),
		};

		assert_eq!(
			as_month("February", &opt),
			Some(datetime!(2018-02-01 00:00:00 UTC))
		);

		assert_eq!(
			as_month("august", &opt),
			Some(datetime!(2017-08-01 00:00:00 UTC))
		);
		assert_eq!(
			as_month("july", &opt),
			Some(datetime!(2018-07-01 00:00:00 UTC))
		);
	}

	#[test]
	fn test_as_weekday() {
		let opt = ParseOptions {
			now: datetime!(2106-03-03 12:00:00 UTC),
		};
		assert_eq!(
			as_weekday("Monday", &opt),
			Some(datetime!(2106-03-08 00:00:00 UTC))
		);
		let opt = ParseOptions {
			now: datetime!(2106-03-03 12:00:00 +05:00),
		};
		assert_eq!(
			as_weekday("Monday", &opt),
			Some(datetime!(2106-03-08 00:00:00 +05:00))
		);
	}
}
