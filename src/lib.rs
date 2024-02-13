// Author: Garrett Drake
// Project: Signed Integer Representation
// Submission Date: 12 February 2024
// Description:
//
// This project allows users to convert decimal values to and from the
// following 8-bit binary notations:
// 
// 1) Signed magnitude
// 2) One's complement
// 3) Two's complement
// 4) Excess-128

use std::fmt::Display;

/// Non-interactive run. Can be omitted from Rust Playground.
pub fn run(n: i32) {
    assert!(n >= -128 && n <= 127, "Outside of range (min: -128, max: 127)");

    println!("Evaluating decimal {n}...");
    println!("Signed magnitude:  {}", unpack(to_signed(n)));
    println!("Ones complement:   {}", unpack(to_ones_complement(n)));
    println!("Twos complement:   {}", unpack(to_twos_complement(n)));
    println!("Excess-128:        {}", unpack(to_excess(128, n)));
}

/// An interactive version of the program.
/// Rust Playground does not support command line arguments, so
/// an interactive mode is necessary.
pub fn main() {
    loop {
        println!("Choose a mode:");
        println!("(1) Decimal to 8-bit binary");
        println!("(2) 8-bit string to decimal");
        println!("(3) Quit");
        let mode: i32 = loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if let Ok(n) = input.trim().parse::<i32>() {
                if n != 1 && n != 2 && n != 3 { continue }
                break n;
            }
            println!("Please enter a valid mode.");
        };

        match mode {
            1 => interactive_to_binary(),
            2 => interactive_to_decimal(),
            3 => break,
            _ => unreachable!(),
        }

        println!();
    }

    println!("Goodbye!");
}

pub fn interactive_to_binary() {
    println!("Enter a decimal number to convert to binary:");
    let n: i32 = loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if let Ok(n) = input.trim().parse::<i32>() {
            if (-128..=127).contains(&n) { break n }
        }
        println!("Please enter a valid integer. (min -128, max 127)");
    };

    println!("Evaluating decimal {n}...");
    println!("Signed magnitude: {}", pad(8, &unpack(to_signed(n))));
    println!("Ones complement:  {}", unpack(to_ones_complement(n)));
    println!("Twos complement:  {}", unpack(to_twos_complement(n)));
    println!("Excess-128:       {}", unpack(to_excess(128, n)));
}

fn interactive_to_decimal() {
    println!("Enter a bit string to convert to decimal:");

    let is_valid_8_bit_binary_string = |s: &str| -> bool {
        s.trim().chars().all(|c| c == '1' || c == '0') && s.trim().len() == 8
    };

    let bit_string: String = loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if is_valid_8_bit_binary_string(&input) {
            break input.trim().to_string();
        };

        println!("Please enter a valid 8-bit bit string.");
    };

    println!("Evaluating bit string {bit_string} as different notations...");
    println!("Signed magnitude: {}", from_signed(&bit_string));
    println!("Ones complement:  {}", from_ones_complement(&bit_string));
    println!("Twos complement:  {}", unpack(from_twos_complement(&bit_string)));
    println!("Excess-128:       {}", unpack(from_excess_128(&bit_string)));
}

/// Converts a decimal to its 8-bit signed binary form.
fn to_signed(n: i32) -> Result<String, String> {
    validate_number(n, -127, 127)?;

    if !n.is_negative() { return Ok(pad(8, &to_unsigned_unpadded(n))) }

    Ok(format!("1{}", pad(7, &to_unsigned_unpadded(n))))
}

