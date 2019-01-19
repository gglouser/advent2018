use regex::Regex;
use std::ops::{Add, Index};
use std::cmp::Ordering;
use crate::search::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos(i32, i32);

impl Add for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}

impl Pos {
    fn dist(self, other: Pos) -> u32 {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as u32
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum RType { Rocky, Wet, Narrow }

impl From<RType> for char {
    fn from(rtype: RType) -> char {
        match rtype {
            RType::Rocky  => '.',
            RType::Wet    => '=',
            RType::Narrow => '|',
        }
    }
}

fn parse_input(s: &str) -> (usize, Pos) {
    let re_depth = Regex::new(r"depth: (\d+)").unwrap();
    let re_target = Regex::new(r"target: (\d+),(\d+)").unwrap();
    let mut lines = s.lines();
    let depth = re_depth.captures(lines.next().unwrap()).unwrap();
    let depth = depth[1].parse().unwrap();
    let target = re_target.captures(lines.next().unwrap()).unwrap();
    let target = Pos(target[1].parse().unwrap(), target[2].parse().unwrap());
    (depth, target)
}

struct Grid(Vec<Vec<RType>>);

impl Index<Pos> for Grid {
    type Output = RType;
    fn index(&self, pos: Pos) -> &RType {
        &self.0[pos.1 as usize][pos.0 as usize]
    }
}

impl Grid {
    fn in_bounds(&self, pos: Pos) -> bool {
        pos.1 >= 0 && pos.1 < self.0.len() as i32
            && pos.0 >= 0 && pos.0 < self.0[0].len() as i32
    }

    fn _show(&self) -> String {
        let mut s = String::new();
        for row in self.0.iter() {
            s.extend(row.iter().map(|&region| char::from(region)));
            s.push('\n');
        }
        s
    }
}

fn gen_terrain(width: usize, height: usize, depth: usize, target: Pos) -> Grid {
    let mut erosion = vec![vec![0; width]; height];
    for y in 0..height {
        for x in 0..width {
            let gi = match (x,y) {
                (0,0) => 0,
                (u,v) if (u, v) == (target.0 as usize, target.1 as usize) => 0,
                (0,_) => y*48271,
                (_,0) => x*16807,
                (_,_) => erosion[y][x-1] * erosion[y-1][x],
            };
            erosion[y][x] = (gi + depth) % 20183;
        }
    }
    Grid(erosion.into_iter().map(|row|
        row.into_iter().map(|region|
            match region % 3 {
                0 => RType::Rocky,
                1 => RType::Wet,
                _ => RType::Narrow,
            }
        ).collect()
    ).collect())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Gear { Torch, Climbing, Neither }

impl Gear {
    fn compatible(self, rtype: RType) -> bool {
        match (self, rtype) {
            (Gear::Torch, RType::Wet)
            | (Gear::Climbing, RType::Narrow)
            | (Gear::Neither, RType::Rocky) => false,
            _ => true,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct PathSearchState {
    pos: Pos,
    gear: Gear,
    time: u32,
    cost: u32,
}

impl Ord for PathSearchState {
    fn cmp(&self, other: &PathSearchState) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for PathSearchState {
    fn partial_cmp(&self, other: &PathSearchState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct PathSearch {
    grid: Grid,
    target: Pos,
}

impl PathSearch {
    fn make_state(&self, pos: Pos, gear: Gear, time: u32) -> PathSearchState {
        PathSearchState {
            pos,
            gear,
            time,
            cost: time + pos.dist(self.target) + if gear == Gear::Torch { 0 } else { 7 },
        }
    }
}

impl SearchSpec for PathSearch {
    type State = PathSearchState;
    type Token = (Pos, Gear);

    fn branch(&self, state: &PathSearchState) -> Vec<PathSearchState> {
        let steps = [Pos(-1, 0), Pos(1, 0), Pos(0, -1), Pos(0, 1)].iter()
            .map(|&step| state.pos + step)
            .filter(|&pos| self.grid.in_bounds(pos)
                && state.gear.compatible(self.grid[pos]))
            .map(|pos| self.make_state(pos, state.gear, state.time + 1));

        let gear_change = [Gear::Torch, Gear::Climbing, Gear::Neither].iter()
            .filter(|&&gear| gear != state.gear && gear.compatible(self.grid[state.pos]))
            .map(|&gear| self.make_state(state.pos, gear, state.time + 7));

        steps.chain(gear_change).collect()
    }

    fn is_goal(&self, state: &PathSearchState) -> bool {
        state.pos == self.target && state.gear == Gear::Torch
    }

    fn token(&self, state: &PathSearchState) -> (Pos, Gear) {
        (state.pos, state.gear)
    }
}

fn solve(input: &str) -> (u32, u32) {
    let (depth, target) = parse_input(input);
    let maze_w = target.0 as usize + 50;
    let maze_h = target.1 as usize + 50;
    let maze = gen_terrain(maze_w, maze_h, depth, target);
    // println!("{}", maze._show());

    let risk_level = maze.0[..=target.1 as usize].iter()
        .flat_map(|row| row[..=target.0 as usize].iter()
            .map(|&t| t as u32)
        ).sum();

    let searcher = PathSearch { grid: maze, target };
    let init_state = searcher.make_state(Pos(0,0), Gear::Torch, 0);
    let result = best_first_search(searcher, init_state);
    let best_time = result.unwrap().time;
    // println!("result {:?}", result);

    (risk_level, best_time)
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
depth: 510
target: 10,10
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE), (510, Pos(10,10)));
    }

    #[test]
    fn example() {
        let (part1, part2) = solve(EXAMPLE);
        assert_eq!(114, part1);
        assert_eq!(45, part2);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day22.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day22.txt"),
                   format!("{:?}", x));
    }
}
