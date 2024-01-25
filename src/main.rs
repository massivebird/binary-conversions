use binary_conversions::{
    to_ones_complement,
    to_twos_complement,
    to_excess_64,
};
use clap::Arg;

fn main() {
    let matches = clap::command!()
        .arg(Arg::new("n")
            .required(true)
            .value_name("NUM")
            .value_parser(clap::value_parser!(i32))
            .help("Specify the decimal number to convert")
        )
        .get_matches();

    let n: i32 = *matches.get_one("n").unwrap();

    println!("Evaluating decimal {n}...");
    println!("1's complement: {:#b}", to_ones_complement(n));
    println!("2's complement: {:#b}", to_twos_complement(n));
    println!("Excess-64:      {:#b}", to_excess_64(n));
}
