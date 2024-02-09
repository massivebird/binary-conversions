//! Author: Garrett Drake
//! Project: Signed Integer Representation
//! Submission Date: TBD
//! Description:
//!
//! This project allows users to convert decimal to/from the following
//! binary notations:
//! 
//! 1) Unsigned
//! 2) One's complement
//! 3) Two's complement
//! 4) Excess-32
//! 5) Excess-64
//! 6) Excess-128

pub fn run(n: i128) {
    let excess_output = |e, n| {
        match to_excess(e, n) {
            Ok(bit_string) => bit_string,
            Err(msg) => msg
        }
    };

    println!("Evaluating decimal {n}...");
    println!("Unsigned:        {}", unsigned_bit_string(n));
    println!("Ones complement: {}", to_ones_complement(n));
    println!("Ones complement: {}", to_twos_complement(n));
    // println!("Excess-32:      {}", excess_output(32, n));
    // println!("Excess-64:      {}", excess_output(64, n));
    println!("Excess-128:      {}", excess_output(128, n));
}

pub fn run_to_binary() {
    println!("Enter a number to convert to binary:");
    let n: i128 = loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if let Ok(n) = input.trim().parse::<i128>() {
            break n;
        }
    };

    let excess_output = |e, n| {
        match to_excess(e, n) {
            Ok(bit_string) => bit_string,
            Err(msg) => msg
        }
    };

    println!("Evaluating decimal {n}...");
    println!("Unsigned:        {}", unsigned_bit_string(n));
    println!("Twos complement: {}", to_ones_complement(n));
    println!("Ones complement: {}", to_twos_complement(n));
    // println!("Excess-32:      {}", excess_output(32, n));
    // println!("Excess-64:      {}", excess_output(64, n));
    println!("Excess-128:      {}", excess_output(128, n));
}

fn run_to_decimal() {
    println!("Enter a bit string to convert to decimal:");
    let bit_string: String = loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if !input.trim().chars().all(|c| c == '1' || c == '0') { continue };
        break input.trim().to_string();
    };

    let ok_value_or_err_msg = |r: Result<i128, String>| {
        match r {
            Ok(a) => a.to_string(),
            Err(msg) => msg,
        }
    };

    println!("Evaluating bit string {bit_string} as different notations...");
    println!("Unsigned:          {}", from_unsigned(&bit_string));
    println!("Signed magnitude:  {}", from_signed(&bit_string));
    println!("Ones complement:   {}", from_ones_complement(&bit_string));
    println!("Twos complement:   {}", ok_value_or_err_msg(from_twos_complement(&bit_string)));
    println!("Excess-128:        {}", ok_value_or_err_msg(from_excess_128(&bit_string)));
}

fn from_excess_128(bit_string: &str) -> Result<i128, String> {
    let maximum_bits = 8;
    if bit_string.len() > maximum_bits {
        return Err(format!("too many bits: maximum of {maximum_bits}"));
    }

    let padding = "0".repeat(maximum_bits - bit_string.len());
    let bit_string = format!("{padding}{bit_string}");
    Ok(from_unsigned(&bit_string) - 128)
}

fn from_signed(bit_string: &str) -> i128 {
    let sign_bit = bit_string.chars().nth(0).unwrap();
    if sign_bit == '0' { return from_unsigned(bit_string) }

    let magnitude = &bit_string.chars().skip(1).collect::<String>();
    -1 * from_unsigned(&magnitude)
}

fn from_twos_complement(bit_string: &str) -> Result<i128, String> {
    let sign_bit = bit_string.chars().nth(0).unwrap();
    // positive -> unchanged from unsigned form
    if sign_bit == '0' { return Ok(from_unsigned(bit_string)) }

    let magnitude = bit_string.chars().skip(1).collect::<String>();

    // since we have to subtract 1 from this bit string,
    // 10000 -> 01111 (overflow??)
    if magnitude.chars().all(|c| c == '0') {
        return Err("not enough bits".to_string())
    }

    let Some(position_of_smallest_one) = magnitude.rfind('1') else {
        // do I need to do this? should it be done? is it ethical?
        todo!();
    };

    // flip the magnitude AFTER subtracting one, which is done by
    // flipping the smallest 1 bit, then flipping all lesser value bits!
    let flipped_magnitude = flip_bits(&magnitude
        .chars()
        .enumerate()
        .map(|(i, value)| {
            if i == position_of_smallest_one { return '0' }
            if i > position_of_smallest_one  { return '1' }
            value
        })
        .collect::<String>()
    );

    return Ok(
        -1 * from_unsigned(&flipped_magnitude)
    );
}

