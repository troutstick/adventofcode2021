const MASK: i32 = 0xFFF;
const NUMS_WIDTH: usize = 12;

pub fn sol(input: &String) {
    let numify = |bin_str: &str| {
        bin_str
            .chars()
            .map(|i| i.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>()
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

    let input = input
        .trim()
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

    let sums = input.iter().fold([0; NUMS_WIDTH], bin_sum);

    let gamma = {
        let bin_str = sums.iter().map(sum_to_flag).collect::<String>();

        i32::from_str_radix(bin_str.as_str(), 2).unwrap()
    };

    let epsilon = !gamma & MASK;

    println!("The answer to Day 3 Part 1 is {}", gamma * epsilon);

    let o2_rating = vec_to_bin(reduce_part2(copy_vect(&input), 0, true));
    let co2_rating = vec_to_bin(reduce_part2(copy_vect(&input), 0, false));
    println!("The answer to Day 3 Part 2 is {}", o2_rating * co2_rating);
}

fn vec_to_bin(bin_vec: Vec<usize>) -> usize {
    let bin_value = |(index, val): (usize, &usize)| -> usize { *val * (1 << index) };
    bin_vec.iter().rev().enumerate().map(bin_value).sum()
}

fn reduce_part2(list_nums: Vec<Vec<usize>>, index: usize, is_most_common: bool) -> Vec<usize> {
    if index >= NUMS_WIDTH {
        panic!("baddo");
    }

    let num_to_match = get_most_or_least_common(&list_nums, index, is_most_common);
    let list_nums = list_nums
        .iter()
        .filter(|n| n[index] == num_to_match)
        .map(|n| n.clone())
        .collect::<Vec<Vec<usize>>>();
    // println!("woo we have {} left", list_nums.len());
    if list_nums.len() == 1 {
        list_nums[0].clone()
    } else {
        reduce_part2(list_nums, index + 1, is_most_common)
    }
}

fn get_most_or_least_common(
    list_nums: &Vec<Vec<usize>>,
    index: usize,
    is_most_common: bool,
) -> usize {
    let get_ith = |v: &Vec<usize>| -> usize { v[index] };

    let filter_ones = |i: &usize| *i == 1;
    let num_ones = list_nums.iter().map(get_ith).filter(filter_ones).count();
    let num_zeros = list_nums.len() - num_ones;
    if num_ones >= num_zeros {
        if is_most_common {
            1
        } else {
            0
        }
    } else {
        if is_most_common {
            0
        } else {
            1
        }
    }
}

fn copy_vect(v: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut ret = Vec::with_capacity(v.len());
    for bin_vec in v {
        ret.push(bin_vec.clone());
    }
    ret
}
