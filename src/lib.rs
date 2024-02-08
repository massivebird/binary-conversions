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
    println!("Unsigned:       {}", unsigned_bit_string(n));
    println!("1's complement: {}", to_ones_complement(n));
    println!("2's complement: {}", to_twos_complement(n));
    println!("Excess-32:      {}", excess_output(32, n));
    println!("Excess-64:      {}", excess_output(64, n));
    println!("Excess-128:     {}", excess_output(128, n));
}

/// A dummy, lightweight, non-`clap` main function.
/// I have to demonstrate this code in class, but Rust Playground
/// does not support command line arguments.
/// This main function expects input via stdin.
pub fn main() {
    println!("Enter a number to convert to binary:");
    let n: i128 = loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if let Ok(n) = input.trim().parse::<i128>() {
            break n;
        }
    };

    run(n);
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

    let flipped_magnitudes: String = unsigned_bit_string.chars()
        .take(unsigned_bit_string.len())
        .map(|c| {
            if c == '0' { return '1' }
            '0'
        })
        .collect::<String>();

    format!("1{flipped_magnitudes}")
}

fn to_twos_complement(n: i128) -> String {
    // use 1c fn instead of unsigned fn to include the sign bit of zero
    if !n.is_negative() { return to_ones_complement(n) }

    // gotta add 1 to 1c form
    let magnitude_part = &to_ones_complement(n)[1..];

    let Some(position_of_smallest_zero) = magnitude_part.rfind('0') else {
        // all ones -> one followed by all zeroes?
        // do I need to do this? should it be done? is it ethical?
        todo!();
    };

    // adding 1 to a bit string goes like this:
    // (1) Locate least-valued zero,
    // (2) Flip that zero to a one (1), then
    // (3) Flip all ones to the right of that position.
    // I like the iterative approach.
    String::from("1") + &magnitude_part
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

    let total_bits = i128::ilog2(e) as usize + 1;
    let unpadded_bit_string = unsigned_bit_string(n + e);
    
    let padding = "0".repeat(total_bits - unpadded_bit_string.len());

    Ok(format!("{padding}{unpadded_bit_string}"))
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
}
