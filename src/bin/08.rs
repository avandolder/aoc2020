use std::collections::HashSet;
use std::io::Read;

#[derive(Clone, Debug)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Op {
    fn parse(p: &str) -> Op {
        let sign = match &p[4..=4] {
            "+" => 1,
            "-" => -1,
            _ => panic!(),
        };

        let arg = sign * &p[5..].parse::<i32>().unwrap();

        match &p[0..3] {
            "nop" => Op::Nop(arg),
            "acc" => Op::Acc(arg),
            "jmp" => Op::Jmp(arg),
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
struct VM {
    acc: i32,
    ops: Vec<Op>,
    pc: i32,
}

impl VM {
    fn new(ops: Vec<Op>) -> VM {
        VM {
            acc: 0,
            ops,
            pc: 0,
        }
    }

    fn step(&mut self) {
        match self.ops[self.pc as usize] {
            Op::Nop(_) => self.pc += 1,
            Op::Acc(x) => {
                self.acc += x;
                self.pc += 1;
            }
            Op::Jmp(x) => self.pc += x,
        }
    }
}

fn part1(vm: &mut VM) -> i32 {
    let mut seen = HashSet::new();

    while !seen.contains(&vm.pc) {
        seen.insert(vm.pc);
        vm.step();
    }
    vm.acc
}

fn part2(vm: &mut VM) -> i32 {
    for i in 0..vm.ops.len() {
        let mut vm = vm.clone();
        match vm.ops[i].clone() {
            Op::Nop(x) => vm.ops[i] = Op::Jmp(x),
            Op::Jmp(x) => vm.ops[i] = Op::Nop(x),
            _ => (),
        }

        // Try to walk to end.
        let mut seen = HashSet::new();

        while !seen.contains(&vm.pc) {
            seen.insert(vm.pc);
            vm.step();

            if vm.pc == vm.ops.len() as i32 {
                return vm.acc;
            }
        }
    }
    panic!()
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read from stdin");

    let ops = input.lines().map(Op::parse).collect::<Vec<_>>();
    let mut vm = VM::new(ops);
    println!("{}", part2(&mut vm));
}
