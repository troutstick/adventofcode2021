use std::collections::HashMap;

/// Consists of two endpoints of a line.
#[derive(Debug)]
struct Line {
    segments: [[usize; 2]; 2],
}

impl Line {
    fn new(s: &str) -> Line {
        let mut segments = [[0; 2]; 2];
        for (i, seg) in s.split("->").enumerate() {
            for (j, coord) in seg
                .split(",")
                .map(|s| s.trim())
                .map(|s| s.parse::<usize>().unwrap())
                .enumerate()
            {
                segments[i][j] = coord;
            }
        }
        Line { segments }
    }

    fn add_points_to_counter(&self, counter: &mut HashMap<Coord, usize>) {
        let start_x = self.segments[0][0];
        let end_x = self.segments[1][0];
        let start_y = self.segments[0][1];
        let end_y = self.segments[1][1];

        let (min_x, max_x, min_y, max_y) = {
            (
                start_x.min(end_x),
                start_x.max(end_x),
                start_y.min(end_y),
                start_y.max(end_y),
            )
        };

        if min_x == max_x {
            for y in min_y..=max_y {
                let coord = Coord(min_x, y);
                *counter.entry(coord).or_insert(0) += 1;
            }
        } else if min_y == max_y {
            for x in min_x..=max_x {
                let coord = Coord(x, min_y);
                *counter.entry(coord).or_insert(0) += 1;
            }
        } else {
            // Return an iterator from a start to end value; reverse it if necessary
            fn create_iter(start: usize, end: usize) -> Box<dyn Iterator<Item = usize>> {
                if start < end {
                    Box::new(start..=end)
                } else {
                    Box::new((end..=start).rev())
                }
            }

            for (x, y) in create_iter(start_x, end_x).zip(create_iter(start_y, end_y)) {
                let coord = Coord(x, y);
                *counter.entry(coord).or_insert(0) += 1;
            }
        }
    }

    fn is_not_diagonal(&self) -> bool {
        return self.segments[0][0] == self.segments[1][0]
            || self.segments[0][1] == self.segments[1][1];
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Coord(usize, usize);

pub fn sol(input: &String) {
    let input = input
        .trim()
        .split("\n")
        .map(Line::new)
        .collect::<Vec<Line>>();

    let mut no_diagonals_cnt = HashMap::new();
    let mut cnt = HashMap::new();
    for line_segment in input {
        if line_segment.is_not_diagonal() {
            line_segment.add_points_to_counter(&mut no_diagonals_cnt);
        }
        line_segment.add_points_to_counter(&mut cnt);
    }

    let part1 = no_diagonals_cnt
        .iter()
        .filter(|(_, count)| **count > 1)
        .count();

    let part2 = cnt.iter().filter(|(_, count)| **count > 1).count();

    println!("The answer to Day 5 Part 1 is {}", part1);
    println!("The answer to Day 5 Part 2 is {}", part2);
}
