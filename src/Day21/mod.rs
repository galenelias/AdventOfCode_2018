use itertools::Itertools;
use std::collections::HashSet;

struct Instruction {
	opcode: String,
	input: Vec<usize>,
}

fn run_instr(inst: &Instruction, regs: &mut [usize]) {
	regs[inst.input[2]] = match inst.opcode.as_ref() {
		"addr" => regs[inst.input[0]] + regs[inst.input[1]], //addr
		"addi" => regs[inst.input[0]] + inst.input[1], //addi
		"mulr" => regs[inst.input[0]] * regs[inst.input[1]], //mulr
		"muli" => regs[inst.input[0]] * inst.input[1], //muli
		"banr" => regs[inst.input[0]] & regs[inst.input[1]], //banr
		"bani" => regs[inst.input[0]] & inst.input[1], //bani
		"borr" => regs[inst.input[0]] | regs[inst.input[1]], //borr
		"bori" => regs[inst.input[0]] | inst.input[1], //bori
		"setr" => regs[inst.input[0]], //setr
		"seti" => inst.input[0], //seti
		"gtir" => if inst.input[0] > regs[inst.input[1]] { 1 } else { 0 }, //gtir
		"gtri" => if regs[inst.input[0]] > inst.input[1] { 1 } else { 0 }, //gtri
		"gtrr" => if regs[inst.input[0]] > regs[inst.input[1]] { 1 } else { 0 }, //gtrr
		"eqir" => if inst.input[0] == regs[inst.input[1]] { 1 } else { 0 }, //eqir
		"eqri" => if regs[inst.input[0]] == inst.input[1] { 1 } else { 0 }, //eqri
		"eqrr" => if regs[inst.input[0]] == regs[inst.input[1]] { 1 } else { 0 }, //eqrr
		_ => unreachable!(),
	};
}

pub fn solve(inputs : Vec<String>) {
	let ip_reg = inputs[0].split(" ").skip(1).next().unwrap().parse::<usize>().unwrap();
	let inputs = inputs.iter().skip(1).map(|line| line.split(" ").collect_vec()).collect_vec();

	let program = inputs.iter().map(|line| {
		let opcode = line[0].to_string();
		let input = line.iter().skip(1).map(|w| w.parse::<usize>().unwrap()).collect_vec();
		Instruction{ opcode, input}
	}).collect_vec();

	let mut regs = vec![0usize; 6];
	let mut seen = HashSet::new();
	let mut first_value = None;
	let mut last_value = 0;

	while regs[ip_reg] < inputs.len() {
		if regs[ip_reg] == 28 {
			let data_reg = regs[program[regs[ip_reg]].input[0]];
			if !seen.insert(data_reg) {
				break;
			}

			first_value.get_or_insert(data_reg);
			last_value = data_reg;
		}
		run_instr(&program[regs[ip_reg]], &mut regs);
		regs[ip_reg] += 1;
	}

	println!("Part 1: {}", first_value.unwrap());
	println!("Part 2: {}", last_value);
}