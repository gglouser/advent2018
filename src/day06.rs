#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);

fn parse_input(s: &str) -> Vec<Point> {
    s.lines().map(|line| {
            let mut coords = line.split(", ");
            let x = coords.next().unwrap().parse().unwrap();
            let y = coords.next().unwrap().parse().unwrap();
            Point(x,y)
        }).collect()
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Location {
    owner_id: Option<usize>,
    dist: usize,
    tot_dist: usize,
}

impl Location {
    fn new() -> Self {
        Location { owner_id: None, dist: std::usize::MAX, tot_dist: 0 }
    }

    fn add_dist(&mut self, owner: usize, dist: usize) {
        if dist < self.dist {
            self.owner_id = Some(owner);
            self.dist = dist;
        } else if dist == self.dist {
            self.owner_id = None;
        }
        self.tot_dist += dist;
    }
}

fn solve(input: &str, near_limit: usize) -> (u32, usize) {
    let coords = parse_input(input);

    let min_x = coords.iter().map(|p| p.0).min().unwrap();
    let max_x = coords.iter().map(|p| p.0).max().unwrap();
    let width = (max_x - min_x + 1) as usize;

    let min_y = coords.iter().map(|p| p.1).min().unwrap();
    let max_y = coords.iter().map(|p| p.1).max().unwrap();
    let height = (max_y - min_y + 1) as usize;

    let mut grid = vec![vec![Location::new(); width]; height];
    let mut areas = vec![0; coords.len()];
    let mut finite = vec![true; coords.len()];
    for x in min_x..=max_x {
        let gx = (x - min_x) as usize;
        for y in min_y..=max_y {
            let gy = (y - min_y) as usize;

            coords.iter()
                .map(|Point(px,py)| (px - x).abs() + (py - y).abs())
                .enumerate()
                .for_each(|(o,d)| grid[gy][gx].add_dist(o, d as usize));

            if let Some(o) = grid[gy][gx].owner_id {
                areas[o] += 1;
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    finite[o] = false;
                }
            }
        }
    }

    let max_finite = (0..coords.len()).filter(|&i| finite[i]).map(|i| areas[i]).max().unwrap();

    let near_area = grid.iter().flat_map(|row| row.iter())
        .filter(|loc| loc.tot_dist < near_limit).count();

    (max_finite, near_area)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input, 10000);
    println!("the solution to part 1 is {}", part1);
    println!("the solution to part 2 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE : &'static str = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE),
            vec![
                Point(1, 1),
                Point(1, 6),
                Point(8, 3),
                Point(3, 4),
                Point(5, 5),
                Point(8, 9),
            ]);
    }

    #[test]
    fn example() {
        let (part1, part2) = solve(EXAMPLE, 32);
        assert_eq!(17, part1);
        assert_eq!(16, part2);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day06.txt");
        let x = solve(&input, 10000);
        assert_eq!(include_str!("../outputs/day06.txt"),
                   format!("{:?}", x));
    }
}
