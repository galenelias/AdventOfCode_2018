use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

fn run_instr(instruction: &usize, inst_data: &[usize], regs: &mut [usize]) {
	regs[inst_data[3]] = match instruction {
		0 => regs[inst_data[1]] + regs[inst_data[2]], //addr
		1 => regs[inst_data[1]] + inst_data[2], //addi
		2 => regs[inst_data[1]] * regs[inst_data[2]], //mulr
		3 => regs[inst_data[1]] * inst_data[2], //muli
		4 => regs[inst_data[1]] & regs[inst_data[2]], //banr
		5 => regs[inst_data[1]] & inst_data[2], //bani
		6 => regs[inst_data[1]] | regs[inst_data[2]], //borr
		7 => regs[inst_data[1]] | inst_data[2], //bori
		8 => regs[inst_data[1]], //setr
		9 => inst_data[1], //seti
		10 => if inst_data[1] > regs[inst_data[2]] { 1 } else { 0 }, //gtir
		11 => if regs[inst_data[1]] > inst_data[2] { 1 } else { 0 }, //gtri
		12 => if regs[inst_data[1]] > regs[inst_data[2]] { 1 } else { 0 }, //gtrr
		13 => if inst_data[1] == regs[inst_data[2]] { 1 } else { 0 }, //eqir
		14 => if regs[inst_data[1]] == inst_data[2] { 1 } else { 0 }, //eqri
		15 => if regs[inst_data[1]] == regs[inst_data[2]] { 1 } else { 0 }, //eqrr
		_ => unreachable!(),
	};
}

pub fn solve(inputs : Vec<String>) {
	let re_before = Regex::new(r"Before: \[(.*)\]").unwrap();
	let re_after = Regex::new(r"After:  \[(.*)\]").unwrap();

	// Instruction grid mapping. Each row is the input instruction, each column is which internal instructions it could map to
	let mut inst_grid = vec![vec![true; 16]; 16]; 
	let mut input_iter = inputs.iter();
	let mut part1 = 0;

	loop {
		let caps_before = re_before.captures(input_iter.next().unwrap());
		if caps_before.is_none() {
			break;
		}

		let caps_before = caps_before.unwrap();
		let instr = input_iter.next().unwrap().split(' ').map(|ch| ch.parse::<usize>().unwrap()).collect_vec();
		let caps_after = re_after.captures(input_iter.next().unwrap()).unwrap();
		input_iter.next();  //Blank line between examples

		let regs_before = caps_before[1].split(", ").map(|ch| ch.parse::<usize>().unwrap()).collect_vec();
		let regs_after = caps_after[1].split(", ").map(|ch| ch.parse::<usize>().unwrap()).collect_vec();

		let mut valid = 0;
		for i in 0..16 {
			let mut sim_regs = regs_before.clone();
			run_instr(&i, &instr, &mut sim_regs);
			if sim_regs == regs_after {
				valid += 1;
			}
			inst_grid[instr[0]][i] = inst_grid[instr[0]][i] && (sim_regs == regs_after);
		}
		if valid >= 3 {
			part1 += 1;
		}
	}

	// Generate the mapping of input instructions to internal instructions by finding the most constrained grid entry. Rinse, repeat
	let mut inst_map = HashMap::new();
	while inst_map.len() < 16 {
		let (i, inst) = inst_grid.iter().cloned().enumerate().filter(|(_, row)| row.iter().filter(|&b| *b).count() == 1).next().unwrap();
		let (j, _res) = inst.iter().cloned().enumerate().find(|&(_i, b)| b).unwrap();
		println!("{} -> {}", i, j);
		inst_map.insert(i, j);

		for row in &mut inst_grid {
			row[j] = false;
		}
	}

	let mut regs = vec![0; 4];
	for line in input_iter {
		if line.len() == 0 {
			continue;
		}
		let instr = line.split(' ').map(|ch| ch.parse::<usize>().unwrap()).collect_vec();
		run_instr(inst_map.get(&instr[0]).unwrap(), &instr, &mut regs);
	}

	println!("Part 1: {}", part1);
	println!("Part 2: {} ({:?})", regs[0], regs);
}