fn from_ones_complement(bit_string: &str) -> i128 {
    let sign_bit = bit_string.chars().nth(0).unwrap();
    // positive -> unchanged from unsigned form
    if sign_bit == '0' { return from_unsigned(bit_string) }

    let magnitude = bit_string.chars().skip(1).collect::<String>();
    let flipped_magnitude = flip_bits(&magnitude);

    return -1 * from_unsigned(&flipped_magnitude);
}

fn from_unsigned(bit_string: &str) -> i128 {
    bit_string
        .chars()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, char)| {
            if char == '0' { return acc; }
            acc + 2i128.pow(index as u32)
        })
}

/// A dummy, lightweight, non-`clap` main function.
/// I have to demonstrate this code in class, but Rust Playground
/// does not support command line arguments.
/// This main function expects input via stdin.
pub fn main() {
    println!("Converting (1) decimal to binary, or (2) binary to decimal?");
    let mode: i128 = loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if let Ok(n) = input.trim().parse::<i128>() {
            if n != 1 && n != 2 { continue }
            break n;
        }
    };

    match mode {
        1 => run_to_binary(),
        2 => run_to_decimal(),
        _ => unreachable!(),
    }
}

fn to_ones_complement(n: i128) -> String {
    let unsigned_bit_string: String = unsigned_bit_string(n);

    if !n.is_negative() {
        // only attach sign bit `0` if the input is nonzero
        match unsigned_bit_string.as_str() {
            "0" => return "0".to_string(),
            _ => return format!("0{unsigned_bit_string}"),
        }
    }

    let flipped_magnitude: String = flip_bits(&unsigned_bit_string);

    format!("1{flipped_magnitude}")
}

fn to_twos_complement(n: i128) -> String {
    // use 1c fn instead of unsigned fn to include the sign bit of zero
    if !n.is_negative() { return to_ones_complement(n) }

    // gotta add 1 to 1c form
    let magnitude = &to_ones_complement(n)[1..];

    let Some(position_of_smallest_zero) = magnitude.rfind('0') else {
        // all ones -> one followed by all zeroes?
        // do I need to do this? should it be done? is it ethical?
        todo!();
    };

    // adding 1 to a bit string goes like this:
    // (1) Locate least-valued zero,
    // (2) Flip that zero to a one (1), then
    // (3) Flip all ones to the right of that position.
    // I like the iterative approach.
    String::from("1") + &magnitude
        .chars()
        .enumerate()
        .map(|(i, value)| {
            if i == position_of_smallest_zero { return '1' }
            if i > position_of_smallest_zero  { return '0' }
            value
        })
        .collect::<String>()
}

fn unsigned_bit_string(n: i128) -> String {
    let n = n.abs();

    if n == 0 { return "0".to_string(); }

    let num_bits = {
        let mut i: u32 = 0;
        loop {
            if 2i128.pow(i) > n { break i };
            i += 1;
        }
    };

    {
        let mut working_bit_string = String::new();

        let mut remaining_value = n;

        for place_value in (0..num_bits).rev().map(|v| 2i128.pow(v)) {
            if place_value <= remaining_value {
                working_bit_string.push('1');
                remaining_value -= place_value;
                continue;
            }
            working_bit_string.push('0');
        }

        working_bit_string
    }
}

