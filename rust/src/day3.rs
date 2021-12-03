const MASK: i32 = 0xFFF;
const NUMS_WIDTH: usize = 12;

pub fn sol(input: &String) {

    let numify = |bin_str: &str| {
        bin_str.chars().map(|i| i.to_digit(10).unwrap() as usize).collect::<Vec<usize>>()
    };

    let bin_sum = |sum: [usize; NUMS_WIDTH], nums: &Vec<usize>| {
        [
            sum[0] + nums[0],
            sum[1] + nums[1],
            sum[2] + nums[2],
            sum[3] + nums[3],
            sum[4] + nums[4],
            sum[5] + nums[5],
            sum[6] + nums[6],
            sum[7] + nums[7],
            sum[8] + nums[8],
            sum[9] + nums[9],
            sum[10] + nums[10],
            sum[11] + nums[11],
        ]
    };

    let input = input.trim()
        .split("\n")
        .map(numify)
        .collect::<Vec<Vec<usize>>>();
    
    let sum_to_flag = |sum: &usize| {
        if *sum > input.len() / 2 {
            '1'
        } else {
            '0'
        }
    };

    let sums = input.iter()
        .fold([0; NUMS_WIDTH], bin_sum);

    let gamma = {
        let bin_str = sums.iter()
            .map(sum_to_flag)
            .collect::<String>();

        println!("binstr: {}", bin_str);

        i32::from_str_radix(bin_str.as_str(), 2).unwrap()
    };

    let o2_rating = input.iter();
    let co2_rating = input.iter();

    for i in 0..NUMS_WIDTH {
        let is_most_common = |bin_input: &&Vec<usize>| {
            bin_input[i] == if sums[i] >= input.len() / 2 {
                1
            } else {
                0
            }
        };
        let is_least_common = |bin_input: &&Vec<usize>| {
            bin_input[i] == if sums[i] <= input.len() / 2 {
                1
            } else {
                0
            }
        };
        o2_rating = o2_rating.filter(is_most_common);
        co2_rating = co2_rating.filter(is_least_common);
    }


    println!("The answer to Day 3 Part 1 is {}", gamma * (!gamma & MASK));
    
}