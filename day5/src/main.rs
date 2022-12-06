use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;

use std::fs;

type Stack = VecDeque<char>;

#[derive(Debug, Clone)]
struct Supply {
    pub state: HashMap<char, Stack>,
}

#[derive(Debug)]
struct Instruction {
    amount: u32,
    src: char,
    dst: char,
}

impl Supply {
    pub fn new(initial_state: HashMap<char, Stack>) -> Supply {
        Supply {
            state: initial_state,
        }
    }

    pub fn pprint(&self) {
        let keys: Vec<char> = (b'1'..=b'9').map(|c| c as char).collect();
        let max_height = self
            .state
            .values()
            .into_iter()
            .map(|s| s.len())
            .max()
            .unwrap();

        for line in 0..max_height {
            for stack in keys.clone() {
                let stack = self.state.get(&stack).unwrap();
                if stack.len() >= (max_height - line) {
                    let idx = line - (max_height - stack.len());
                    print!("[{}] ", stack.get(idx).unwrap());
                } else {
                    print!("    ");
                }
            }
            print!("\n")
        }

        for num in keys {
            print!(" {}  ", num);
        }

        println!("\n\n");
    }

    pub fn modify_state(&mut self, instruction: Instruction) {
        println!("{:?}", instruction);

        for _i in 0..instruction.amount {
            let value = self.state.get_mut(&instruction.src).unwrap().pop_front();
            self.state
                .get_mut(&instruction.dst)
                .unwrap()
                .push_front(value.unwrap());
        }

        self.pprint();
    }

    pub fn modify_state_9001(&mut self, instruction: Instruction) {
        println!("{:?}", instruction);

        let mut moving = VecDeque::new();
        for _i in 0..instruction.amount {
            moving.push_front(
                self.state
                    .get_mut(&instruction.src)
                    .unwrap()
                    .pop_front()
                    .unwrap(),
            );
        }
        moving
            .into_iter()
            .for_each(|m| self.state.get_mut(&instruction.dst).unwrap().push_front(m));

        self.pprint();
    }
}

fn parse_initial_state(input: &str) -> Supply {
    let legal_characters: Vec<char> = (b'A'..=b'Z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect();

    let mut supply: Supply = Supply::new(HashMap::from([
        ('1', VecDeque::new()),
        ('2', VecDeque::new()),
        ('3', VecDeque::new()),
        ('4', VecDeque::new()),
        ('5', VecDeque::new()),
        ('6', VecDeque::new()),
        ('7', VecDeque::new()),
        ('8', VecDeque::new()),
        ('9', VecDeque::new()),
    ]));
    for row in input.split("\r\n") {
        row.chars()
            .into_iter()
            .enumerate()
            .chunks(4)
            .into_iter()
            .for_each(|mut item| {
                let one = item.next().unwrap_or_default();
                let two = item.next().unwrap_or_default();
                let three = item.next().unwrap_or_default();

                let stack_idx = char::from_digit((one.0 / 4) as u32 + 1, 10).unwrap();
                if one.1 == '[' && three.1 == ']' {
                    if legal_characters.contains(&two.1) {
                        match supply.state.get_mut(&stack_idx) {
                            Some(stack) => stack.push_back(two.1),
                            None => println!("Well, this is awkward"),
                        }
                    }
                }
            });
    }

    supply
}

fn parse_instruction(input: &str) -> Instruction {
    let mut words = input.split(" ");
    let _move = words.next();
    let amount = words.next().unwrap();
    let _from = words.next();
    let src = words.next().unwrap().chars().next().unwrap();
    let _to = words.next();
    let dst = words.next().unwrap().chars().next().unwrap();

    Instruction {
        amount: FromStr::from_str(amount).unwrap(),
        src,
        dst,
    }
}

fn main() {
    let content = fs::read_to_string("input").expect("Should have been able to read the file");
    let mut content = content.splitn(2, "\r\n\r\n");
    let mut supply = parse_initial_state(content.next().unwrap());

    supply.pprint();

    for line in content.next().unwrap().split("\r\n") {
        let instruction = parse_instruction(line);
        supply.modify_state(instruction);
    }

    supply.pprint();

    let content = fs::read_to_string("input").expect("Should have been able to read the file");
    let mut content = content.splitn(2, "\r\n\r\n");
    let mut supply = parse_initial_state(content.next().unwrap());

    supply.pprint();

    for line in content.next().unwrap().split("\r\n") {
        let instruction = parse_instruction(line);
        supply.modify_state_9001(instruction);
    }

    supply.pprint();
}
