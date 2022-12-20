use std::ops::Range;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let forest = create_forest_map(INPUT);

    println!("Problem one: {}", problem_one(&forest));
    println!("Problem two: {}", problem_two(&forest));
}

fn problem_one(forest: &Forest) -> usize {
    forest
        .iter()
        .flatten()
        .filter(|tree| tree.is_visible_from_outside(&forest))
        .count()
}

fn problem_two(forest: &Forest) -> usize {
    forest
        .iter()
        .flatten()
        .map(|tree| tree.scenic_score(&forest))
        .max()
        .unwrap_or(0)
}

type Forest = Vec<Vec<Tree>>;

enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

impl Direction {
    fn collect() -> [Self; 4] {
        [Self::Left, Self::Right, Self::Top, Self::Bottom]
    }
}

struct Tree {
    height: u32,
    x: usize,
    y: usize,
}

impl Tree {
    fn new(height: u32, x: usize, y: usize) -> Self {
        Self { height, x, y }
    }

    fn viewing_distance(&self, forest: &Forest, side: &Direction) -> usize {
        let mut range = self.get_range_of_trees(forest, side);
        let amount_of_trees = range.len();

        let distance = match side {
            Direction::Left => range
                .rev()
                .position(|idx| forest[self.y][idx].height >= self.height),
            Direction::Right => range.position(|idx| forest[self.y][idx].height >= self.height),
            Direction::Top => range
                .rev()
                .position(|idx| forest[idx][self.x].height >= self.height),
            Direction::Bottom => range.position(|idx| forest[idx][self.x].height >= self.height),
        };

        // when no tree is found, the distance is the amount of trees in the direction
        match distance {
            Some(distance) => distance + 1,
            None => amount_of_trees,
        }
    }

    /// The scenic score of a tree is the product of the viewing distances from each side.
    fn scenic_score(&self, forest: &Forest) -> usize {
        Direction::collect()
            .iter()
            .map(|side| self.viewing_distance(forest, side))
            .product()
    }

    /// A tree is visible from a side if all the trees in the range of that side are shorter.
    fn is_visible_from_side(&self, forest: &Forest, side: &Direction) -> bool {
        let mut range = self.get_range_of_trees(forest, side);

        match side {
            Direction::Left => range.all(|idx| forest[self.y][idx].height < self.height),
            Direction::Right => range.all(|idx| forest[self.y][idx].height < self.height),
            Direction::Top => range.all(|idx| forest[idx][self.x].height < self.height),
            Direction::Bottom => range.all(|idx| forest[idx][self.x].height < self.height),
        }
    }

    /// Get the range of trees in the forest for a given side.
    fn get_range_of_trees(&self, forest: &Forest, side: &Direction) -> Range<usize> {
        match side {
            Direction::Left => 0..self.x,
            Direction::Right => (self.x + 1)..forest[self.y].len(),
            Direction::Top => 0..self.y,
            Direction::Bottom => (self.y + 1)..forest.len(),
        }
    }

    /// A tree is visible from outside if it is visible from any side.
    fn is_visible_from_outside(&self, forest: &Forest) -> bool {
        Direction::collect()
            .iter()
            .any(|side| self.is_visible_from_side(forest, side))
    }
}

fn create_forest_map(input: &str) -> Forest {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, height)| Tree::new(height.to_digit(10).unwrap(), x, y))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
