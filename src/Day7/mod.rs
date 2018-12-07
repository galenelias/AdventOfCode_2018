use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use regex::Regex;

fn solve_steps(part: usize, steps: &Vec<char>, req_map : &HashMap<char, Vec<char>>, total_workers: usize, work_cost_fn : &Fn(&char) -> usize) {
	let mut done = HashMap::new();
	let mut in_progress = HashMap::new();

	let mut result = Vec::new();
	let mut workers = vec![(0, 'A'); total_workers];  // Worker is a represented as a tuple of (timeleft, step in progress)
	let mut total_time = 0;

	while result.len() != steps.len() {
		total_time += 1;
		let mut workers_free = 0;

		// Have all workers make progress on their task, and potentially complete it an add it to our results
		for worker in &mut workers {
			if worker.0 == 1 {
				done.insert(worker.1, true);
				result.push(worker.1);
			}
			if worker.0 > 0 {
				worker.0 -= 1;
			}
			if worker.0 == 0 {
				workers_free += 1;
			}
		}

		if workers_free == 0 {
			continue;
		}

		for step in steps {
			if *done.get(step).unwrap_or(&false) || *in_progress.get(step).unwrap_or(&false) {
				continue;
			}

			let empty = vec![];
			let reqs = req_map.get(step).unwrap_or(&empty);

			// If our requirements are all completed, find a worker to start working on it
			if reqs.iter().all(|r| *done.get(r).unwrap_or(&false)) {
				// Start working
				let worker = workers.iter_mut().find(|w| w.0 == 0).unwrap();
				worker.0 = work_cost_fn(step);
				worker.1 = *step;

				in_progress.insert(*step, true);

				workers_free -= 1;
				if workers_free == 0 {
					break;
				}
			}
		}
	}

	println!("Part {}: {} ({})", part, total_time - 1, result.iter().collect::<String>());
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