/// Converts a value `n` to excess `e`.
///
/// # Errors
///
/// Returns `Err(msg)`, where `msg` tells you retells what went wrong.
///
/// Throws an error if the value `n` is too large for Excess-`e` notation.
pub fn to_excess(e: i128, n: i128) -> Result<String, String> {
    if n.abs() > e - 1 {
        return Err(format!("input {n} too large for Excess-{e}"))
    }

    let unpadded_bit_string = unsigned_bit_string(n + e);

    let total_bits = i128::ilog2(e) as usize + 1;
    let padding = "0".repeat(total_bits - unpadded_bit_string.len());

    Ok(format!("{padding}{unpadded_bit_string}"))
}

fn flip_bits(bit_string: &str) -> String {
    bit_string.chars()
        .map(|c| {
            if c == '0' { return '1' }
            '0'
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_bit_string_zero() {
        assert_eq!(unsigned_bit_string(0), "0")
    }

    #[test]
    fn unsigned_bit_string_negative() {
        assert_eq!(unsigned_bit_string(-90), "1011010")
    }

    #[test]
    fn unsigned_bit_string_positive() {
        assert_eq!(unsigned_bit_string(37), "100101")
    }

    #[test]
    fn ones_complement_zero() {
        assert_eq!(to_ones_complement(0), "0".to_string());
    }

    #[test]
    fn ones_complement_positive() {
        assert_eq!(to_ones_complement(25), "011001".to_string());
        assert_eq!(to_ones_complement(35), "0100011".to_string());
        assert_eq!(to_ones_complement(7), "0111".to_string());
    }

    #[test]
    fn ones_complement_negative() {
        assert_eq!(to_ones_complement(-35), "1011100".to_string());
        assert_eq!(to_ones_complement(-90), "10100101".to_string());
        assert_eq!(to_ones_complement(-22), "101001".to_string());
        assert_eq!(to_ones_complement(-42), "1010101".to_string());
    }

    #[test]
    fn twos_complement_positive() {
        assert_eq!(to_twos_complement(25), "011001".to_string());
        assert_eq!(to_twos_complement(129), "010000001".to_string());
        assert_eq!(to_twos_complement(7), "0111".to_string());
    }

    #[test]
    fn twos_complement_zero() {
        assert_eq!(to_twos_complement(0), "0".to_string());
    }

    #[test]
    fn twos_complement_negative() {
        assert_eq!(to_twos_complement(-90), "10100110".to_string());
        assert_eq!(to_twos_complement(-2), "110".to_string());
        assert_eq!(to_twos_complement(-32), "1100000".to_string());
    }

    #[test]
    fn excess_64_positive() {
        assert_eq!(to_excess(64, 35), Ok("1100011".to_string()));
    }

    #[test]
    fn excess_64_zero() {
        assert_eq!(to_excess(64, 0), Ok("1000000".to_string()));
    }

    #[test]
    fn excess_64_negative() {
        assert_eq!(to_excess(64, -22), Ok("0101010".to_string()));
        assert_eq!(to_excess(64, -37), Ok("0011011".to_string()));
    }

    #[test]
    fn from_signed_positive() {
        assert_eq!(from_signed("01000"), 8);
    }

    #[test]
    fn from_signed_zero() {
        assert_eq!(from_signed("0"), 0);
    }

    #[test]
    fn from_signed_negative() {
        assert_eq!(from_signed("11000000"), -64);
    }

    #[test]
    fn from_twos_complement_positive() {
        assert_eq!(from_twos_complement("011000"), Ok(24));
    }

    #[test]
    fn from_twos_complement_negative() {
        assert_eq!(from_twos_complement("1011011"), Ok(-37));
    }

    #[test]
    fn from_twos_complement_zero() {
        assert_eq!(from_twos_complement("0"), Ok(0));
    }

    #[test]
    fn from_ones_complement_negative() {
        assert_eq!(from_ones_complement("110"), -1);
    }

    #[test]
    fn from_ones_complement_zero() {
        assert_eq!(from_ones_complement("0"), 0);
    }

    #[test]
    fn from_unsigned_positive() {
        assert_eq!(from_unsigned("110"), 6);
    }

    #[test]
    fn from_unsigned_zero() {
        assert_eq!(from_unsigned("0"), 0);
    }
}
