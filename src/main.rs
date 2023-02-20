use std::{env, process};

use time::format_description::well_known::Rfc2822;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

mod parsers;

//const FORM: &[FormatItem<'_>] =
//	format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z |[weekday]");

fn display(date: &OffsetDateTime) {
	//println!("{}", date.format(&FORM).unwrap());
	println!("{}", date.format(&Rfc2822).unwrap());
	println!("{}", date.format(&Rfc3339).unwrap());
	println!("{}", date.unix_timestamp());
}

fn display_usage(arg0: &str) {
	println!("Usage: {} [options] [datetime]", arg0);
	println!("");
	println!("  datetime: A string representing a datetime to be formatted");
	println!("  options:");
	println!("    -h, --help, --usage: Print this usage information and exit");
	process::exit(1);
}

fn main() {
	let mut all_args = env::args();
	let arg0 = all_args.next().unwrap();
	let (params, time_bits): (Vec<String>, Vec<String>) =
		all_args.partition(|arg| arg.starts_with("-"));

	if params.contains(&"-h".to_string())
		|| params.contains(&"--help".to_string())
		|| params.contains(&"--usage".to_string())
		|| time_bits.contains(&"help".to_string())
	{
		display_usage(&arg0);
		return;
	}

	let opts = parsers::ParseOptions {
		now: OffsetDateTime::now_local().unwrap(),
	};

	let x = parsers::try_parse(&time_bits.join(" "), &opts);
	match x {
		Some(datetime) => display(&datetime),
		None => {
			println!("Could not parse input!");
			process::exit(2);
		}
	}
}
