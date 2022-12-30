use std::fs;
use std::io::BufRead;

fn main() {
    let file = "input.txt";

    let input = fs::File::open(file).unwrap();
    let reader = std::io::BufReader::new(input);

    for line in reader.lines() {
        let line = line.unwrap();
        let output = format!("- text - [{line}]({line})");
        println!("{output}");
    }
}