/// Converts a decimal to its 8-bit unsigned binary form, which is NOT
/// padded to any number of bits.
fn to_unsigned_unpadded(n: i32) -> String {
    let n = n.abs();

    if n == 0 { return "0".to_string(); }

    // let num_bits = 8;
    let num_bits = {
        let mut i: u32 = 0;
        loop {
            if 2i32.pow(i) > n { break i };
            i += 1;
        }
    };

    {
        let mut working_bit_string = String::new();

        let mut remaining_value = n;

        for place_value in (0..num_bits).rev().map(|v| 2i32.pow(v)) {
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

/// Converts a decimal to its 8-bit ones complement binary form.
fn to_ones_complement(n: i32) -> Result<String,String> {
    validate_number(n, -127, 127)?;
    let unsigned_bit_string: String = to_unsigned_unpadded(n);

    if !n.is_negative() {
        // only attach sign bit `0` if the input is nonzero
        match unsigned_bit_string.as_str() {
            "0" => return Ok(pad(8, "0")),
            _ => return Ok(pad(8, &unsigned_bit_string)),
        }
    }

    Ok(flip_bits(&pad(8, &unsigned_bit_string)))
}

/// Converts a decimal to its 8-bit twos complement binary form.
fn to_twos_complement(n: i32) -> Result<String, String> {
    validate_number(n, -128, 127)?;

    if !n.is_negative() { return Ok(to_ones_complement(n).unwrap()) }

    let Ok(as_ones_comp) = &to_ones_complement(n) else {
        // -128 is the ONLY input that is valid for 8-bit twos comp
        // but invalid for 8-bit ones comp; the ones comp function rejects
        // the input. I can just return the correct answer
        return Ok(String::from("10000000"))
    };

    let magnitude = &as_ones_comp[1..];

    let position_of_smallest_zero = magnitude.rfind('0').unwrap();

    // adding 1 to a bit string:
    // (1) Locate least-valued zero,
    // (2) Flip that zero to a one (1), then
    // (3) Flip all lesser ones
    let magnitude_plus_one = magnitude
        .chars()
        .enumerate()
        .map(|(i, value)| {
            if i == position_of_smallest_zero { return '1' }
            if i > position_of_smallest_zero  { return '0' }
            value
        })
        .collect::<String>();

    Ok(format!("1{magnitude_plus_one}"))
}

/// Converts a value `n` to excess `e`.
///
/// # Errors
///
/// Throws an error if the value `n` is too large for Excess-`e` notation.
///
/// Returns an error message detailing the incident.
pub fn to_excess(e: i32, n: i32) -> Result<String, String> {
    validate_number(n, -128, 255)?;

    let unpadded_bit_string = to_unsigned_unpadded(n + e);

    // let total_bits = i32::ilog2(e) as usize + 1;

    Ok(pad(8, &unpadded_bit_string))
}

/// Converts a bit string in unsigned form to its decimal value.
fn from_unsigned(bit_string: &str) -> i32 {
    bit_string
        .chars()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, char)| {
            if char == '0' { return acc; }
            acc + 2i32.pow(u32::try_from(index).unwrap())
        })
}

/// Converts a bit string in signed form to its decimal value.
fn from_signed(bit_string: &str) -> i32 {
    let sign_bit = bit_string.chars().next().unwrap();
    if sign_bit == '0' { return from_unsigned(bit_string) }

    let magnitude = &bit_string.chars().skip(1).collect::<String>();
    -from_unsigned(magnitude)
}

/// Converts a bit string in ones complement form to its decimal value.
fn from_ones_complement(bit_string: &str) -> i32 {
    let sign_bit = bit_string.chars().next().unwrap();
    // positive -> unchanged from unsigned form
    if sign_bit == '0' { return from_unsigned(bit_string) }

    // we can just flip the entire bit string...??
    let flipped = flip_bits(bit_string);

    -from_unsigned(&flipped)
}

/// Converts a bit string in twos complement form to its decimal value.
fn from_twos_complement(bit_string: &str) -> Result<i32, String> {
    let sign_bit = bit_string.chars().next().unwrap();
    // positive -> unchanged from unsigned form
    if sign_bit == '0' { return Ok(from_unsigned(bit_string)) }

    let magnitude = bit_string.chars().skip(1).collect::<String>();

    let Some(position_of_smallest_one) = magnitude.rfind('1') else {
        // -128 is the ONLY case in which there is a negative number with no
        // `1` bits in the unsigned magnitude
        return Ok(-128);
    };

    // subtract one from the bit string, then flip bits
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

    Ok(
        -from_unsigned(&flipped_magnitude)
    )
}

/// Converts a bit string in excess-128 form to its decimal value.
fn from_excess_128(bit_string: &str) -> Result<i32, String> {
    let maximum_bits = 8;
    if bit_string.len() > maximum_bits {
        return Err(format!("too many bits: maximum of {maximum_bits}"));
    }

    Ok(from_unsigned(bit_string) - 128)
}

/// Flips all bits in a bit string.
fn flip_bits(bit_string: &str) -> String {
    bit_string.chars()
        .map(|c| {
            if c == '0' { return '1' }
            '0'
        })
        .collect::<String>()
}

/// Pads a bit string so that it expresses the same value in 
/// a specified number of bits.
fn pad(num_bits: usize, bit_string: &str) -> String {
    if num_bits < bit_string.len() { return bit_string.to_string() };
    let padding = "0".repeat(num_bits - bit_string.len());
    format!("{padding}{bit_string}")
}

/// Verifies that a number is within the inclusive range `min..=max`.
///
/// # Errors
///
/// Throws an error if the number is not within the inclusive range.
///
/// Returns an informative error message.
fn validate_number(n: i32, min: i32, max: i32) -> Result<(), String> {
    if !(min..=max).contains(&n) {
        return Err(
            format!("outside acceptable range: min = {min}, max = {max}")
        )
    }
    Ok(())
}

/// Returns a string of the value contained within a `Result<T, E>` type,
/// whether `Ok` or `Err`.
fn unpack<T, E>(r: Result<T, E>) -> String
where
    T: Display,
    E: Display,
{
    match r {
        Ok(value) => value.to_string(),
        Err(msg) => msg.to_string(),
    }
}

#[cfg(test)]
// 2s comp: https://www.exploringbinary.com/twos-complement-converter/
mod tests {
    use super::*;

    #[test]
    fn to_signed_negative() {
        assert_eq!(to_signed(-30), Ok("10011110".to_string()));
    }

    #[test]
    fn to_signed_positive() {
        assert_eq!(to_signed(30), Ok("00011110".to_string()));
    }

    #[test]
    fn to_signed_zero() {
        assert_eq!(to_signed(0), Ok("00000000".to_string()));
    }

    #[test]
    fn to_ones_complement_zero() {
        assert_eq!(to_ones_complement(0), Ok("00000000".to_string()));
    }

    #[test]
    fn to_ones_complement_positive() {
        assert_eq!(to_ones_complement(7),   Ok("00000111".to_string()));
        assert_eq!(to_ones_complement(25),  Ok("00011001".to_string()));
        assert_eq!(to_ones_complement(35),  Ok("00100011".to_string()));
        assert_eq!(to_ones_complement(127), Ok("01111111".to_string()));
    }

    #[test]
    fn to_ones_complement_negative() {
        assert_eq!(to_ones_complement(-35),  Ok("11011100".to_string()));
        assert_eq!(to_ones_complement(-90),  Ok("10100101".to_string()));
        assert_eq!(to_ones_complement(-22),  Ok("11101001".to_string()));
        assert_eq!(to_ones_complement(-32),  Ok("11011111".to_string()));
        assert_eq!(to_ones_complement(-42),  Ok("11010101".to_string()));
        assert_eq!(to_ones_complement(-127), Ok("10000000".to_string()));
    }

    #[test]
    fn to_twos_complement_positive() {
        assert_eq!(to_twos_complement(7),   Ok("00000111".to_string()));
        assert_eq!(to_twos_complement(25),  Ok("00011001".to_string()));
        assert_eq!(to_twos_complement(126), Ok("01111110".to_string()));
        assert_eq!(to_twos_complement(127), Ok("01111111".to_string()));
    }

    #[test]
    fn to_twos_complement_zero() {
        assert_eq!(to_twos_complement(0), Ok("00000000".to_string()));
    }

    #[test]
    fn to_twos_complement_negative() {
        assert_eq!(to_twos_complement(-2),   Ok("11111110".to_string()));
        assert_eq!(to_twos_complement(-32),  Ok("11100000".to_string()));
        assert_eq!(to_twos_complement(-42),  Ok("11010110".to_string()));
        assert_eq!(to_twos_complement(-90),  Ok("10100110".to_string()));
        assert_eq!(to_twos_complement(-128), Ok("10000000".to_string()));
        assert_eq!(to_twos_complement(-127), Ok("10000001".to_string()));
        assert_eq!(to_twos_complement(-126), Ok("10000010".to_string()));
    }

    #[test]
    fn to_excess_128_positive() {
        assert_eq!(to_excess(128, 35), Ok("10100011".to_string()));
    }

    #[test]
    fn to_excess_128_zero() {
        assert_eq!(to_excess(128, 0), Ok("10000000".to_string()));
    }

    #[test]
    fn to_excess_128_negative() {
        assert_eq!(to_excess(128, -22), Ok("01101010".to_string()));
        assert_eq!(to_excess(128, -37), Ok("01011011".to_string()));
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
        assert_eq!(from_twos_complement("01111110"), Ok(126));
        assert_eq!(from_twos_complement("01111111"), Ok(127));
    }

    #[test]
    fn from_twos_complement_negative() {
        assert_eq!(from_twos_complement("11011011"), Ok(-37));
        assert_eq!(from_twos_complement("10000000"), Ok(-128));
        assert_eq!(from_twos_complement("10000010"), Ok(-126));
    }

    #[test]
    fn from_twos_complement_zero() {
        assert_eq!(from_twos_complement("0"), Ok(0));
    }

    #[test]
    fn from_ones_complement_positive() {
        assert_eq!(from_ones_complement("00000001"), 1);
        assert_eq!(from_ones_complement("01111111"), 127);
    }

    #[test]
    fn from_ones_complement_negative() {
        assert_eq!(from_ones_complement("11111110"), -1);
        assert_eq!(from_ones_complement("11111101"), -2);
        assert_eq!(from_ones_complement("11111100"), -3);
        assert_eq!(from_ones_complement("10000000"), -127);
        assert_eq!(from_ones_complement("10000001"), -126);
    }

    #[test]
    fn from_ones_complement_zero() {
        assert_eq!(from_ones_complement("00000000"), 0);
    }

    #[test]
    fn from_unsigned_positive() {
        assert_eq!(from_unsigned("110"), 6);
        assert_eq!(from_unsigned("00000001"), 1);
        assert_eq!(from_unsigned("11111111"), 255);
    }

    #[test]
    fn from_unsigned_zero() {
        assert_eq!(from_unsigned("00000000"), 0);
    }

    #[test]
    fn from_excess_128_zero() {
        assert_eq!(from_excess_128("00000000"), Ok(-128));
    }

    #[test]
    fn from_excess_128_positive() {
        assert_eq!(from_excess_128("11111111"), Ok(127));
        assert_eq!(from_excess_128("11111110"), Ok(126));
    }

    #[test]
    fn from_excess_128_negative() {
        assert_eq!(from_excess_128("00000000"), Ok(-128));
        assert_eq!(from_excess_128("00000001"), Ok(-127));
    }
}
