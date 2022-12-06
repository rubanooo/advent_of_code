use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");

    println!("Problem one: {}", calculate_start_of_packet(input, 4));
    println!("Problem two: {}", calculate_start_of_packet(input, 14));
}

fn calculate_start_of_packet(input: &str, window_size: usize) -> usize {
    let index = input
        .as_bytes()
        .windows(window_size)
        .position(|window| window.iter().collect::<HashSet<&u8>>().len() == window_size)
        .expect("No valid position found");

    index + window_size
}
