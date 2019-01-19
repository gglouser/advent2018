use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::{Index, IndexMut};
use crate::search::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos(i32, i32);

impl Pos {
    fn neighbors(self) -> impl Iterator<Item=Pos> {
        vec![
            Pos(self.0 - 1, self.1),
            Pos(self.0,     self.1 - 1),
            Pos(self.0,     self.1 + 1),
            Pos(self.0 + 1, self.1),
        ].into_iter()
    }

    fn dist(self, other: Pos) -> u32 {
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
    fn symbol(self) -> char {
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct PathSearchState {
    pos: Pos,
    steps: u32,
    cost: u32,
    target: Pos,
    first_step: Option<Pos>,
}

impl Ord for PathSearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
            .then(self.target.cmp(&other.target))
            .then(self.first_step.cmp(&other.first_step))
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
            first_step: None,
        }
    }
}

struct PathSearch<'a> {
    grid: &'a Grid,
    dests: &'a HashSet<Pos>,
}

impl<'a> SearchSpec for PathSearch<'a> {
    type State = PathSearchState;
    type Token = Pos;

    fn branch(&self, state: &Self::State) -> Vec<Self::State> {
        state.pos.neighbors()
            .filter(|&n| self.grid[n] == GridContents::Open)
            .map(|pos| {
                let (tdist, &target) = self.dests.iter()
                    .map(|d| (pos.dist(*d), d))
                    .min().unwrap();
                let steps = state.steps + 1;
                PathSearchState {
                    pos,
                    steps,
                    cost: steps + tdist,
                    target,
                    first_step: state.first_step.or_else(|| Some(pos)),
                }
            }).collect()
    }

    fn is_goal(&self, state: &Self::State) -> bool {
        self.dests.contains(&state.pos)
    }

    fn token(&self, state: &Self::State) -> Self::Token {
        state.pos
    }
}

