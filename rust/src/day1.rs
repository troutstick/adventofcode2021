pub fn sol(input: &String) {
    let input = input
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let part1 = || {
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
            let curr = prev + input[i] - input[i - 3];
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
