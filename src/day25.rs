type Point4D = [i32; 4];

fn parse_input(s: &str) -> Vec<Point4D> {
    s.lines().map(|line| {
        let coords: Vec<_> = line.split(',').map(|x| x.parse().unwrap()).collect();
        [coords[0], coords[1], coords[2], coords[3]]
    }).collect()
}

fn dist(a: &Point4D, b: &Point4D) -> i32 {
    (0..4).map(|i| (a[i] - b[i]).abs()).sum()
}

struct DisjointSet {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    counts: Vec<usize>,
    nsets: usize,
}

impl DisjointSet {
    fn new(size: usize) -> DisjointSet {
        DisjointSet {
            parents: (0..size).collect(),
            ranks: vec![0; size],
            counts: vec![1; size],
            nsets: size,
        }
    }

    fn find(&mut self, a: usize) -> usize {
        let x = self.parents[a];
        if x == a {
            a
        } else {
            let k = self.find(x);
            self.parents[a] = k;
            k
        }
    }

    fn union(&mut self, a: usize, b: usize) {
        let i = self.find(a);
        let j = self.find(b);
        if i != j {
            let i_rank = self.ranks[i];
            let j_rank = self.ranks[j];
            let count = self.counts[i] + self.counts[j];
            if i_rank < j_rank {
                self.parents[i] = j;
                self.counts[j] = count;
            } else {
                self.parents[j] = i;
                self.counts[i] = count;
                if i_rank == j_rank {
                    self.ranks[i] += 1;
                }
            }
            self.nsets -= 1;
        }
    }
}

fn solve(input: &str) -> usize {
    let points = parse_input(input);

    let mut dset = DisjointSet::new(points.len());
    for i in 0..points.len() {
        for j in i+1..points.len() {
            if dist(&points[i], &points[j]) <= 3 {
                dset.union(i, j);
            }
        }
    }

    dset.nsets
}

pub fn run(input: &str) {
    let part1 = solve(input);
    println!("the solution is {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        const EXAMPLE : &'static str = "-1,2,2,0\n0,0,2,-2\n";
        assert_eq!(parse_input(EXAMPLE), vec![[-1,2,2,0], [0,0,2,-2]]);
    }

    #[test]
    fn example1() {
        const EXAMPLE : &'static str = "\
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0
";
        assert_eq!(2, solve(EXAMPLE));
    }

    #[test]
    fn example2() {
        const EXAMPLE : &'static str = "\
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
";
        assert_eq!(4, solve(EXAMPLE));
    }

    #[test]
    fn example3() {
        const EXAMPLE : &'static str = "\
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2
";
        assert_eq!(3, solve(EXAMPLE));
    }

    #[test]
    fn example4() {
        const EXAMPLE : &'static str = "\
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2
";
        assert_eq!(8, solve(EXAMPLE));
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day25.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day25.txt"),
                   format!("{:?}", x));
    }
}
