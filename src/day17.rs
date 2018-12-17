use regex::Regex;

const SHOW_FINAL_GRID: bool = false;

type Vein = (usize, usize, usize, usize);

fn parse_input(s: &str) -> Vec<Vein> {
    let re_vein_v = Regex::new(r"x=(\d+), y=(\d+)..(\d+)").unwrap();
    let re_vein_h = Regex::new(r"y=(\d+), x=(\d+)..(\d+)").unwrap();
    s.lines().map(|line| {
        if let Some(caps) = re_vein_v.captures(line) {
            let x = caps[1].parse().unwrap();
            let y0 = caps[2].parse().unwrap();
            let y1 = caps[3].parse().unwrap();
            (x, x, y0, y1)
        } else if let Some(caps) = re_vein_h.captures(line) {
            let y = caps[1].parse().unwrap();
            let x0 = caps[2].parse().unwrap();
            let x1 = caps[3].parse().unwrap();
            (x0, x1, y, y)
        } else {
            panic!("invalid line in input: {}", line);
        }
    }).collect()
}

fn region_bounds(veins: &[Vein]) -> (usize, usize, usize, usize) {
    let min_x = veins.iter().map(|v| v.0).min().unwrap();
    let max_x = veins.iter().map(|v| v.1).max().unwrap();
    let min_y = veins.iter().map(|v| v.2).min().unwrap();
    let max_y = veins.iter().map(|v| v.3).max().unwrap();
    (min_x, max_x, min_y, max_y)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Square {
    Sand,
    Clay,
    Water,
    WetSand,
}

impl Square {
    fn is_open(&self) -> bool {
        match self {
            Square::Sand | Square::WetSand => true,
            _ => false,
        }
    }

    fn is_wet(&self) -> bool {
        match self {
            Square::Water | Square::WetSand => true,
            _ => false,
        }
    }
}

type Grid = Vec<Vec<Square>>;

fn show_grid(grid: &Grid) {
    for row in grid.iter() {
        let line: String = row.iter().map(|s| match s {
            Square::Sand => '.',
            Square::Clay => '#',
            Square::Water => '~',
            Square::WetSand => '|',
        }).collect();
        println!("{}", line);
    }
}

// Drop water at given point. Return true if it flows off the bottom.
fn drop_water(grid: &mut Grid, x: usize, y: usize) -> bool {
    assert_eq!(grid[y][x], Square::Sand);
    grid[y][x] = Square::WetSand;

    // If at bottom or dropping water below flows out, then flow out.
    if y+1 >= grid.len()
        || grid[y+1][x] == Square::WetSand
        || grid[y+1][x] == Square::Sand && drop_water(grid, x, y+1)
    {
        return true;
    }

    // If flowing left or right flows out, then flow out.
    let flow_left = flow(grid, x-1, y, -1);
    let flow_right = flow(grid, x+1, y, 1);
    if flow_left || flow_right {
        return true;
    }

    // Didn't flow out, so fill row with water.
    fill(grid, x, y, -1);
    fill(grid, x+1, y, 1);

    false
}

fn flow(grid: &mut Grid, mut x: usize, y: usize, dir: i32) -> bool {
    while grid[y][x] == Square::Sand {
        grid[y][x] = Square::WetSand;

        // If at bottom or dropping water below flows out, then flow out.
        if y+1 >= grid.len()
            || grid[y+1][x] == Square::WetSand
            || grid[y+1][x] == Square::Sand && drop_water(grid, x, y+1)
        {
            return true;
        }

        x = (x as i32 + dir) as usize;
    }

    // If we found wet sand, it must have flowed out. Otherwise, we don't flow out.
    grid[y][x] == Square::WetSand
}

fn fill(grid: &mut Grid, mut x: usize, y: usize, dir: i32) {
    while grid[y][x].is_open() {
        grid[y][x] = Square::Water;
        x = (x as i32 + dir) as usize;
    }
}

fn solve(input: &str) -> (usize, usize) {
    let veins = parse_input(input);
    let (min_x, max_x, min_y, max_y) = region_bounds(&veins);
    // println!("region top left: ({},{}), bot right", min_x, min_y, max_x, max_y);

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut grid = vec![vec![Square::Sand; width+2]; height];
    for vein in veins.iter() {
        for col in vein.0 ..= vein.1 {
            for row in vein.2 ..= vein.3 {
                grid[row - min_y][col - min_x + 1] = Square::Clay;
            }
        }
    }

    // println!("initial grid:");
    // show_grid(&grid);
    // println!("--");

    drop_water(&mut grid, 500 - min_x + 1, 0);

    if SHOW_FINAL_GRID {
        show_grid(&grid);
    }

    let wet = grid.iter().flat_map(|row| row.iter())
        .filter(|s| s.is_wet())
        .count();
    let water = grid.iter().flat_map(|row| row.iter())
        .filter(|&&s| s == Square::Water)
        .count();

    (wet, water)
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
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE), vec![
            (495, 495, 2, 7),
            (495, 501, 7, 7),
            (501, 501, 3, 7),
            (498, 498, 2, 4),
            (506, 506, 1, 2),
            (498, 498, 10, 13),
            (504, 504, 10, 13),
            (498, 504, 13, 13),
        ]);
    }

    #[test]
    fn example() {
        let (part1, part2) = solve(EXAMPLE);
        assert_eq!(57, part1);
        assert_eq!(29, part2);
    }

    #[test]
    fn flow_check() {
        let input = "\
x=494, y=3..9
y=5, x=499..501
y=8, x=497..499
y=8, x=501..503
x=504, y=7..8
";
        assert_eq!((22, 0), solve(input));
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day17.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day17.txt"),
                   format!("{:?}", x));
    }
}
