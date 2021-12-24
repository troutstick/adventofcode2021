pub fn sol(input: &str) {
    let mut age_to_num_fish = [0; 9];
    let input = input.trim().split(",").map(|s| s.parse::<usize>().unwrap());

    for age in input {
        age_to_num_fish[age] += 1;
    }
    for _ in 0..80 {
        age_to_num_fish = advance_ages(age_to_num_fish);
    }
    println!(
        "The answer to Day 6 Part 1 is {}",
        age_to_num_fish.iter().sum::<u64>()
    );

    for _ in 80..256 {
        age_to_num_fish = advance_ages(age_to_num_fish);
    }
    println!(
        "The answer to Day 6 Part 2 is {}",
        age_to_num_fish.iter().sum::<u64>()
    );
}

fn advance_ages(prev_day: [u64; 9]) -> [u64; 9] {
    let mut next_day = [0; 9];
    for i in 1..prev_day.len() {
        next_day[i - 1] = prev_day[i];
    }
    next_day[6] += prev_day[0];
    next_day[8] += prev_day[0];
    next_day
}
