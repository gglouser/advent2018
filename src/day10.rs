#[derive(Clone, Copy, Debug, PartialEq)]
struct Point(i32, i32);

#[derive(Clone, Copy, Debug, PartialEq)]
struct Star(Point, Point);

impl Star {
    fn step(&mut self) {
        self.jump(1);
    }
    
    fn jump(&mut self, t: i32) {
        (self.0).0 += (self.1).0 * t;
        (self.0).1 += (self.1).1 * t;
    }
}

fn parse_input(s: &str) -> Vec<Star> {
    s.lines().map(|line| {
        let mut parts = line.split(|c| c == '<' || c == '>' || c == ',');
        parts.next();
        let x = parts.next().unwrap().trim().parse().unwrap();
        let y = parts.next().unwrap().trim().parse().unwrap();
        parts.next();
        let vx = parts.next().unwrap().trim().parse().unwrap();
        let vy = parts.next().unwrap().trim().parse().unwrap();
        Star(Point(x,y), Point(vx, vy))
    }).collect()
}

fn show_stars(stars: &[Star], left: i32, right: i32, top: i32, bottom: i32) {
    for y in top..=bottom {
        for x in left..=right {
            if stars.iter().any(|s| s.0 == Point(x,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve(input: &str) -> usize {
    let mut stars = parse_input(input);
    let mut last_area = usize::max_value();
    let mut stop_time = 0;
    for t in 0.. {
        let min_x = stars.iter().map(|s| (s.0).0).min().unwrap();
        let max_x = stars.iter().map(|s| (s.0).0).max().unwrap();
        let min_y = stars.iter().map(|s| (s.0).1).min().unwrap();
        let max_y = stars.iter().map(|s| (s.0).1).max().unwrap();

        let width = (max_x - min_x) as usize;
        let height = (max_y - min_y) as usize;
        let area = width * height;
        // println!("bounding rect {}x{} ({} total)", width, height, area);
        if area > last_area {
            stop_time = t - 1;
            break;
        }
        last_area = area;
        stars.iter_mut().for_each(|s| s.step());
    }

    stars.iter_mut().for_each(|s| s.jump(-1));
    let min_x = stars.iter().map(|s| (s.0).0).min().unwrap();
    let max_x = stars.iter().map(|s| (s.0).0).max().unwrap();
    let min_y = stars.iter().map(|s| (s.0).1).min().unwrap();
    let max_y = stars.iter().map(|s| (s.0).1).max().unwrap();
    show_stars(&stars, min_x-1, max_x+1, min_y-1, max_y+1);
    
    stop_time
}

pub fn run(input: &str) {
    let time = solve(input);
    println!("the solution to part 2 is {}", time);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE : &'static str = "\
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input("position=< 9,  1> velocity=< 0,  2>\n\
                                position=< 11063, -22004> velocity=<-1, -2>\n"),
            vec![
                Star(Point( 9, 1), Point( 0, 2)),
                Star(Point(11063, -22004), Point(-1,-2)),
            ]);
    }

    #[test]
    fn example() {
        let part2 = solve(EXAMPLE);
        assert_eq!(3, part2);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day10.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day10.txt"),
                   format!("{:?}", x));
    }
}
