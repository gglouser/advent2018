use std::collections::HashMap;

fn parse_input(s: &str) -> (&[u8], HashMap<&[u8],u8>) {
    let mut lines = s.lines();
    let init = lines.next().unwrap();
    let init = &init.as_bytes()[15..];

    lines.next(); // blank line
    let rules = lines.map(|line| {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let rule = parts[0].as_bytes();
        let result = parts[2].as_bytes()[0];
        (rule, result)
    }).collect::<HashMap<_,_>>();

    (init, rules)
}

fn step(state: &[u8], rules: &HashMap<&[u8],u8>) -> Vec<u8> {
    let temp = [b"....", state, b"...."].concat();
    temp.windows(5)
        .map(|w| if let Some(&x) = rules.get(w) { x } else { b'.' })
        .collect::<Vec<_>>()
}

fn is_plant(b: u8) -> bool {
    b == b'#'
}

// fn show(state: &[u8]) -> String {
    // state.iter().map(|&b| if is_plant(b) { '#' } else { '.' }).collect()
// }

fn plant_machine(initial_state: &[u8], rules: &HashMap<&[u8],u8>, gens: usize) -> (Vec<u8>, i64) {
    let mut state = initial_state.iter().cloned().collect::<Vec<_>>();
    let mut pos: i64 = 0;
    for _ in 0..gens {
        let first_plant = state.iter().position(|&p| is_plant(p)).unwrap();
        let last_plant = state.iter().rposition(|&p| is_plant(p)).unwrap();
        let next = step(&state[first_plant..=last_plant], &rules);
        pos = pos - 2 + first_plant as i64;
        state = next;
    }
    (state, pos)
}

fn score(state: &[u8], left_pos: i64) -> i64 {
    state.iter().enumerate()
        .filter_map(|(i,&p)| if is_plant(p) { Some(i as i64 + left_pos) } else { None })
        .sum()
}

fn solve(input: &str) -> (i64, i64) {
    let (initial_state, rules) = parse_input(input);

    let (state20, pos20) = plant_machine(initial_state, &rules, 20);
    let part1 = score(&state20, pos20);

    let (st1000, pos1000) = plant_machine(initial_state, &rules, 1000);
    let (st2000, pos2000) = plant_machine(initial_state, &rules, 2000);
    assert_eq!(st1000, st2000);

    let pos_x = (50_000_000 - 1) * (pos2000 - pos1000) as usize + pos1000 as usize;
    let score_x = score(&st1000, pos_x as i64);

    (part1, score_x)
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
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
";

    #[test]
    fn parsing() {
        let (init, rules) = parse_input(EXAMPLE);
        assert_eq!(init, b"#..#.#..##......###...###");
        assert_eq!(rules.get("...##".as_bytes()), Some(&b'#'));
        assert_eq!(rules.get("####.".as_bytes()), Some(&b'#'));
    }

    #[test]
    fn example() {
        let (init, rules) = parse_input(EXAMPLE);
        let (st,pos) = plant_machine(init, &rules, 20);
        let part1 = score(&st, pos);
        assert_eq!(325, part1);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day12.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day12.txt"),
                   format!("{:?}", x));
    }
}
