use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos(i32, i32);

#[derive(Clone, Copy, Debug, PartialEq)]
struct Heading(i32, i32);

impl Heading {
    fn turn_left(self) -> Self {
        Heading(self.1, -self.0)
    }

    fn turn_right(self) -> Self {
        Heading(-self.1, self.0)
    }

    fn curve_pos(self) -> Self {
        Heading(self.1, self.0)
    }

    fn curve_neg(self) -> Self {
        Heading(-self.1, -self.0)
    }
}

impl Add<Heading> for Pos {
    type Output = Pos;
    fn add(self, heading: Heading) -> Pos {
        Pos(self.0 + heading.0, self.1 + heading.1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Cart {
    pos: Pos,
    heading: Heading,
    turns: u32,
    dead: bool,
}

impl Cart {
    fn new(pos: Pos, heading: Heading) -> Self {
        Cart { pos, heading, turns: 0, dead: false }
    }

    fn next_pos(&self) -> Pos {
        self.pos + self.heading
    }

    // Move onto a new piece of track, possibly changing heading
    fn change_heading(&mut self, track: u8) {
        self.heading = match track {
            b'/' => self.heading.curve_neg(),
            b'\\' => self.heading.curve_pos(),
            b'+' => {
                self.turns += 1;
                match self.turns % 3 {
                    0 => self.heading.turn_right(),
                    1 => self.heading.turn_left(),
                    _ => self.heading,
                }
            },
            _ => self.heading,
        };
    }
}

#[derive(Clone)]
struct Simulation {
    grid: Vec<Vec<u8>>,
    carts: Vec<Cart>,
}

fn parse_pos(c: u8) -> (u8, Option<Heading>) {
    match c {
        b'^' => (b'|', Some(Heading( 0,-1))),
        b'v' => (b'|', Some(Heading( 0, 1))),
        b'<' => (b'-', Some(Heading(-1, 0))),
        b'>' => (b'-', Some(Heading( 1, 0))),
        _    => (c, None),
    }
}

fn parse_input(s: &str) -> Simulation {
    let mut carts = vec![];
    let grid = s.lines().enumerate().map(|(y, line)|
        line.bytes().enumerate().map(|(x, c)| {
            let (t, h) = parse_pos(c);
            if let Some(heading) = h {
                carts.push(Cart::new(Pos(x as i32, y as i32), heading));
            }
            t
        }).collect()
    ).collect();
    Simulation { grid, carts }
}

impl Simulation {
    fn move_carts(&mut self) -> Vec<Pos> {
        let mut crashes = vec![];
        self.carts.sort_unstable_by_key(|c| (c.pos.1, c.pos.0));
        for i in 0..self.carts.len() {
            if self.carts[i].dead { continue; }
            let new_pos = self.carts[i].next_pos();
            if let Some(j) = self.carts.iter().position(|c2| !c2.dead && c2.pos == new_pos) {
                crashes.push(new_pos);
                self.carts[i].dead = true;
                self.carts[j].dead = true;
            } else {
                self.carts[i].change_heading(self.grid[new_pos.1 as usize][new_pos.0 as usize]);
                self.carts[i].pos = new_pos;
            }
        }
        crashes
    }

    fn find_first_crash(&mut self) -> Pos {
        loop {
            let crashes = self.move_carts();
            if !crashes.is_empty() {
                return crashes[0];
            }
        }
    }

    fn find_last_cart(&mut self) -> Pos {
        let mut num_carts = self.carts.len();
        // println!("started with {} carts", num_carts);
        for _tick in 0.. {
            // println!("tick {}", _tick);
            let crashes = self.move_carts();
            num_carts -= 2*crashes.len();
            if num_carts == 1 {
                // println!("stopping at tick {}", _tick);
                return self.carts.iter().find(|c| !c.dead).unwrap().pos;
            }
        }
        unreachable!()
    }
}

fn solve(input: &str) -> (String, String) {
    let sim = parse_input(input);

    let first_crash = sim.clone().find_first_crash();
    let first_crash = format!("{},{}", first_crash.0, first_crash.1);

    let last_cart = sim.clone().find_last_cart();
    let last_cart = format!("{},{}", last_cart.0, last_cart.1);

    (first_crash, last_cart)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the solution to part 1 is {}", part1);
    println!("the solution to part 2 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE : &'static str =
r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
";

    const EXAMPLE2 : &'static str =
r"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
";

    #[test]
    fn parsing() {
        let sim = parse_input(EXAMPLE);
        assert_eq!(sim.grid[0][2], b'-');
        assert_eq!(sim.grid[3][9], b'|');
        assert_eq!(sim.carts, vec![
            Cart { pos: Pos(2,0), heading: Heading(1,0), turns: 0, dead: false },
            Cart { pos: Pos(9,3), heading: Heading(0,1), turns: 0, dead: false },
            ]);
    }

    #[test]
    fn example() {
        let mut sim = parse_input(EXAMPLE);
        let first_crash = sim.find_first_crash();
        assert_eq!(Pos(7,3), first_crash);
    }

    #[test]
    fn example2() {
        let mut sim = parse_input(EXAMPLE2);
        let last_cart = sim.find_last_cart();
        assert_eq!(Pos(6,4), last_cart);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day13.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day13.txt"),
                   format!("{:?}", x));
    }
}
