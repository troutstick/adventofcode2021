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
            _ => panic!("Solution not implemented yet for {}", filename),
        }
    }
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