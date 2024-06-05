use std::io::BufRead;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(default_value = "challenges/c03/large_text_file.txt")]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let f = std::fs::File::open(&args.file).unwrap();
    let buf_reader = std::io::BufReader::new(f);

    let mut total = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();

        total += count_as_in_line(line);
    }

    println!("Total words: {}", total);
}

fn count_as_in_line(s: String) -> usize {
    s.split_whitespace()
        .map(|word| count_as_in_word(word.to_string()))
        .sum()
}

fn count_as_in_word(word: String) -> usize {
    word.chars().filter(|c| *c == 'a').count()
}
