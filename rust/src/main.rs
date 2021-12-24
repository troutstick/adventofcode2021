use std::fs;

const INPUTS_DIR: &str = "../inputs";

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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
            "day4.txt" => day4::sol(&input),
            "day5.txt" => day5::sol(&input),
            "day5test.txt" => (),
            "day6.txt" => day6::sol(&input),
            "day7.txt" => day7::sol(&input),
            _ => panic!("Solution not implemented yet for {}", filename),
        }
    }
}
