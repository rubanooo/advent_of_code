use instruction::Instruction;
use position::Position;
use position_with_aim::PositionWithAim;

mod direction;
mod instruction;
mod position;
mod position_with_aim;

fn main() {
    let input = std::fs::read_to_string("./2021/day_2/input.txt").unwrap();

    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

fn problem_one(input: &str) -> i32 {
    let mut position = Position::default();

    parse_instructions(input).for_each(|instruction| {
        position.move_to(instruction);
    });

    return position.depth * position.horizontal;
}

fn problem_two(input: &str) -> i32 {
    let mut position = PositionWithAim::default();

    parse_instructions(input).for_each(|instruction| {
        position.move_to(instruction);
    });

    return position.depth * position.horizontal;
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input
        .split("\n")
        .filter_map(|instruction| instruction.parse::<Instruction>().ok())
}
