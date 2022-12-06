use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");

    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

fn problem_one(input: &str) -> String {
    let (mut ship, instructions) = parse_input(input);

    for instruction in instructions {
        for item in ship.get_crates_for_instruction(&instruction) {
            ship.stacks[instruction.to].push(item);
        }
    }

    ship.message()
}

fn problem_two(input: &str) -> String {
    let (mut ship, instructions) = parse_input(input);

    for instruction in instructions {
        for item in ship.get_crates_for_instruction(&instruction).iter().rev() {
            ship.stacks[instruction.to].push(item.clone());
        }
    }

    ship.message()
}

fn parse_input(input: &str) -> (Ship, Vec<Instruction>) {
    let (stacks, instructions) = input.split_once("\n\n").expect("Invalid input");

    let ship = Ship::from(stacks);
    let instructions: Vec<Instruction> = instructions.lines().map(Instruction::from).collect();

    (ship, instructions)
}

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (amount, from, to) = value
            .split(" ")
            .filter_map(|s| s.parse::<usize>().ok())
            .collect_tuple::<(usize, usize, usize)>()
            .expect("Failed to parse");

        Self {
            amount,
            from: from - 1,
            to: to - 1,
        }
    }
}

#[derive(Debug)]
struct Ship {
    stacks: Vec<Vec<char>>,
}

impl Ship {
    fn new(size: usize) -> Self {
        Self {
            stacks: vec![Vec::default(); size],
        }
    }

    fn get_crates_for_instruction(&mut self, instruction: &Instruction) -> Vec<char> {
        (0..instruction.amount)
            .into_iter()
            .filter_map(|_| self.stacks[instruction.from].pop())
            .collect()
    }

    fn message(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack[stack.len() - 1])
            .collect()
    }
}

impl From<&str> for Ship {
    fn from(value: &str) -> Self {
        let mut input = value.lines().rev();

        // get the number of stacks from the stack line
        let num_stacks = input
            .next()
            .expect("Missing stack count line")
            .split(" ")
            .filter_map(|s| s.parse::<usize>().ok())
            .count();

        let mut ship = Self::new(num_stacks);

        input
            .map(|line| line.chars().skip(1).step_by(4).collect::<Vec<char>>())
            .for_each(|line| {
                line.into_iter()
                    .enumerate()
                    .filter(|(_, char)| !char.is_whitespace())
                    .for_each(|(index, element)| {
                        ship.stacks[index].push(element);
                    });
            });

        ship
    }
}
