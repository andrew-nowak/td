use std::env;

use time::format_description::FormatItem;
use time::{macros::format_description, OffsetDateTime};

const FORM: &[FormatItem<'_>] =
	format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z |[weekday]");

fn as_unix_timestamp(ts: &str) -> Option<OffsetDateTime> {
	let number = ts.parse::<i128>().ok()?;

	let as_secs: Option<i64> = number.try_into().ok();
	let as_secs: Option<OffsetDateTime> = as_secs
		.and_then(|secs| OffsetDateTime::from_unix_timestamp(secs).ok())
		.and_then(|sts| if sts.year() > 9999 { None } else { Some(sts) });

	as_secs.or_else(|| OffsetDateTime::from_unix_timestamp_nanos(number * 1_000_000).ok())
}

fn display(date: &OffsetDateTime) {
	println!("{}", date.format(&FORM).unwrap());
}

fn main() {
	let (params, time_bits): (Vec<String>, Vec<String>) =
		env::args().skip(1).partition(|arg| arg.starts_with("-"));

	if time_bits.len() == 1 {
		let x = as_unix_timestamp(&time_bits[0]);
		display(&x.unwrap());
	}
}
