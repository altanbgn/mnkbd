use clap::Parser;

mod dictionary;
mod commands;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    value: String,
}

fn main() {
    let args = Args::parse();

    println!("{}", commands::translate::en_to_mn(args.value.as_str()));

    std::process::exit(0);
}

