use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos(i32, i32);

impl Add for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Clone, Copy)]
enum Dir { N, S, E, W }

impl Dir {
    fn step(&self) -> Pos {
        match self {
            Dir::N => Pos(-1, 0),
            Dir::S => Pos( 1, 0),
            Dir::E => Pos( 0, 1),
            Dir::W => Pos( 0,-1),
        }
    }

    fn opposite(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }
}

type Room = [bool; 4];

fn add_door(rooms: &mut HashMap<Pos, Room>, pos: Pos, dir: Dir) {
    let room = rooms.entry(pos).or_insert([false; 4]);
    room[dir as usize] = true;
}

fn step_all(states: &HashSet<Pos>, rooms: &mut HashMap<Pos, Room>, dir: Dir) -> HashSet<Pos> {
    states.iter().map(|&pos| {
        add_door(rooms, pos, dir);
        let new_pos = pos + dir.step();
        add_door(rooms, new_pos, dir.opposite());
        new_pos
    }).collect()
}

fn explore(regex: &str) -> HashMap<Pos, Room> {
    let mut rooms: HashMap<Pos, Room> = HashMap::new();
    let mut states = HashSet::new();
    states.insert(Pos(0,0));
    let mut stack = Vec::new();
    let mut branched = HashSet::new();
    for c in regex.chars() {
        match c {
            'N' => states = step_all(&states, &mut rooms, Dir::N),
            'S' => states = step_all(&states, &mut rooms, Dir::S),
            'E' => states = step_all(&states, &mut rooms, Dir::E),
            'W' => states = step_all(&states, &mut rooms, Dir::W),
            '(' => {
                stack.push((states.clone(), branched));
                branched = HashSet::new();
            },
            '|' => {
                branched.extend(states.into_iter());
                states = stack.last().expect("branch arm not in branch context").0.clone();
            },
            ')' => {
                branched.extend(states.into_iter());
                states = branched;
                let (_, prev_branched) = stack.pop().expect("unmatched branch close");
                branched = prev_branched;
                // println!("branch end, now at {} states", states.len());
            },
            '$' => {
                // println!("reached end with {} states", states.len());
                break;
            },
            '^' => (),
            c => println!("unrecognized regex character: {}", c)
        }
    }
    rooms
}

fn solve(input: &str) -> (u32, u32) {
    let facility = explore(input);
    // println!("{}", _show_facility(&facility));

    let mut bfs = VecDeque::new();
    bfs.push_back((Pos(0,0), 0));
    let mut max_dist = 0;
    let mut dist_1000 = 0;
    let mut visited: HashSet<Pos> = HashSet::new();
    while let Some((pos, dist)) = bfs.pop_front() {
        if visited.contains(&pos) { continue; }
        visited.insert(pos);
        if dist > max_dist {
            max_dist = dist;
        }
        if dist >= 1000 {
            dist_1000 += 1;
        }
        if let Some(room) = facility.get(&pos) {
            if room[Dir::N as usize] { bfs.push_back((pos + Dir::N.step(), dist+1)); }
            if room[Dir::S as usize] { bfs.push_back((pos + Dir::S.step(), dist+1)); }
            if room[Dir::E as usize] { bfs.push_back((pos + Dir::E.step(), dist+1)); }
            if room[Dir::W as usize] { bfs.push_back((pos + Dir::W.step(), dist+1)); }
        } else {
            panic!("In room ({},{}), but room not found!", pos.0, pos.1);
        }
    }

    (max_dist, dist_1000)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the solution to part 1 is {}", part1);
    println!("the solution to part 2 is {}", part2);
}

fn _show_facility(rooms: &HashMap<Pos, Room>) -> String {
    let rs: Vec<i32> = rooms.keys().map(|Pos(r,_)| *r).collect();
    let cs: Vec<i32> = rooms.keys().map(|Pos(_,c)| *c).collect();
    let min_r = *rs.iter().min().unwrap();
    let max_r = *rs.iter().max().unwrap();
    let min_c = *cs.iter().min().unwrap();
    let max_c = *cs.iter().max().unwrap();

    let mut result = String::new();
    for r in min_r..=max_r {
        // Wall
        result.push('#');
        for c in min_c..=max_c {
            result += if let Some(room) = rooms.get(&Pos(r,c)) {
                if room[Dir::N as usize] { "-#" } else { "##" }
            } else {
                "##"
            };
        }
        result.push('\n');

        // Rooms
        result.push('#');
        for c in min_c..=max_c {
            if let Some(room) = rooms.get(&Pos(r,c)) {
                result.push(if (r,c) == (0,0) { 'X' } else { '.' });
                result.push(if room[Dir::E as usize] { '|' } else { '#' });
            } else {
                result += "##";
            }
        }
        result.push('\n');
    }
    // Final wall
    result.push('#');
    for _ in min_c..=max_c { result += "##"; }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let (part1, part2) = solve("^WNE$");
        assert_eq!(3, part1);
        assert_eq!(0, part2);
    }

    #[test]
    fn example2() {
        let (part1, part2) = solve("^ENWWW(NEEE|SSE(EE|N))$");
        assert_eq!(10, part1);
        assert_eq!(0, part2);
    }

    #[test]
    fn example3() {
        let (part1, part2) = solve("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
        assert_eq!(18, part1);
        assert_eq!(0, part2);
    }

    #[test]
    fn example4() {
        let (part1, part2) = solve("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$");
        assert_eq!(23, part1);
        assert_eq!(0, part2);
    }

    #[test]
    fn example5() {
        let (part1, part2) = solve("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
        assert_eq!(31, part1);
        assert_eq!(0, part2);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day20.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day20.txt"),
                   format!("{:?}", x));
    }
}
