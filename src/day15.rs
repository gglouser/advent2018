use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};
use search::*;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos(i32, i32);

impl Pos {
    fn neighbors(&self) -> impl Iterator<Item=Pos> {
        vec![
            Pos(self.0 - 1, self.1),
            Pos(self.0,     self.1 - 1),
            Pos(self.0,     self.1 + 1),
            Pos(self.0 + 1, self.1),
        ].into_iter()
    }

    fn dist(&self, other: &Pos) -> u32 {
        (self.0 - other.0).abs() as u32 + (self.1 - other.1).abs() as u32
    }
}

type UnitID = usize;

#[derive(Clone, Copy, Debug, PartialEq)]
enum GridContents {
    Wall,
    Open,
    Unit(UnitID),
}

#[derive(Clone)]
struct Grid(Vec<Vec<GridContents>>);

impl Index<Pos> for Grid {
    type Output = GridContents;
    fn index(&self, pos: Pos) -> &GridContents {
        &self.0[pos.0 as usize][pos.1 as usize]
    }
}

impl IndexMut<Pos> for Grid {
    fn index_mut(&mut self, pos: Pos) -> &mut GridContents {
        &mut self.0[pos.0 as usize][pos.1 as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Team { Elf, Goblin }

impl Team {
    fn symbol(&self) -> char {
        match self {
            Team::Elf => 'E',
            Team::Goblin => 'G',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Unit {
    team: Team,
    pos: Pos,
    atk: u32,
    hp: u32,
}

impl Unit {
    fn new(team: Team, pos: Pos) -> Self {
        Unit { team, pos, atk: 3 , hp: 200}
    }
}

fn push_unit(units: &mut Vec<Unit>, team: Team, row: usize, col: usize) -> UnitID {
    units.push(Unit::new(team, Pos(row as i32, col as i32)));
    units.len() - 1
}

fn parse_input(s: &str) -> (Grid, Vec<Unit>) {
    let mut units = Vec::new();
    let grid = s.lines().enumerate().map(|(row, line)|
        line.chars().enumerate().map(|(col, c)|
            match c {
                'E' => {
                    let uid = push_unit(&mut units, Team::Elf, row, col);
                    GridContents::Unit(uid)
                },
                'G' => {
                    let uid = push_unit(&mut units, Team::Goblin, row, col);
                    GridContents::Unit(uid)
                },
                '.' => GridContents::Open,
                _   => GridContents::Wall,
            }
        ).collect()
    ).collect();
    (Grid(grid), units)
}

fn grid_string(grid: &Grid, units: &[Unit]) -> String {
    grid.0.iter()
        .map(|row| {
            let mut line = row.iter().map(|&c|
                match c {
                    GridContents::Open    => '.',
                    GridContents::Wall    => '#',
                    GridContents::Unit(u) => units[u].team.symbol(),
                }
            ).collect::<String>();

            row.iter()
                .filter_map(|&c| if let GridContents::Unit(u) = c { Some(&units[u]) } else { None })
                .enumerate()
                .for_each(|(i,u)| {
                    if i == 0 {
                        line += "   ";
                    } else {
                        line += ", ";
                    }
                    line += &format!("{}({})", u.team.symbol(), u.hp);
                });

            line.push('\n');
            line
        }).collect()
}

#[derive(Clone, Debug, Eq)]
struct PathSearchState {
    pos: Pos,
    steps: u32,
    cost: u32,
    target: Pos,
    first_step_rank: Option<u32>,
    route: Vec<Pos>,
}

impl PartialEq for PathSearchState {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Hash for PathSearchState {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.pos.hash(hasher);
    }
}

impl Ord for PathSearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
            .then(other.pos.cmp(&self.pos))
        // other.cost.cmp(&self.cost)
            // .then(other.target.cmp(&self.target))
            .then(other.first_step_rank.cmp(&self.first_step_rank))
    }
}

impl PartialOrd for PathSearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PathSearchState {
    fn new(pos: Pos) -> Self {
        PathSearchState {
            pos,
            steps: 0,
            cost: 0,
            target: pos,
            first_step_rank: None,
            route: Vec::new(),
        }
    }
}

struct PathSearch<'a> {
    grid: &'a Grid,
    dests: &'a HashSet<Pos>,
}

impl<'a> SearchSpec for PathSearch<'a> {
    type SearchState = PathSearchState;

    fn branch(&self, state: &Self::SearchState) -> Vec<Self::SearchState> {
        state.pos.neighbors().enumerate()
            .filter(|&(_, n)| self.grid[n] == GridContents::Open)
            .map(|(step_rank, pos)| {
                
                let (&target,heuristic) = self.dests.iter()
                    .map(|d| (d, pos.dist(d)))
                    .min_by_key(|&(_,dist)| dist)
                    .unwrap();
                
                let mut route = state.route.clone();
                route.push(pos);

                PathSearchState {
                    pos,
                    steps: state.steps + 1,
                    cost: state.steps + 1 + heuristic,
                    target,
                    first_step_rank: state.first_step_rank.or(Some(step_rank as u32)),
                    route,
                }
            }).collect()
    }

    fn is_goal(&self, state: &Self::SearchState) -> bool {
        self.dests.contains(&state.pos)
    }
}

fn choose_step(grid: &Grid, start: Pos, dests: &HashSet<Pos>) -> Option<Pos> {
    let searcher = PathSearch { grid, dests };
    best_first_search(searcher, PathSearchState::new(start))
        .map(|st| st.route[0])
}

fn simulate(grid: &Grid, units: &[Unit], elf_atk: u32) -> (u32, bool) {
    println!("\nSimulating with elf attack power = {}", elf_atk);
    let initial_elves = units.iter().filter(|u| u.team == Team::Elf).count();

    let mut grid = grid.clone();
    let mut units = units.to_vec();
    for unit in units.iter_mut() {
        if unit.team == Team::Elf {
            unit.atk = elf_atk;
        }
    }
    let mut active_units: Vec<UnitID> = (0..units.len()).collect();
    let mut round = 0u32;
    'combat: loop {
        // println!("-- begin round {} -- {} active units --", round, active_units.len());
        // println!("{}", grid_string(&grid, &units));

        // Initiative order for this round
        active_units.sort_unstable_by_key(|&u| units[u].pos);

        // For each unit...
        for &u in active_units.iter() {
            // This unit may have been killed during the round
            if units[u].hp == 0 { continue; }

            // I. Move
            //   A. Identify targets
            let targets = active_units.iter()
                .filter(|&&t| units[t].hp != 0 && units[t].team != units[u].team)
                .cloned().collect::<Vec<UnitID>>();

            // If no targets remain, combat ends.
            if targets.is_empty() { break 'combat; }

            //   B. Identify open squares in range of (adjacent to) targets
            let mut adj_squares = targets.iter()
                .flat_map(|&t| units[t].pos.neighbors())
                .collect::<HashSet<Pos>>();
            if !adj_squares.contains(&units[u].pos) {
                adj_squares.retain(|&p| grid[p] == GridContents::Open);
                if !adj_squares.is_empty() {
                    //   C. Determine which can be reached in fewest steps
                    if let Some(s) = choose_step(&grid, units[u].pos, &adj_squares) {
                        // Finally, move this unit
                        let uid = grid[units[u].pos];
                        grid[units[u].pos] = GridContents::Open;
                        grid[s] = uid;
                        units[u].pos = s;
                    }
                }
            }

            // II. Attack
            // A. Determine targets in range
            let mut targets = units[u].pos.neighbors()
                .filter_map(|n|
                    if let GridContents::Unit(t) = grid[n] {
                        if units[t].team != units[u].team {
                            Some(t)
                        } else {
                            None
                        }
                    } else {
                        None
                    })
                .collect::<Vec<_>>();

            // B. Choose target
            if !targets.is_empty() {
                targets.sort_unstable_by_key(|&t| (units[t].hp, units[t].pos));
                let t = targets[0];

                // C. Deal damage
                if units[u].atk >= units[t].hp {
                    // Target killed
                    units[t].hp = 0;
                    grid[units[t].pos] = GridContents::Open;
                } else {
                    units[t].hp -= units[u].atk;
                }
            }
        }

        // Update active units
        active_units.retain(|&u| units[u].hp > 0);

        round += 1;
    }

    let hp_total = units.iter().map(|u| u.hp).sum::<u32>();
    let outcome = round * hp_total;

    active_units.retain(|&u| units[u].hp > 0);
    println!("-- final state -- {} active units --", active_units.len());
    println!("{}", grid_string(&grid, &units));
    println!("Combat ends after {} full rounds", round);
    println!("{:?} win with {} total hit points left", units[active_units[0]].team, hp_total);
    println!("Outcome: {} * {} = {}", round, hp_total, outcome);

    let final_elves = units.iter().filter(|u| u.hp > 0 && u.team == Team::Elf).count();
    let elf_victory = initial_elves == final_elves;

    (outcome, elf_victory)
}

fn solve(input: &str) -> (u32, u32) {
    let (grid, units) = parse_input(input);
    let (outcome,_) = simulate(&grid, &units, 3);

    let outcome2 = (4..).map(|elf_atk| simulate(&grid, &units, elf_atk))
        .find(|&(_,ev)| ev)
        .unwrap().0;

    (outcome, outcome2)
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
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
";

    #[test]
    fn parsing() {
        let (grid, units) = parse_input(EXAMPLE);
        assert_eq!(6, units.len());
        assert_eq!(units[0], Unit { team: Team::Goblin, pos: Pos(1,2), hp: 200, atk: 3 });
        assert_eq!(grid_string(&grid, &units), "\
#######
#.G...#   G(200)
#...EG#   E(200), G(200)
#.#.#G#   G(200)
#..G#E#   G(200), E(200)
#.....#
#######
");
    }

    #[test]
    fn example1() {
        let (part1, part2) = solve(EXAMPLE);
        assert_eq!(27730, part1);
        assert_eq!(4988, part2);
    }

    #[test]
    fn example2() {
        let (part1, part2) = solve("\
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
");
        assert_eq!(36334, part1);
        assert_eq!(29064, part2);
    }

    #[test]
    fn example3() {
        let (part1, part2) = solve("\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
");
        assert_eq!(39514, part1);
        assert_eq!(31284, part2);
    }

    #[test]
    fn example4() {
        let (part1, part2) = solve("\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
");
        assert_eq!(27755, part1);
        assert_eq!(3478, part2);
    }

    #[test]
    fn example5() {
        let (part1, part2) = solve("\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
");
        assert_eq!(28944, part1);
        assert_eq!(6474, part2);
    }

    #[test]
    fn example6() {
        let (part1, part2) = solve("\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
");
        assert_eq!(18740, part1);
        assert_eq!(1140, part2);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day15.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day15.txt"),
                   format!("{:?}", x));
    }
}
