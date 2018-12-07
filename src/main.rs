extern crate clap;
extern crate regex;
extern crate serde_json;
extern crate chrono;
extern crate itertools;

#[macro_use]
extern crate serde_derive;

use clap::{Arg,App};
use std::io::{self, BufRead};
use std::io::{BufReader};
use std::fs::File;

mod stats;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
// mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

fn main() {
	let matches = App::new("Advent of Code")
		.author("Galen Elias, gelias@gmail.com")
		.version("0.1.0")
		.about("Advent of code solutions in Rust")
		.arg(
			Arg::with_name("day")
				.short("d")
				.required(true)
				.index(1)
				.help("specifies which day's challenge to run")
				.validator(|str|
					str.parse::<u32>()
						.or(Err("day must be an integer".to_owned()))
						.and_then(|v| match v {
							0...25 => Ok(()),
							_ => Err("day must be between 1 and 25".to_owned())
						})))
		.arg(
			Arg::with_name("stats")
				.long("stats")
				.help("Parses leaderboard JSON into a readable format"))
		.arg(
			Arg::with_name("file")
				.short("f")
				.takes_value(true)
				.help("Uses a file instead of reading from standard in"))
		.after_help("Longer explaination to appear after the options when \
					displaying the help information from --help or -h")
		.get_matches();

	let input;
	if matches.is_present("file") {
		let f = File::open(matches.value_of("file").unwrap()).unwrap();
		let file = BufReader::new(&f);
		input = file.lines()
			.filter_map(|l| l.ok())
			.collect::<Vec<String>>();
	} else {
		let stdin = io::stdin();
		input = stdin.lock().lines()
			.filter_map(|l| l.ok())
			.collect::<Vec<String>>();
	}

	if matches.is_present("stats") {
		stats::show_stats(matches.value_of("day").unwrap_or("0").parse::<u32>().unwrap(), input);	
		return;
	}

	let day = matches.value_of("day").unwrap().parse::<u32>().unwrap();
	match day {
		1 => day1::solve(input),
		2 => day2::solve(input),
		3 => day3::solve(input),
		4 => day4::solve(input),
		5 => day5::solve(input),
		6 => day6::solve(input),
		7 => day7::solve(input),
		// 8 => day8::solve(input),
		// 9 => day9::solve(input),
		// 10 => day10::solve(input),
		// 11 => day11::solve(input),
		// 12 => day12::solve(input),
		// 13 => day13::solve(input),
		// 14 => day14::solve(input),
		// 15 => day15::solve(input),
		// 16 => day16::solve(input),
		// 17 => day17::solve(input),
		// 18 => day18::solve(input),
		// 19 => day19::solve(input),
		// 20 => day20::solve(input),
		// 21 => day21::solve(input),
		// 22 => day22::solve(input),
		// 23 => day23::solve(input),
		// 24 => day24::solve(input),
		// 25 => day25::solve(input),
		_ => println!("Oops! Day {} isn't implemented yet!", day)
	}
}
