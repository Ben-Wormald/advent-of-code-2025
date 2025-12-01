use clap::Parser;

mod input;
use input::get_input;

macro_rules! day {
    ($day:ident) => {
        mod solutions { pub mod $day; }
        use solutions::$day::*;

        const INPUT: &str = stringify!($day);
    };
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    part: String,
    #[arg(short, long)]
    test: bool,
}

day!(day_00);

fn main() {
    dotenv::dotenv().ok();
    let args = Args::parse();

    let input = get_input(INPUT, args.test);

    let solution = match args.part.as_str() {
        "1" | "one" => solve_part_one(&input),
        "2" | "two" => solve_part_two(&input),
        _ => panic!(),
    };
    println!("{solution}");
}
