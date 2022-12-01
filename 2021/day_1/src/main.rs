#![feature(array_windows)]

fn main() {
    let input = std::fs::read_to_string("./2021/day_1/input.txt").unwrap();

    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

fn problem_one(input: &str) -> usize {
    submarine_depths(input)
        .array_windows()
        .filter(|[a, b]| a < b)
        .count()
}

fn problem_two(input: &str) -> usize {
    submarine_depths(input)
        .array_windows()
        .filter(|[a, _, _, d]| a < d)
        .count()
}

fn submarine_depths(input: &str) -> Vec<i32> {
    input
        .split("\n")
        .filter_map(|depth| depth.parse::<i32>().ok())
        .collect::<Vec<i32>>()
}
