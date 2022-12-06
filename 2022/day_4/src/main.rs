fn main() {
    let input = include_str!("./input.txt");

    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

fn problem_one(input: &str) -> usize {
    parse_input(input)
        .filter(|(left, right)| left.contains_range(right) || right.contains_range(left))
        .count()
}

fn problem_two(input: &str) -> usize {
    parse_input(input)
        .filter(|(left, right)| left.has_overlap(right) || right.has_overlap(left))
        .count()
}

fn parse_input(input: &str) -> impl Iterator<Item = (Range, Range)> + '_ {
    input.lines().filter_map(|pairs| {
        let (left, right) = pairs.split_once(",")?;

        Some((Range::from(left), Range::from(right)))
    })
}

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains_range(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn has_overlap(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (start, end) = value.split_once("-").expect("Invalid range");

        Self {
            start: start.parse::<u64>().expect("Invalid start of range"),
            end: end.parse::<u64>().expect("Invalid end of range"),
        }
    }
}