fn choose_step(grid: &Grid, start: Pos, dests: &HashSet<Pos>) -> Option<Pos> {
    if dests.is_empty() { return None };
    let searcher = PathSearch { grid, dests };
    best_first_search(searcher, PathSearchState::new(start))
        .map(|st| st.first_step.expect("no first step"))
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum FindMoveResult {
    NoTargets,
    NoPath,
    Step(Pos),
}

struct Simulation {
    grid: Grid,
    units: Vec<Unit>,
    verbose: bool,
}

impl Simulation {
    fn new(grid: Grid, units: Vec<Unit>) -> Self {
        Simulation { grid, units, verbose: false }
    }

    fn move_unit(&mut self, uid: UnitID, new_pos: Pos) {
        assert!(self.grid[self.units[uid].pos] == GridContents::Unit(uid));
        assert!(self.grid[new_pos] == GridContents::Open);
        self.grid[self.units[uid].pos] = GridContents::Open;
        self.grid[new_pos] = GridContents::Unit(uid);
        self.units[uid].pos = new_pos;
    }

    fn resolve_attack(&mut self, attacker: UnitID, defender: UnitID) -> bool {
        if self.units[attacker].atk >= self.units[defender].hp {
            self.units[defender].hp = 0;
            self.grid[self.units[defender].pos] = GridContents::Open;
            true
        } else {
            self.units[defender].hp -= self.units[attacker].atk;
            false
        }
    }

    fn find_move(&self, uid: UnitID) -> FindMoveResult {
        // Identify targets.
        let mover = &self.units[uid];
        let targets: Vec<&Unit> = self.units.iter()
            .filter(|&target| target.hp != 0 && target.team != mover.team)
            .collect();
        if targets.is_empty() { return FindMoveResult::NoTargets; }

        // Identify open squares in range of (adjacent to) targets.
        let adj_squares: HashSet<Pos> = targets.into_iter()
            .flat_map(|target| target.pos.neighbors())
            .filter(|&pos| self.grid[pos] == GridContents::Open)
            .collect();
        match choose_step(&self.grid, mover.pos, &adj_squares) {
            Some(step) => FindMoveResult::Step(step),
            None => FindMoveResult::NoPath,
        }
    }

    fn find_attack_target(&self, attacker: UnitID) -> Option<UnitID> {
        self.units[attacker].pos.neighbors()
            .filter_map(|pos| if let GridContents::Unit(uid) = self.grid[pos] { Some(uid) } else { None })
            .filter(|&target| self.units[target].team != self.units[attacker].team)
            .min_by_key(|&target| (self.units[target].hp, self.units[target].pos))
    }

    // Return true if combat should stop.
    fn take_turn(&mut self, uid: UnitID, stop_on_elf_death: bool) -> bool {
        // The unit may have been killed earlier in the current round.
        if self.units[uid].hp == 0 { return false; }

        // I. Move
        let mut attack_target = self.find_attack_target(uid);
        if attack_target.is_none() {
            match self.find_move(uid) {
                FindMoveResult::NoTargets => { return true; },  // No targets remain; combat ends.
                FindMoveResult::NoPath => (),                   // Targets exist but no path, so do nothing.
                FindMoveResult::Step(new_pos) => {
                    // Move and reacquire attack target.
                    self.move_unit(uid, new_pos);
                    attack_target = self.find_attack_target(uid);
                },
            }
        }

        // II. Attack
        if let Some(target) = attack_target {
            let killed = self.resolve_attack(uid, target);
            if stop_on_elf_death && killed && self.units[target].team == Team::Elf {
                return true;
            }
        }

        // Turn resolved successfully, combat continues.
        false
    }

    fn simulate(&mut self, stop_on_elf_death: bool) -> (u32, bool) {
        let initial_elves = self.units.iter().filter(|u| u.team == Team::Elf).count();
        let mut active_units: Vec<UnitID> = (0..self.units.len()).collect();
        let mut round = 0u32;
        loop {
            if self.verbose {
                println!("-- begin round {} -- {} active units --", round, active_units.len());
                println!("{}", grid_string(&self.grid, &self.units));
            }

            // Initiative order for this round.
            active_units.sort_unstable_by_key(|&u| self.units[u].pos);

            // Take turns. If any unit indicates combat should stop, then stop.
            if active_units.iter().any(|&uid| self.take_turn(uid, stop_on_elf_death)) {
                break;
            }

            // Remove dead units from active list.
            active_units.retain(|&uid| self.units[uid].hp > 0);

            round += 1;
        }

        let hp_total = self.units.iter().map(|u| u.hp).sum::<u32>();
        let outcome = round * hp_total;

        if self.verbose {
            active_units.retain(|&u| self.units[u].hp > 0);
            println!("-- final state -- {} active units --", active_units.len());
            println!("{}", grid_string(&self.grid, &self.units));
            println!("Combat ends after {} full rounds", round);
            println!("{:?} win with {} total hit points left", self.units[active_units[0]].team, hp_total);
            println!("Outcome: {} * {} = {}", round, hp_total, outcome);
        }

        let final_elves = self.units.iter().filter(|u| u.hp > 0 && u.team == Team::Elf).count();
        let elf_victory = initial_elves == final_elves;

        (outcome, elf_victory)
    }
}

fn solve(input: &str) -> (u32, u32) {
    let (grid, units) = parse_input(input);
    let mut sim = Simulation::new(grid.clone(), units.clone());
    let (outcome,_) = sim.simulate(false);

    let outcome2 = (4..).map(|elf_atk| {
        // Set elf attack power.
        println!("\nSimulating with elf attack power = {}", elf_atk);
        let mut xunits = units.clone();
        for unit in xunits.iter_mut() {
            if unit.team == Team::Elf {
                unit.atk = elf_atk;
            }
        }
        let mut sim = Simulation::new(grid.clone(), xunits);
        sim.simulate(true)
    })
    .find(|&(_,ev)| ev).unwrap().0;

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

    #[test]
    fn path1() {
        // The elf should move right.
        let (grid, units) = parse_input("\
#######
#.E..G#
#.#####
#G#####
#######
");
        let sim = Simulation::new(grid, units);
        assert_eq!(FindMoveResult::Step(Pos(1,3)), sim.find_move(0))
    }

    #[test]
    fn path2() {
        // The elf should move left.
        let (grid, units) = parse_input("\
########
#..E..G#
#G######
########
");
        let sim = Simulation::new(grid, units);
        assert_eq!(FindMoveResult::Step(Pos(1,2)), sim.find_move(0))
    }

    #[test]
    fn path3() {
        // The goblin should move down.
        let (grid, units) = parse_input("\
######
#.G..#
##..##
#...E#
#E...#
######
");
        let sim = Simulation::new(grid, units);
        assert_eq!(FindMoveResult::Step(Pos(2,2)), sim.find_move(0))
    }

    #[test]
    fn attack1() {
        // Simulate a complete round. The elf should attack the goblin directly above.
        let (grid, units) = parse_input("\
####
#GG#
#.E#
####
");
        let mut sim = Simulation::new(grid, units);
        let stop = (0..3).any(|uid| sim.take_turn(uid, false));
        assert!(!stop);
        assert_eq!(sim.units, vec![
            Unit { team: Team::Goblin, pos: Pos(2,1), atk: 3, hp: 200 },
            Unit { team: Team::Goblin, pos: Pos(1,2), atk: 3, hp: 197 },
            Unit { team: Team::Elf,    pos: Pos(2,2), atk: 3, hp: 194 },
        ])
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
