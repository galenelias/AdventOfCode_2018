
extern crate serde_json;

use serde_json::{Value, Map};
use std::cmp;
use chrono::prelude::*;

#[derive(Deserialize)]
struct Member {
	local_score : i32,
	name : String,
	completion_day_level : Map<String, Value>,
}

#[derive(Deserialize)]
struct LeaderBoard {
	members : Map<String, Value>,
}

pub fn show_stats(day_filter : u32, input : Vec<String>) {
	let leaderboard: LeaderBoard = serde_json::from_str(input[0].as_ref()).unwrap();

	let mut members : Vec<Member> = Vec::new();
	let mut max_day = 1;

	for (_id, mem_json) in leaderboard.members {
		let m : Member = serde_json::from_value(mem_json).unwrap();

		for (day, _) in &m.completion_day_level {
			max_day = cmp::max(max_day, day.parse::<u32>().unwrap());
		}

		if (day_filter == 0 && !m.completion_day_level.is_empty())
			|| (day_filter != 0 && m.completion_day_level.contains_key(&day_filter.to_string())) {
			members.push(m);
		}
	}

	if day_filter == 0 {
		members.sort_by(|a,b| b.local_score.cmp(&a.local_score));
	} else {
		let get_day_score = |m: &Member| -> String { m.completion_day_level[&day_filter.to_string()].as_object().unwrap()["1"]["get_star_ts"].as_str().unwrap().to_string() };
		members.sort_by(|a,b| get_day_score(a).cmp(&get_day_score(b)));
	}

	let column_width = 19;

	print!("        ");
	for mem in &members {
		print!("    {:^col$} ", mem.name, col=column_width);
	}
	println!("");
	
	for day in 1..=max_day {
		if day_filter != 0 && day_filter != day {
			continue;
		}
		let str_day = day.to_string();
		for star in 1..=2 {
			let str_star = star.to_string();
			print!("{:>2}-{}:  ", str_day, str_star);

			let mut times : Vec<DateTime<Utc>>  = Vec::new();

			for mem in &members {
				if mem.completion_day_level.contains_key(&str_day) {
					let data = &mem.completion_day_level[&str_day].as_object().unwrap();
					if data.contains_key(&str_star) {
						let date_str = data[&str_star]["get_star_ts"].as_str().unwrap();
						let naive = NaiveDateTime::from_timestamp(date_str.parse::<i64>().unwrap(), 0);
						let date = DateTime::from_utc(naive, Utc);
						times.push(date);
					}
				}
			}

			times.sort();

			for mem in &members {
				if mem.completion_day_level.contains_key(&str_day) {
					let data = &mem.completion_day_level[&str_day].as_object().unwrap();
					if data.contains_key(&str_star) {
						let date_str = data[&str_star]["get_star_ts"].as_str().unwrap();

						let naive = NaiveDateTime::from_timestamp(date_str.parse::<i64>().unwrap(), 0);
						let date = DateTime::<Utc>::from_utc(naive, Utc);
						let date_local = date.with_timezone(&Local);

						let rank = times.iter().position(|&t| t == date).unwrap();
						print!("({}) {:^column_width$} ", rank+1, date_local.format("%b %d %l:%M:%S%P").to_string(), column_width=column_width);
					}
					else {
						print!("    {:^column_width$} ", "-", column_width=column_width);
					}
				}
				else {
					print!("    {:^column_width$} ", "-", column_width=column_width);
				}
			}
			println!("");
		}
		println!(""); // End of day gap
	}
}