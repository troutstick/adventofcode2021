pub fn sol(input: &str) {
    enum Dir {
        Up,
        Down,
        Forward,
    }

    let translate_instructions = |instr: &str| {
        let dir_num = instr.split_ascii_whitespace().collect::<Vec<&str>>();
        let dir = match dir_num[0] {
            "forward" => Dir::Forward,
            "up" => Dir::Up,
            "down" => Dir::Down,
            _ => panic!("unknown instruction"),
        };
        (dir, dir_num[1].parse::<i32>().unwrap())
    };

    let part1 = |(pos, depth), (dir, delta): &(Dir, i32)| match dir {
        Dir::Forward => (pos + delta, depth),
        Dir::Up => (pos, depth - delta),
        Dir::Down => (pos, depth + delta),
    };

    let part2 = |(pos, depth, aim), (dir, delta): &(Dir, i32)| match dir {
        Dir::Forward => (pos + delta, depth + (aim * delta), aim),
        Dir::Up => (pos, depth, aim - delta),
        Dir::Down => (pos, depth, aim + delta),
    };

    // Translate text into easily parseable instructions
    let input = input
        .trim()
        .split("\n")
        .map(translate_instructions)
        .collect::<Vec<(Dir, i32)>>();

    let (pos1, depth1) = input.iter().fold((0, 0), part1);
    println!("The answer for Day 2 Part 1 is {}", pos1 * depth1);

    let (pos2, depth2, _) = input.iter().fold((0, 0, 0), part2);
    println!("The answer for Day 2 Part 2 is {}", pos2 * depth2);
}
