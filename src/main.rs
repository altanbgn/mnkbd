use std::collections::HashMap;
use std::char;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    value: String,
}

// АБВГДЕЁЖЗИЙКЛМНОӨПРСТУҮФХЦЧШЩЫЬЭЮЯ
// абвгдеёжзийклмноөпрстуүфхцчшщыьэюя
const TRANSFER_DICTIONARY: HashMap<char, char> = HashMap::from([
    (char::from('A'), char::from('А')),
    // ("А", "А"),
    // ("А", "А"),
    // ("А", "А"),
    // ("А", "А"),
    // ("А", "А"),
    // ("А", "А"),
]);

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}

fn translate(value: &str) {
    let new_value: String = String::from("");

    value.chars().into_iter().for_each(|character| {
        character.
    })

}
