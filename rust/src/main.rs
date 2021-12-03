use std::fs;

const INPUTS_DIR: &str = "../inputs";

mod day1;
mod day2;
mod day3;

fn main() {
    let paths = fs::read_dir(INPUTS_DIR).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let input = fs::read_to_string(&path).unwrap();
        match filename {
            "day1.txt" => day1::sol(&input),
            "day2.txt" => day2::sol(&input),
            "day3.txt" => day3::sol(&input),
            _ => panic!("Solution not implemented yet for {}", filename),
        }
    }
}
