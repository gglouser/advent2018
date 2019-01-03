use regex::Regex;
use std::ops::Add;
use std::cmp::Ordering;
use search::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos(i32, i32, i32);

impl Add for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Pos {
    fn dist(&self, other: &Pos) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Nanobot {
    pos: Pos,
    radius: i32,
}

fn parse_input(s: &str) -> Vec<Nanobot> {
    let re_nano = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    s.lines().filter_map(|line| {
        if let Some(caps) = re_nano.captures(line) {
            let x = caps[1].parse().unwrap();
            let y = caps[2].parse().unwrap();
            let z = caps[3].parse().unwrap();
            let radius = caps[4].parse().unwrap();
            Some(Nanobot { pos: Pos(x,y,z), radius })
        } else {
            println!("invalid nanobot: {}", line);
            None
        }
    }).collect()
}

impl Nanobot {
    fn intersects(&self, cube: &Cube) -> bool {
        // Get closest point on cube to nanobot.
        let x = self.pos.0.max(cube.min.0).min(cube.min.0 + cube.side - 1);
        let y = self.pos.1.max(cube.min.1).min(cube.min.1 + cube.side - 1);
        let z = self.pos.2.max(cube.min.2).min(cube.min.2 + cube.side - 1);
        // Check the distance from that point to nanobot.
        self.pos.dist(&Pos(x,y,z)) <= self.radius
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Cube {
    min: Pos,
    side: i32,
}

impl Cube {
    fn subdivide(&self) -> Vec<Cube> {
        let side = self.side / 2;
        if side == 0 { return vec![]; }
        let xs = &[self.min.0, self.min.0 + side];
        let ys = &[self.min.1, self.min.1 + side];
        let zs = &[self.min.2, self.min.2 + side];
        xs.iter().flat_map(|&x|
            ys.iter().flat_map(move |&y|
                zs.iter().map(move |&z|
                    Cube { min: Pos(x, y, z), side }
                )
            )
        ).collect()
    }

    fn origin_dist(&self) -> i32 {
        span_dist(self.min.0, self.side)
        + span_dist(self.min.1, self.side)
        + span_dist(self.min.2, self.side)
    }
}

fn span_dist(min: i32, len: i32) -> i32 {
    let max = min + len - 1;
    if min > 0 { min } else if max < 0 { -max } else { 0 }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct CubeSearchState {
    cube: Cube,
    count: usize,
}

impl Ord for CubeSearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.count.cmp(&self.count).then(
            self.cube.origin_dist().cmp(&other.cube.origin_dist()).then(
                self.cube.side.cmp(&other.cube.side)))
    }
}

impl PartialOrd for CubeSearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct CubeSearch(Vec<Nanobot>);

impl SearchSpec for CubeSearch {
    type State = CubeSearchState;
    type Token = Cube;

    fn branch(&self, state: &CubeSearchState) -> Vec<CubeSearchState> {
        state.cube.subdivide().into_iter().map(|cube| {
            let count = self.0.iter().filter(|&n| n.intersects(&cube)).count();
            CubeSearchState { cube, count }
        }).collect()
    }

    fn is_goal(&self, state: &CubeSearchState) -> bool {
        state.cube.side == 1
    }

    fn token(&self, state: &CubeSearchState) -> Cube {
        state.cube
    }
}

fn search_cubes(nanobots: &[Nanobot]) -> (Pos, usize) {
    let searcher = CubeSearch(nanobots.to_vec());
    let v = 1<<29;
    let init_state = CubeSearchState {
        cube: Cube { min: Pos(-v, -v, -v), side: 2*v },
        count: nanobots.len(),
    };
    let best = best_first_search(searcher, init_state).unwrap();
    (best.cube.min, best.count)
}

fn solve(input: &str) -> (usize, u32) {
    let nanos = parse_input(input);
    let max_r = nanos.iter().max_by_key(|n| n.radius).unwrap();
    // println!("max radius nano = {:?}", max_r);
    let in_range = nanos.iter().filter(|&n| max_r.pos.dist(&n.pos) <= max_r.radius).count();

    let (pos, _count) = search_cubes(&nanos);
    // println!("best position {:?}, count {}", pos, _count);

    (in_range, Pos(0,0,0).dist(&pos) as u32)
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
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE), vec![
            Nanobot { pos: Pos(0,0,0), radius: 4 },
            Nanobot { pos: Pos(1,0,0), radius: 1 },
            Nanobot { pos: Pos(4,0,0), radius: 3 },
            Nanobot { pos: Pos(0,2,0), radius: 1 },
            Nanobot { pos: Pos(0,5,0), radius: 3 },
            Nanobot { pos: Pos(0,0,3), radius: 1 },
            Nanobot { pos: Pos(1,1,1), radius: 1 },
            Nanobot { pos: Pos(1,1,2), radius: 1 },
            Nanobot { pos: Pos(1,3,1), radius: 1 },
        ]);
    }

    #[test]
    fn example() {
        let (part1, part2) = solve(EXAMPLE);
        assert_eq!(7, part1);
        assert_eq!(1, part2);
    }

    #[test]
    fn example1() {
        let (part1, part2) = solve("\
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
");
        assert_eq!(6, part1);
        assert_eq!(36, part2);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day23.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day23.txt"),
                   format!("{:?}", x));
    }
}
