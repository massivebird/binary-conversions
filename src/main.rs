use binary_conversions::{
    to_ones_complement,
    to_twos_complement
};

fn main() {
    let n: i32 = -35;
    println!("Evaluating decimal {n}...");
    println!("1's complement: {:#b}", to_ones_complement(n));
    println!("2's complement: {:#b}", to_twos_complement(n));
}
