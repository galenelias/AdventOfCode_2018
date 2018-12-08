use itertools::Itertools;
use std::collections::{HashMap, HashSet, BinaryHeap};
use regex::Regex;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
struct Worker {
	task: char,
	end_time: usize,
}

impl Ord for Worker {
	fn cmp(&self, other: &Self) -> Ordering {
		self.end_time.cmp(&other.end_time).reverse()
	}
}

impl PartialOrd for Worker {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}


fn solve_steps(part: usize, steps: &Vec<char>, req_map : &HashMap<char, Vec<char>>, total_workers: usize, work_cost_fn : &Fn(&char) -> usize) {
	let mut done = HashMap::new();

	let mut result = Vec::new();
	let mut active_workers : BinaryHeap<Worker> = BinaryHeap::new();
	let mut time = 0;

	while result.len() != steps.len() {
		time += 1;

		// Find any workers who are now done (active_workers is sorted by end_time)
		while !active_workers.is_empty() && active_workers.peek().unwrap().end_time == time {
			let worker = active_workers.pop().unwrap();
			done.insert(worker.task, true);
			result.push(worker.task);
		}

		if active_workers.len() == total_workers {
			continue;
		}

		for step in steps {
			if *done.get(step).unwrap_or(&false) || active_workers.iter().any(|worker| &worker.task == step) {
				continue;
			}

			let empty = vec![];
			let reqs = req_map.get(step).unwrap_or(&empty);

			// If our requirements are all completed, find a worker to start working on it
			if reqs.iter().all(|r| *done.get(r).unwrap_or(&false)) {
				active_workers.push(Worker{task: *step, end_time: time + work_cost_fn(step)});

				if active_workers.len() == total_workers {
					break;
				}
			}
		}
	}

	println!("Part {}: {} ({})", part, time - 1, result.iter().collect::<String>());
}

pub fn solve(inputs : Vec<String>) {
	let re_step = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();

	let mut req_map = HashMap::new();
	let mut steps = HashSet::new();
	for input in inputs {
		let caps = &re_step.captures(&input).unwrap();

		let step_a = caps[1].chars().next().unwrap();
		let step_b = caps[2].chars().next().unwrap();

		steps.insert(step_a);
		steps.insert(step_b);

		(*req_map.entry(step_b).or_insert(vec![])).push(step_a);
	}

	let mut steps_sorted = steps.into_iter().collect_vec();
	steps_sorted.sort();

	// Solve part 1 as a special case of part 2.  1 worker, each step takes 1 time unit
	solve_steps(1, &steps_sorted, &req_map, 1 /*total_workers*/, &|_: &char| 1);

	// Solve sample input constraints
	// solve_steps(2, &steps_sorted, &req_map, 2 /*total_workers*/, &|ch: &char| ((*ch as u8) - ('A' as u8) + 1) as usize);

	solve_steps(2, &steps_sorted, &req_map, 5 /*total_workers*/, &|ch: &char| ((*ch as u8) - ('A' as u8) + 61) as usize);
}