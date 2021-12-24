pub fn sol(input: &str) {
    let input = {
        let mut v = input
            .trim()
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        v.sort();
        v
    };

    let cumulative_sum = {
        let mut v = Vec::with_capacity(input.len());
        let mut sum = 0;
        for n in &input {
            sum += *n;
            v.push(sum);
        }
        v
    };

    let midway_index = input.len() / 2;
    let midway_value = input[midway_index];

    println!("test {}", fuel_cost(10, &input, &cumulative_sum));
    println!("input: {:?}, cum_sum: {:?}", input, cumulative_sum);
}

fn fuel_cost(index: usize, input: &Vec<usize>, cum_sum: &Vec<usize>) -> usize {
    let num_right = cum_sum.len() - index - 1;
    let num_left = index; 

    // the cost of all indices to the right
    let cost_right = cum_sum[cum_sum.len() - 1] - cum_sum[index] - (num_right * input[index]);

    // distance from 0 minus the distance from 0 to the first element
    let cost_left = cum_sum[index] - (num_left * input[0]);

    cost_left + cost_right
}