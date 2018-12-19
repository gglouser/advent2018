use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Square {
    Open,
    Trees,
    Lumberyard,
}

impl From<char> for Square {
    fn from(c: char) -> Square {
        match c {
            '|' => Square::Trees,
            '#' => Square::Lumberyard,
            _   => Square::Open,
        }
    }
}

impl From<Square> for char {
    fn from(s: Square) -> char {
        match s {
            Square::Open => '.',
            Square::Trees => '|',
            Square::Lumberyard => '#',
        }
    }
}

type Grid = Vec<Vec<Square>>;

fn parse_input(s: &str) -> Grid {
    s.lines().map(|line| line.chars().map(|c| Square::from(c)).collect()).collect()
}

fn show_grid(grid: &Grid) -> String {
    grid.iter().map(|row|
        row.iter().map(|&s| char::from(s)).collect::<String>() + "\n"
    ).collect()
}

// Create a table of partial counts.
// Add extra two rows and two columns to simulate border of open squares.
fn partial_counts(grid: &Grid, kind: Square) -> Vec<Vec<usize>> {
    let row_len = grid[0].len();
    let mut p = vec![vec![0; row_len+3]; grid.len()+2];
    for row in 0..grid.len() {
        for col in 0..row_len {
            let here = (grid[row][col] == kind) as usize;
            p[row+2][col+2] = here + p[row+1][col+2] + p[row+2][col+1] - p[row+1][col+1];
        }
        p[row+2][row_len+2] = p[row+2][row_len+1];
    }
    let last_row = p[p.len()-1].clone();
    p.push(last_row);
    p
}

fn get_count(partials: &Vec<Vec<usize>>, r: usize, c: usize, h: usize, w: usize) -> usize {
    partials[r+h][c+w] + partials[r][c] - partials[r+h][c] - partials[r][c+w]
}

fn step(grid: &Grid) -> Grid {
    let tree_counts = partial_counts(&grid, Square::Trees);
    let yard_counts = partial_counts(&grid, Square::Lumberyard);
    let mut next = grid.clone();
    let row_len = grid[0].len();
    for row in 0..grid.len() {
        for col in 0..row_len {
            let trees = get_count(&tree_counts, row, col, 3, 3);
            let yards = get_count(&yard_counts, row, col, 3, 3);
            match grid[row][col] {
                Square::Open => if trees >= 3 { next[row][col] = Square::Trees; },
                Square::Trees => if yards >= 3 { next[row][col] = Square::Lumberyard; },
                Square::Lumberyard => if trees < 1 || yards < 2 { next[row][col] = Square::Open; },
            }
        }
    }

    next
}

fn iterate(initial_state: &Grid, minutes: usize) -> Grid {
    let mut state = initial_state.clone();
    for _t in 0..minutes {
        state = step(&state);
    }
    state
}

fn iterate_long(initial_state: &Grid, minutes: usize) -> Grid {
    let mut state = initial_state.clone();
    let mut state_mem = HashMap::new();
    state_mem.insert(state.clone(), 0);
    for t in 1.. {
        state = step(&state);
        if state_mem.contains_key(&state) {
            let t0 = *state_mem.get(&state).unwrap();
            // println!("found repeat, {} == {}", t0, t);
            let period = t - t0;
            let tn = (minutes - t0) % period + t0;
            // println!("t={} is the same as t={}", minutes, tn);
            return state_mem.drain().find(|&(_,tk)| tk == tn).unwrap().0;
        } else {
            state_mem.insert(state.clone(), t);
        }
    }
    unreachable!()
}

fn resource_value(grid: &Grid) -> usize {
    let trees = grid.iter().flat_map(|row| row.iter()).filter(|&&s| s == Square::Trees).count();
    let yards = grid.iter().flat_map(|row| row.iter()).filter(|&&s| s == Square::Lumberyard).count();
    trees * yards
}

fn solve(input: &str) -> (usize, usize) {
    let initial_grid = parse_input(input);
    const SHOW_STATES: bool = false;
    if SHOW_STATES {
        println!("Initial state:\n{}", show_grid(&initial_grid));
    }
    let state1 = iterate(&initial_grid, 10);
    let state2 = iterate_long(&initial_grid, 1_000_000_000);
    if SHOW_STATES {
        println!("Final state:\n{}", show_grid(&state2));
    }

    (resource_value(&state1), resource_value(&state2))
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the solution to part 1 is {}", part1);
    println!("the solution to part 2 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE : &'static str = "\
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
";

    #[test]
    fn example() {
        let (part1, part2) = solve(EXAMPLE);
        assert_eq!(1147, part1);
        assert_eq!(0, part2);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day18.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day18.txt"),
                   format!("{:?}", x));
    }
}
