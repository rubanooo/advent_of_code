fn main() {
    let input = include_str!("./input.txt");

    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

fn problem_one(input: &str) -> i32 {
    elf_calorie_iterator(&input).max().unwrap_or(0)
}

fn problem_two(input: &str) -> i32 {
    let mut elf_calories: Vec<i32> = elf_calorie_iterator(&input).collect();

    elf_calories.sort_unstable();

    elf_calories.iter().rev().take(3).sum()
}

fn elf_calorie_iterator(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.split("\n\n").map(|elf_inventory: &str| {
        elf_inventory
            .lines()
            .filter_map(|calories| calories.parse::<i32>().ok())
            .sum()
    })
}
