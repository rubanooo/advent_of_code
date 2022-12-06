#![feature(iter_array_chunks)]

use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");

    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

fn problem_one(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            let compartment_one = left.chars().collect::<HashSet<char>>();
            let compartment_two = right.chars().collect::<HashSet<char>>();

            compartment_one
                .intersection(&compartment_two)
                .map(|element| *element)
                .next()
        })
        .map(calculate_priority)
        .sum::<u64>()
}

fn problem_two(input: &str) -> u64 {
    input
        .lines()
        .array_chunks::<3>()
        .filter_map(|[first, second, third]| {
            let first = first.chars().collect::<HashSet<char>>();
            let second = second.chars().collect::<HashSet<char>>();
            let third = third.chars().collect::<HashSet<char>>();

            first
                .intersection(&second)
                .map(|element| *element)
                .collect::<HashSet<char>>()
                .intersection(&third)
                .map(|element| *element)
                .next()
        })
        .map(calculate_priority)
        .sum::<u64>()
}

fn calculate_priority(input: char) -> u64 {
    if input.is_lowercase() {
        input as u64 - 96
    } else {
        input as u64 - 38
    }
}
