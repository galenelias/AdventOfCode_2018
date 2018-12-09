use std::collections::VecDeque;
use regex::Regex;

fn run_game(players: usize, last_marble_points: u64) {
	let mut marbles = VecDeque::new();
	let mut scores = vec![0u64; players];

	marbles.push_back(0);
	for marble_number in 1..=last_marble_points {
		if marble_number % 23 == 0 {
			for _ in 0..7 {
				let temp = marbles.pop_back().unwrap();
				marbles.push_front(temp);
			}
			let removed_marble = marbles.pop_front().unwrap();
			scores[(marble_number as usize - 1) % players] += marble_number + removed_marble;
		} else {
			for _ in 0..2 {
				let temp = marbles.pop_front().unwrap();
				marbles.push_back(temp);
			}
			marbles.push_front(marble_number);
		}
	}

	println!("Players: {}\tLast Marble: {}\tHigh Score: {}", players, last_marble_points, scores.iter().max().unwrap());
}


pub fn solve(inputs : Vec<String>) {
	let re_input = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();

	for input in inputs {
		let caps = re_input.captures(&input).unwrap();
		let players = caps[1].parse::<usize>().unwrap();
		let last_marble_points = caps[2].parse::<u64>().unwrap();

		run_game(players, last_marble_points);
		run_game(players, last_marble_points * 100);
	}
}