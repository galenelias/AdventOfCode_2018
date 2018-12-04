use std::collections::HashMap;
use regex::Regex;

struct GuardRecord {
	num: usize,
	naps: Vec<(usize, usize)>,
}

impl GuardRecord {
	fn total_naptime(&self) -> usize {
		self.naps.iter().map(|(sleep, wake)| wake - sleep).sum::<usize>()
	}

	// Return (minute, amount) of the most slept minute for a given guard
	fn most_slept_minute(&self) -> (usize, usize) {
		let mut minutes = vec![0usize; 60];
		for (sleep,wake) in &self.naps {
			for m in *sleep..*wake {
				minutes[m] += 1;
			}
		}
		let max_minute_amt = minutes.iter().max().unwrap();
		minutes.iter().cloned().enumerate().find(|(_i, v)| v == max_minute_amt).unwrap()
	}
}

pub fn solve(mut inputs : Vec<String>) {
	// Sorting lexicographically should put the record in order correctly
	inputs.sort();

	let re_guard = Regex::new(r"Guard #(\d+)").unwrap();
	let re_sleep = Regex::new(r"00:(\d+)] falls asleep").unwrap();
	let re_wake = Regex::new(r"00:(\d+)] wakes up").unwrap();

	let mut guards = HashMap::new();
	let mut guard_num = 0;
	let mut sleep_min = 0;

	for input in inputs {
		if let Some(guard) = &re_guard.captures(&input) {
			guard_num = guard[1].parse::<usize>().unwrap();
		} else if let Some(sleep_cap) = &re_sleep.captures(&input) {
			sleep_min = sleep_cap[1].parse::<usize>().unwrap();
		} else if let Some(wake_cap) = &re_wake.captures(&input) {
			let wake = wake_cap[1].parse::<usize>().unwrap();
			let record = guards.entry(guard_num).or_insert(GuardRecord { num: guard_num, naps: vec![] });
			record.naps.push((sleep_min, wake));
		}
	}

	let part1_guard = guards.values().max_by_key(|guard| guard.total_naptime()).unwrap();
	let part1_guard_max_minute = part1_guard.most_slept_minute();
	println!("Part 1: Guard #{} * minute {} ({} times) = {}", part1_guard.num, part1_guard_max_minute.0, part1_guard_max_minute.1,
		part1_guard.num * part1_guard_max_minute.0);

	let part2_guard = guards.values().max_by_key(|guard| guard.most_slept_minute().1).unwrap();
	let part2_guard_max_minute = part2_guard.most_slept_minute();
	println!("Part 2: Guard #{} * minute {} ({} times) = {}", part2_guard.num, part2_guard_max_minute.0, part2_guard_max_minute.1, 
		part2_guard.num * part2_guard_max_minute.0);
}