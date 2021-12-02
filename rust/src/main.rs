use std::fs;

const INPUTS_DIR: &str = "../inputs";

fn main() {
    let paths = fs::read_dir(INPUTS_DIR).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let input = fs::read_to_string(&path).unwrap();
        match filename {
            "day1.txt" => day1(&input),
            "day2.txt" => day2(&input),
            _ => panic!("Solution not implemented yet for {}", filename),
        }
    }
}

fn day2(input: &String) {
    enum Dir {
        Up, Down, Forward
    }
        
    let translate_instructions = |n: Vec<&str>| {
        let dir = n[0];
        let num = n[1];
        let dir = match dir {
            "forward" => Dir::Forward,
            "up" => Dir::Up,
            "down" => Dir::Down,
            _ => panic!("unknown instruction"),
        };
        (dir, num.parse::<i32>().unwrap())
    };

    let input = input.trim()
        .split("\n")
        .map(|instr| { instr.split_ascii_whitespace().collect::<Vec<&str>>() })
        .map(translate_instructions)
        .collect::<Vec<(Dir, i32)>>(); 

    let part1 = |(pos,depth), (dir, delta): &(Dir, i32)| {
        match dir {
            Dir::Forward => (pos+delta, depth),
            Dir::Up => (pos, depth-delta),
            Dir::Down => (pos, depth+delta),
        }
    };
    let part2 = |(pos, depth, aim), (dir, delta): &(Dir, i32)| {
        match dir {
            Dir::Forward => (pos+delta, depth+(aim*delta), aim),
            Dir::Up => (pos, depth, aim-delta),
            Dir::Down => (pos, depth, aim+delta),
        }
    };

    let (pos1, depth1) = input.iter()
        .fold((0,0), part1);
    let (pos2, depth2, _) = input.iter()
        .fold((0,0,0), part2);

    println!("The answer for Day 2 Part 1 is {}", pos1*depth1);
    println!("The answer for Day 2 Part 2 is {}", pos2*depth2);

}

fn day1(input: &String) {
    let input = input.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let part1 =  || {
        let mut ans = 0;
        let mut prev = input[0];
        for curr in input.iter().skip(1) {
            if *curr > prev {
                ans += 1;
            }
            prev = *curr;
        }
        println!("The answer to Day 1 Part 1 is {}", ans);
    };
    
    let part2 = || {
        let mut prev = input.iter().take(3).sum::<i64>();
        let mut ans = 0;
        for i in 3..input.len() {
            let curr = prev + input[i] - input[i-3];
            if curr > prev {
                ans += 1;
            }
            prev = curr;
        }
        println!("The answer to Day 1 Part 2 is {}", ans);
    };

    part1();
    part2();
}