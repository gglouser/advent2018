fn parse_input(s: &str) -> i32 {
    s.trim().parse().unwrap()
}

fn fuel_cell(x: i32, y: i32, ser_no: i32) -> i32 {
    let rack_id = x + 10;
    (((rack_id * y + ser_no) * rack_id) / 100) % 10 - 5
}

fn generate_grid(ser_no: i32) -> Vec<Vec<i32>> {
    (1..=300).map(|y|
        (1..=300).map(|x|
            fuel_cell(x, y, ser_no)
        ).collect()
    ).collect()
}

fn partial_sums(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut p = vec![vec![0; 301]; 301];
    for y in 1..=300 {
        for x in 1..=300 {
            p[y][x] = grid[y-1][x-1] + p[y-1][x] + p[y][x-1] - p[y-1][x-1];
        }
    }
    p
}

fn find_most_power(psum_grid: &Vec<Vec<i32>>, window: usize) -> (i32, (usize, usize)) {
    (0..=300-window).flat_map(|y|
        (0..=(300-window)).map(move |x| {
            let a = psum_grid[y+window][x+window];
            let b = psum_grid[y+window][x];
            let c = psum_grid[y][x+window];
            let d = psum_grid[y][x];
            (a - b - c + d, (x+1, y+1))
        }))
        .max_by_key(|&(power,_)| power).unwrap()
}

fn solve(input: &str) -> (String, String) {
    let grid_serial_no = parse_input(input);
    let grid = generate_grid(grid_serial_no);
    let psum_grid = partial_sums(&grid);

    let (_, best3) = find_most_power(&psum_grid, 3);
    let part1 = format!("{},{}", best3.0, best3.1);
    
    let (_, best_pos, best_window) = (1..300).map(|window| {
            let (power, pos) = find_most_power(&psum_grid, window);
            (power, pos, window)
        }).max_by_key(|&(power,_,_)| power).unwrap();
    let part2 = format!("{},{},{}", best_pos.0, best_pos.1, best_window);

    (part1, part2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the solution to part 1 is {}", part1);
    println!("the solution to part 2 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(parse_input("18\n"), 18);
    }

    #[test]
    fn example() {
        let (part1, part2) = solve("18");
        assert_eq!("33,45", part1);
        assert_eq!("90,269,16", part2);
    }
    
    #[test]
    fn example2() {
        let (part1, part2) = solve("42");
        assert_eq!("21,61", part1);
        assert_eq!("232,251,12", part2);
    }
    
    #[test]
    fn fuel_cells() {
        assert_eq!(fuel_cell(3, 5, 8), 4);
        assert_eq!(fuel_cell(122, 79, 57), -5);
        assert_eq!(fuel_cell(217, 196, 39), 0);
        assert_eq!(fuel_cell(101, 153, 71), 4);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day11.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day11.txt"),
                   format!("{:?}", x));
    }
}
