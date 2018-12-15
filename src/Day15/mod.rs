use std::collections::{HashSet, VecDeque};
use itertools::Itertools;
use std::cell::RefCell;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Race {
    Elf, Goblin
}

#[derive(Debug)]
struct Entity {
    race: Race,
    pos: (usize, usize),
    health: i32,
}

// Returns a matrix of distances, for each reachable square, plus the first step in the shortest route to get to that square
fn build_distances(pos: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<Vec<(usize, (usize, usize))>> {
	let mut queue = VecDeque::new();

	let mut visited = HashSet::new();
	let mut distances = vec![vec![(0, (0, 0)); grid[0].len()]; grid.len()];

	// Start navigation, recording the intial step in our results so we can quickly resolve the next step
	// We need to make sure we start navigation in reading order for the BFS to yield optimal results
	queue.push_back(((pos.0 - 1, pos.1), 1, (pos.0 - 1, pos.1)));
	queue.push_back(((pos.0, pos.1 - 1), 1, (pos.0, pos.1 - 1)));
	queue.push_back(((pos.0, pos.1 + 1), 1, (pos.0, pos.1 + 1)));
	queue.push_back(((pos.0 + 1, pos.1), 1, (pos.0 + 1, pos.1)));
	visited.insert(pos);

	while !queue.is_empty() {
		let (p, d, fs) = queue.pop_front().unwrap();
		let ch = grid[p.0][p.1];
		if ch == '#' {
			continue;
		}
		if !visited.insert(p) {
			continue;
		}
		distances[p.0][p.1] = (d, fs);

		// Record distances to other entities, but don't move past them
		if ch == 'G' || ch == 'E' {
			continue;
		}

		queue.push_back(((p.0 - 1, p.1), d + 1, fs));
		queue.push_back(((p.0 + 1, p.1), d + 1, fs));
		queue.push_back(((p.0, p.1 - 1), d + 1, fs));
		queue.push_back(((p.0, p.1 + 1), d + 1, fs));
	}
	distances
}

fn sub_solve(inputs : Vec<String>, elf_attack: i32, allow_elf_deaths: bool) -> Option<(Race, i32, i32)> {
    let mut grid = inputs.iter().map(|line| line.chars().collect_vec()).collect_vec();

    let mut entities = Vec::new();

    // Pull out all entities
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'G' || ch == 'E' {
                let race = if ch == 'G' { Race::Goblin } else { Race::Elf };
                entities.push(RefCell::new(Entity{ race: race, pos: (r, c), health: 200}));
            }
        }
    }

	let mut iterations = 0;
    loop {
		iterations += 1;
		// println!("--- Round #{} ---", iterations);

		entities.sort_by_key(|entity| entity.borrow().pos);

		// for entity in &entities {
		// 	let entity = entity.borrow();
		// 	println!("{:?}", entity);
		// }

        let mut activity = false;
        let mut no_targets = false;

		// println!("{}", grid.iter().map(|line| line.iter().collect::<String>()).join("\n"));

        for (i, entity) in entities.iter().enumerate() {
			let mut entity = entity.borrow_mut();

			if entity.health <= 0 {
				continue;
			}

			// println!("Turn {}, Entity {:?}:", iterations, entity);

			let distances = build_distances(entity.pos, &grid);

			let mut target = None;
			let mut target_dist : Option<(usize, (usize, usize))> = None;
			let mut target_val : Option<(i32, (usize, usize))> = None; // (Health, pos), for prioritization

			let no_one_to_fight = !entities.iter().enumerate().any(|(j, other)| i != j && other.borrow().health > 0 && other.borrow().race != entity.race);
			if no_one_to_fight {
				// println!("No targets");
				no_targets = true;
			}

			for (j, other_rc) in entities.iter().enumerate() {
				if i == j {
					continue;
				}
				let other = other_rc.borrow();
				if other.health <= 0 {
					continue;
				}
				let dist = distances[other.pos.0][other.pos.1];

				if entity.race == other.race || dist.0 == 0 {
					continue;
				}

				// println!("Considering {:?}: {:?}", other, dist);
				// if target_dist.is_some() && target_dist.unwrap() < dist {
				// 	println!("Too far");
				// 	continue;
				// }
				let other_val = (other.health, other.pos);
				// if target_dist.is_some() && target_dist.unwrap().0 == 1 && dist.0 == 1 &&  {
				// 	println!("Too low value");
				// 	continue;
				// }

				if target_dist.is_some() {
					let target_dist = target_dist.unwrap();
					let target_val = target_val.unwrap();
					if target_dist.0 == 1 && dist.0 == 1 {
						if target_val < other_val {
							// println!("Too low value");
							continue;
						}
					} else if target_dist < dist {
						// println!("Too far");
						continue;
					}
				}

				target_dist = Some(dist);
				target_val = Some(other_val);
				target = Some(other_rc);
			}

			if target.is_none() {
				continue;
			}
			activity = true;

			let mut target = target.unwrap().borrow_mut();
			let target_dist = target_dist.unwrap();

			// println!("Target: {:?}, {:?}", target, target_dist, );

			// If we're more than one square away, our turn is moving, otherwise combat
			if target_dist.0 > 1 {
				// println!("Moved from {:?} -> {:?}", entity.pos, target_dist.1);
				grid[(target_dist.1).0][(target_dist.1).1] = grid[entity.pos.0][entity.pos.1];
				grid[entity.pos.0][entity.pos.1] = '.';
				entity.pos = target_dist.1;
			}

			if target_dist.0 <= 2 {
				//Combat!
				// println!("Whack!! {} -> {}", target.health, target.health - 3);
				if entity.race == Race::Elf {
					target.health -= elf_attack;
				} else {
					target.health -= 3;
				}

				if target.health <= 0 {
					if !allow_elf_deaths && target.race == Race::Elf {
						return None;
						// println!("Elf death in round {}: {:?}", iterations, target);
						// return false;
					}
					// println!("{:?} death in round {}: {:?}", target.race, iterations, target);
					grid[target.pos.0][target.pos.1] = '.';
				}
			}
			// println!("");
		}

		entities = entities.into_iter().filter(|entity| entity.borrow().health > 0).collect_vec();

		if !activity || no_targets {
			println!("No activity detected after {} turns ({}, {})", iterations, !activity, no_targets);
			break;
		}
	}

	let sum_health = entities.iter().map(|entity| entity.borrow().health).sum::<i32>();
	return Some((entities[0].borrow().race.clone(), iterations - 1, sum_health));
}

pub fn solve(inputs : Vec<String>) {

	// Part 1:
	let (part1_race, part1_rounds, part1_healths) = sub_solve(inputs.clone(), 3, true /*allow_elf_deaths*/).unwrap();
	println!("Part 1: {} ({:?} won in {} rounds with {} health remaining)", part1_rounds * part1_healths, part1_race, part1_rounds, part1_healths);

	// Part 2
	for elf_attack in 4.. {
		if let Some((part2_race, part2_rounds, part2_healths)) = sub_solve(inputs.clone(), elf_attack, false /*allow_elf_deaths*/) {
			println!("Part 2: {} ({:?} won in {} rounds with {} health remaining, using attack of {})", part2_rounds * part2_healths, part2_race, part2_rounds, part2_healths, elf_attack);
			break;
		}
	}

}