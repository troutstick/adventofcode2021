struct Board {
    nums: [[usize;5];5],
    marked: [[bool;5];5],
}

pub fn sol(input: &String) {
    let mut lines = input.trim()
        .split("\n\n");
        
    let drawn_nums = lines.next().unwrap()
        .trim()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
        
        
    let mut boards = lines
        .map(Board::new)
        .collect::<Vec<Board>>();

    println!("The answer to Day 4 Part 1 is {}", play_game(&mut boards, &drawn_nums).unwrap());
}

fn play_game(boards: &mut Vec<Board>, drawn_nums: &Vec<usize>) -> Option<usize> {
    for n in drawn_nums {
        for b in boards.iter_mut() {
            b.play(*n);
            if b.is_won() {
                return Some(b.get_score(*n));
            }
        }
    }
    None
}


impl Board {
    fn get_score(&self, winning_n: usize) -> usize {
        let mut sum = 0;
        for (i, row) in self.nums.iter().enumerate() {
            for (j, n) in row.iter().enumerate() {
                if !self.marked[i][j] {
                    sum += *n;
                }
            }
        }
        sum * winning_n
    }

    fn is_won(&self) -> bool {
        let is_won_col = |i: usize| {
            self.marked.iter().all(|r| r[i])
        };

        let is_won_row = |i: usize| {
            self.marked[i].iter().all(|w| *w)
        };
        
        (0..5).any(|i| is_won_row(i) || is_won_col(i))
    }

    fn play(&mut self, drawn_n: usize) {
        for (i, row) in self.nums.iter().enumerate() {
            for (j, n) in row.iter().enumerate() {
                if drawn_n == *n {
                    self.marked[i][j] = true;
                }
            }
        };
    }

    fn new(s: &str) -> Board {
        let nums = {
            let nums = s.split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap());
            let mut arr = [[0;5];5];
            for (i, n) in nums.enumerate() {
                arr[i/5][i%5] = n;
            }
            arr
        };
        
        Board {
            nums,
            marked: [[false;5];5],
        }
    }
}


