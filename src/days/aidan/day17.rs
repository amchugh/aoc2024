use crate::days::Solution;

#[derive(Debug)]
struct Machine {
    a: i64,
    b: i64,
    c: i64,
    pc: usize,
    outputs: Vec<u8>,
}

impl Machine {
    fn new() -> Machine {
        Machine { a: 0, b: 0, c: 0, pc: 0, outputs: vec![] }
    }

    fn combo(&self, operand: &u8) -> i64 {
        match operand {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => { panic!("Should never see 7 as a combo operand"); }
            i => *i as i64,
        }
    }

    fn run(&mut self, data: &Vec<u8>) {
        loop {
            // Get the instruction
            let instruction = data.get(self.pc);
            if instruction.is_none() {
                break;
            }
            let instruction = instruction.unwrap();
            let operand = data.get(self.pc + 1).unwrap();

            match instruction {
                0 => {
                    // adv
                    let numerator = self.a;
                    let denominator = 2_i64.pow(self.combo(operand).try_into().unwrap());
                    self.a = numerator / denominator;
                    self.pc += 2;
                }
                1 => {
                    // bxl
                    self.b = self.b ^ (*operand as i64);
                    self.pc += 2;
                }
                2 => {
                    // bst
                    self.b = self.combo(operand) % 8;
                    self.pc += 2;
                }
                3 => {
                    // jnz
                    if self.a != 0 {
                        self.pc = *operand as usize;
                    } else {
                        self.pc += 2;
                    }
                }
                4 => {
                    // bxc
                    self.b = self.b ^ self.c;
                    self.pc += 2;
                }
                5 => {
                    // out
                    let output = self.combo(operand) % 8;
                    self.outputs.push(output as u8);
                    self.pc += 2;
                }
                6 => {
                    // bdv
                    let numerator = self.a;
                    let denominator = 2_i64.pow(self.combo(operand).try_into().unwrap());
                    self.b = numerator / denominator;
                    self.pc += 2;
                }
                7 => {
                    // cdv
                    let numerator = self.a;
                    let denominator = 2_i64.pow(self.combo(operand).try_into().unwrap());
                    self.c = numerator / denominator;
                    self.pc += 2;
                }
                _ => { panic!("Invalid instruction"); }
            }
        }
    }
}

#[derive(Debug)]
pub struct Day17 {
    // State generated by `parse_input`
    data: Vec<u8>,
    initial_values: (i64, i64, i64),
}

impl Day17 {
    // Needed for creating a blank day
    pub fn new() -> Day17 {
        Day17 { data: vec![], initial_values: (0, 0, 0) }
    }
}

impl Solution for Day17 {
    fn reset(&mut self) {
        // Should probably do the same thing new() does.
        self.data = vec![];
    }

    fn parse_input(&mut self, file_contents: &str) {
        let mut parts = file_contents.split("\n\n");
        let mut registers = parts.next().unwrap().split("\n");
        let a = registers.next().unwrap().strip_prefix("Register A: ").unwrap().parse().unwrap();
        let b = registers.next().unwrap().strip_prefix("Register B: ").unwrap().parse().unwrap();
        let c = registers.next().unwrap().strip_prefix("Register C: ").unwrap().parse().unwrap();
        self.initial_values = (a, b, c);

        let operations = parts.next().unwrap();
        let operations = operations.strip_prefix("Program: ").unwrap();
        self.data = operations.split(",").map(|x| x.parse::<u8>().unwrap()).collect();
    }

    fn part1(&self) -> String {
        let mut machine = Machine {
            a: self.initial_values.0,
            b: self.initial_values.1,
            c: self.initial_values.2,
            pc: 0,
            outputs: vec![],
        };

        machine.run(&self.data);

        let mut building = String::new();
        let len = machine.outputs.len();
        for (i, o) in machine.outputs.iter().enumerate() {
            if i < len - 1 {
                building = format!("{}{},", building, o);
            }
            else {
                building = format!("{}{}", building, o);
            }
        }
        building
    }

    #[allow(unreachable_code)]
    fn part2(&self) -> String {
        return "Not Implemented".to_string();

        // So, the program ends with jmp-ing back to 0
        assert!(self.data[self.data.len() - 2] == 3); // jnz
        assert!(self.data[self.data.len() - 1] == 0); // 0
        // Before that, it prints (in this base, register B)
        assert!(self.data[self.data.len() - 4] == 5); // out
        let printing_register = self.data[self.data.len() - 3];
        println!("Using register at combo {}", printing_register);
        // And before that, it divides a by 8
        assert!(self.data[self.data.len() - 6] == 0); // adv
        assert!(self.data[self.data.len() - 5] == 3); // 8 = 2.pow(3)

        // Otherwise...
        for i in self.data.as_slice()[0..self.data.len()-6].iter().step_by(2) {
            assert!(*i != 0); // It doesn't modify a through 'adv'
            assert!(*i != 3); // It doesn't jump
            assert!(*i != 5); // It doesn't print
        }

        // Also, everything can fit in a single i64.
        assert!(8_u64.pow(self.data.len() as u32) < 2_u64.pow(63) - 1);

        // There's only one way to make the biggest number,
        // and we know what scale we need to operate on.
        let mut total = 0;
        // for idx in 0..self.data.len() {
        //     let invidx = self.data.len() - idx - 1;

        //     let target = dbg!(self.data[invidx]);
        //     for i in 0..8 {
        //         let candidate = i * 8_i64.pow(invidx as u32);

        //         let mut machine = Machine::new();
        //         machine.a = total + candidate;
        //         machine.run(&self.data);

        //         if machine.outputs.len() != self.data.len() {
        //             continue;
        //         }

        //         let result = machine.outputs.get(invidx);
        //         if let Some(result) = result {
        //             if *result == target {
        //                 total += candidate;
        //                 dbg!(total);
        //                 break;
        //             }
        //         }
        //     }
        // }

        // let idxs = (0..self.data.len()).rev().collect::<Vec<usize>>();
        // for target_digit in idxs.windows(2) {
        //     dbg!(target_digit);
        //     let first_target = self.data[target_digit[1]];
        //     let second_target = self.data[target_digit[0]];
        //     let mut flag = false;
        //     for i in 0..64 {
        //         let candidate = i * 8_i64.pow(target_digit[1] as u32);
        //         let mut machine = Machine::new();
        //         machine.a = total + candidate;
        //         machine.run(&self.data);
        //         let first = machine.outputs.get(target_digit[1]).unwrap_or(&8);
        //         if let Some(second) = machine.outputs.get(target_digit[0]) {
        //             if *first == first_target && *second == second_target {
        //                 total = 8 * total + (i / 8) * 8_i64.pow(target_digit[1] as u32);
        //                 flag = true;
        //                 break;
        //             }
        //         }
        //     }
        //     if !flag {
        //         let mut machine = Machine::new();
        //         machine.a = total;
        //         machine.run(&self.data);
        //         dbg!(machine.outputs);
        //     }
        //     assert!(flag)
        // }

        // Ensure we got the right one?
        let mut machine = Machine::new();
        machine.a = total;
        machine.run(&self.data);
        dbg!(machine.outputs.len(), self.data.len());
        assert!(dbg!(machine.outputs) == *dbg!(&self.data));

        total.to_string()
    }
}

