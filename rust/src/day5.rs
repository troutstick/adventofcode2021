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
}

pub fn sol(input: &String) {
    let input = input
        .trim()
        .split("\n")
        .map(Line::new)
        .collect::<Vec<Line>>();
    
    
}
