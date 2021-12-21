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
        let (min, max, is_fixed_x) = if self.segments[0][0] == self.segments[1][0] {
            // we iterate over y
            let a = self.segments[0][1];
            let b = self.segments[1][1];
            (a.min(b), a.max(b), true)
        } else if self.segments[0][1] == self.segments[1][1] {
            // we iterate over x
            let a = self.segments[0][0];
            let b = self.segments[1][0];
            (a.min(b), a.max(b), false)
        } else {
            // unimplemented
            return;
        };

        for i in min..=max {
            let coord = if is_fixed_x {
                Coord(self.segments[0][0], i)
            } else {
                Coord(i, self.segments[0][1])
            };
            *counter.entry(coord).or_insert(0) += 1;
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Coord(usize, usize);

pub fn sol(input: &String) {
    let input = input
        .trim()
        .split("\n")
        .map(Line::new)
        .collect::<Vec<Line>>();

    println!("{:?}", input);

    let mut counter = HashMap::new();
    for line_segment in input {
        line_segment.add_points_to_counter(&mut counter);
    }

    let part1 = counter.iter().filter(|(coord, count)| **count > 1).count();
    println!("The answer to Day 5 Part 1 is {}", part1);
}
