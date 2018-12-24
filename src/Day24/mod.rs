use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ImmuneType {
	ImmuneSystem,
	Infection,	
}

#[derive(Debug, Clone)]
struct Group {
	id: usize,
	category: ImmuneType,
	units: i64,
	hp: i64,
	attack: i64,
	attack_type: String,
	initiative: i64,
	immunities: Vec<String>,
	weaknesses: Vec<String>,
}

impl Group {
	fn damage_to(&self, other : &Group) -> i64 {
		if self.category == other.category || other.hp <= 0 || other.immunities.contains(&self.attack_type) {
			0
		} else if other.weaknesses.contains(&self.attack_type) {
			2 * self.attack
		} else {
			self.attack
		}
	}

	fn effective_power(&self) -> i64 {
		self.units * self.attack
	}

	fn take_damage(&mut self, damage : i64) -> i64 {
		let deaths = std::cmp::min(self.units, damage / self.hp);
		self.units -= deaths;
		deaths
	}
}

fn simulate(mut entities: Vec<RefCell<Group>>, boost: i64) -> Option<(ImmuneType, i64)> {
	for ent in &entities {
		let mut ent = ent.borrow_mut();
		if ent.category == ImmuneType::ImmuneSystem {
			ent.attack += boost;
		}
	}

	loop {
		entities.sort_by_key(|ent| (ent.borrow().effective_power(), ent.borrow().initiative));
		entities.reverse();

		// Targetting
		let mut targets = HashMap::new();
		for e in &entities {
			let e = e.borrow();
			if e.hp <= 0 {
				continue;
			}

			let mut damages = entities.iter().map(|o| o.borrow()).map(|o| { ((e.damage_to(&o), o.effective_power(), o.initiative), o.id)}).collect_vec();
			damages.sort();
			damages.reverse();

			for ((d, _, _), j) in damages {
				if d > 0 && targets.values().find(|&t| *t == j).is_none() {
					targets.insert(e.id, j);
					break;
				}
			}
		}

		// Attacking
		let mut total_deaths = 0;
		entities.sort_by(|a,b| a.borrow().initiative.cmp(&b.borrow().initiative).reverse());
		for e in &entities {
			let e = e.borrow();
			if let Some(target) = targets.get(&e.id) {
				
				let mut target_ent = entities.iter().find(|o| o.borrow().id == *target).unwrap().borrow_mut();
				let damage = e.units * e.damage_to(&target_ent);
				let deaths = target_ent.take_damage(damage);
				total_deaths += deaths;
			}
		}

		entities = entities.into_iter().filter(|e| e.borrow().units > 0).collect_vec();

		let first_category = entities[0].borrow().category;
		if entities.iter().all(|e| e.borrow().category == first_category) {
			let units_left = entities.iter().map(|e| e.borrow().units).sum::<i64>();
			return Some((entities[0].borrow().category, units_left))	
		} else if total_deaths == 0 {
			return None
		}
	}
	unreachable!();
}

pub fn solve(inputs : Vec<String>) {
	let re_input = Regex::new(r"(\d+) units each with (\d+) hit points (\(.*\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();

	let mut category = ImmuneType::ImmuneSystem;
	let mut entities = Vec::new();

	for (i, input) in inputs.iter().enumerate() {
		if input == "Immune System:" {
			category = ImmuneType::ImmuneSystem;
			continue;
		} else if input == "Infection:" {
			category = ImmuneType::Infection;
			continue;
		} else if input.len() == 0 {
			continue;
		}

		let caps = re_input.captures(&input).unwrap();

		let units = caps[1].parse::<i64>().unwrap();
		let hp = caps[2].parse::<i64>().unwrap();
		let attack = caps[4].parse::<i64>().unwrap();
		let attack_type = caps[5].to_string();
		let initiative = caps[6].parse::<i64>().unwrap();
		let mut weaknesses = vec![];
		let mut immunities = vec![];

		if let Some(extra_info) = caps.get(3) {
			let extra_info = extra_info.as_str().trim().trim_matches('(').trim_matches(')');
			for info in extra_info.split("; ") {
				// Example: 'weak to bludgeoning, slashing'
				let parts = info.split(" to " ).collect_vec();
				let tokens = parts[1].split(", ").map(ToString::to_string).collect_vec();
				if parts[0] == "weak" {
					weaknesses = tokens;
				} else if parts[0] == "immune" {
					immunities = tokens;
				} else {
					unreachable!("Unrecognized extra info");
				}
			}
		}

		entities.push(RefCell::new(Group { id: i, category, units, hp, attack, attack_type, initiative, immunities, weaknesses }));
	}

	let part1 = simulate(entities.clone(), 0);
	println!("Part 1: {}", part1.unwrap().1);

	for boost in 0.. {
		if let Some(part2) = simulate(entities.clone(), boost) {
			if part2.0 == ImmuneType::ImmuneSystem {
				println!("Part 2: {} (boost = {})", part2.1, boost);
				break;
			}
		}
	}
}