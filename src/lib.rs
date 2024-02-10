//! Author: Garrett Drake
//! Project: Signed Integer Representation
//! Submission Date: TBD
//! Description:
//!
//! This project allows users to convert decimal values to and from the
//! following 8-bit binary notations:
//! 
//! 1) Signed magnitude
//! 2) One's complement
//! 3) Two's complement
//! 4) Excess-128

/// Non-interactive run. Can be omitted from Rust Playground.
pub fn run(n: i32) {
    let excess_output = |e, n| {
        match to_excess(e, n) {
            Ok(bit_string) => bit_string,
            Err(msg) => msg
        }
    };

    println!("Evaluating decimal {n}...");
    // println!("Unsigned:          {}", unsigned_bit_string(n));
    // println!("Signed magnitude:  {}", to_signed(n));
    println!("Ones complement:   {}", to_ones_complement(n));
    // println!("Twos complement:   {}", to_twos_complement(n));
    // println!("Excess-32:       {}", excess_output(32, n));
    // println!("Excess-64:       {}", excess_output(64, n));
    println!("Excess-128:        {}", excess_output(128, n));
}

/// An interactive version of the program.
/// Rust Playground does not support command line arguments, so
/// an interactive mode is necessary.
pub fn main() {
    loop {
        println!("Choose a mode:");
        println!("(1) Binary to decimal");
        println!("(2) Decimal to binary");
        println!("(3) Quit");
        let mode: i32 = loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if let Ok(n) = input.trim().parse::<i32>() {
                if n != 1 && n != 2 && n != 3 { continue }
                break n;
            }
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
    println!("Enter a number to convert to binary:");
    let n: i32 = loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if let Ok(n) = input.trim().parse::<i32>() {
            break n;
        }
    };

    let excess_output = |e, n| {
        match to_excess(e, n) {
            Ok(bit_string) => bit_string,
            Err(msg) => msg
        }
    };

    let unpack = |r: Result<String, String>| {
        match r {
            Ok(a) => a,
            Err(msg) => msg,
        }
    };

    println!("Evaluating decimal {n}...");
    // println!("Unsigned:         {}", to_unsigned_unpadded(n));
    println!("Signed magnitude: {}", pad(8, &unpack(to_signed(n))));
    println!("Ones complement:  {}", to_ones_complement(n));
    println!("Twos complement:  {}", unpack(to_twos_complement(n)));
    // println!("Excess-32:      {}", excess_output(32, n));
    // println!("Excess-64:      {}", excess_output(64, n));
    println!("Excess-128:       {}", excess_output(128, n));
}

fn interactive_to_decimal() {
    println!("Enter a bit string to convert to decimal:");
    let bit_string: String = loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if !input.trim().chars().all(|c| c == '1' || c == '0') { continue };
        break input.trim().to_string();
    };

    let unpack = |r: Result<i32, String>| {
        match r {
            Ok(a) => a.to_string(),
            Err(msg) => msg,
        }
    };

    println!("Evaluating bit string {bit_string} as different notations...");
    // println!("Unsigned:         {}", from_unsigned(&bit_string));
    println!("Signed magnitude: {}", from_signed(&bit_string));
    println!("Ones complement:  {}", from_ones_complement(&bit_string));
    println!("Twos complement:  {}", unpack(from_twos_complement(&bit_string)));
    println!("Excess-128:       {}", unpack(from_excess_128(&bit_string)));
}

/// Converts a decimal to its 8-bit signed binary form.
fn to_signed(n: i32) -> Result<String, String> {
    let unsigned_max = 127;
    if n.abs() > unsigned_max {
        return Err(
            format!("Too large for 8 bits: max is +-{unsigned_max}")
        )
    }

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
fn to_ones_complement(n: i32) -> String {
    let unsigned_bit_string: String = to_unsigned_unpadded(n);

    if !n.is_negative() {
        // only attach sign bit `0` if the input is nonzero
        match unsigned_bit_string.as_str() {
            "0" => return pad(8, "0"),
            _ => return pad(8, &unsigned_bit_string),
        }
    }

    let flipped_magnitude: String = flip_bits(&unsigned_bit_string[1..]);

    // format!("1{}", pad(7, &flipped_magnitude))
    dbg!((n, &unsigned_bit_string));
    flip_bits(&pad(8, &unsigned_bit_string))
}

/// Converts a decimal to its 8-bit twos complement binary form.
fn to_twos_complement(n: i32) -> Result<String, String> {
    if n < -128 || n > 127 {
        return Err(
            format!("Outside acceptable range")
        )
    }

    if !n.is_negative() { return Ok(to_ones_complement(n)) }

    let magnitude = &to_ones_complement(n)[1..];

    let Some(position_of_smallest_zero) = magnitude.rfind('0') else {
        // the ONLY case in which there is a negative number with no
        // `1` bits in the magnitude is -128.
        return Ok(String::from("10000000"))
    };

    // adding 1 to a bit string goes like this:
    // (1) Locate least-valued zero,
    // (2) Flip that zero to a one (1), then
    // (3) Flip all ones to the right of that position.
    let ones_comp_plus_one = magnitude
        .chars()
        .enumerate()
        .map(|(i, value)| {
            if i == position_of_smallest_zero { return '1' }
            if i > position_of_smallest_zero  { return '0' }
            value
        })
        .collect::<String>();

    Ok(format!("1{ones_comp_plus_one}"))
}

/// Converts a value `n` to excess `e`.
///
/// # Errors
///
/// Throws an error if the value `n` is too large for Excess-`e` notation.
///
/// Returns an error message detailing the incident.
pub fn to_excess(e: i32, n: i32) -> Result<String, String> {
    if n.abs() > e - 1 {
        return Err(format!("input {n} too large for Excess-{e}"))
    }

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
            acc + 2i32.pow(index as u32)
        })
}

/// Converts a bit string in signed form to its decimal value.
fn from_signed(bit_string: &str) -> i32 {
    let sign_bit = bit_string.chars().nth(0).unwrap();
    if sign_bit == '0' { return from_unsigned(bit_string) }

    let magnitude = &bit_string.chars().skip(1).collect::<String>();
    -1 * from_unsigned(&magnitude)
}

/// Converts a bit string in ones complement form to its decimal value.
fn from_ones_complement(bit_string: &str) -> i32 {
    let sign_bit = bit_string.chars().nth(0).unwrap();
    // positive -> unchanged from unsigned form
    if sign_bit == '0' { return from_unsigned(bit_string) }

    // we can just flip the entire bit string...??
    // let magnitude = bit_string.chars().skip(1).collect::<String>();
    let flipped_magnitude = flip_bits(&bit_string);

    return -1 * from_unsigned(&flipped_magnitude);
}

/// Converts a bit string in twos complement form to its decimal value.
fn from_twos_complement(bit_string: &str) -> Result<i32, String> {
    let sign_bit = bit_string.chars().nth(0).unwrap();
    // positive -> unchanged from unsigned form
    if sign_bit == '0' { return Ok(from_unsigned(bit_string)) }

    let magnitude = bit_string.chars().skip(1).collect::<String>();

    let flipped_magnitude =
    if let Some(position_of_smallest_one) = magnitude.rfind('1') {
        // subtract one from the bit string
        flip_bits(&magnitude
            .chars()
            .enumerate()
            .map(|(i, value)| {
                if i == position_of_smallest_one { return '0' }
                if i > position_of_smallest_one  { return '1' }
                value
            })
            .collect::<String>()
        )
    } else {
        bit_string.to_string()
    };

    return Ok(
        -1 * from_unsigned(&flipped_magnitude)
    );
}

/// Converts a bit string in excess-128 form to its decimal value.
fn from_excess_128(bit_string: &str) -> Result<i32, String> {
    let maximum_bits = 8;
    if bit_string.len() > maximum_bits {
        return Err(format!("too many bits: maximum of {maximum_bits}"));
    }

    Ok(from_unsigned(&bit_string) - 128)
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

#[cfg(test)]
// 1's comp: https://www.compscilib.com/calculate/decimal-to-ones-complement
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
        assert_eq!(to_ones_complement(0), "00000000".to_string());
    }

    #[test]
    fn to_ones_complement_positive() {
        assert_eq!(to_ones_complement(7),   "00000111".to_string());
        assert_eq!(to_ones_complement(25),  "00011001".to_string());
        assert_eq!(to_ones_complement(35),  "00100011".to_string());
        assert_eq!(to_ones_complement(127), "01111111".to_string());
    }

    #[test]
    fn to_ones_complement_negative() {
        assert_eq!(to_ones_complement(-35),  "11011100".to_string());
        assert_eq!(to_ones_complement(-90),  "10100101".to_string());
        assert_eq!(to_ones_complement(-22),  "11101001".to_string());
        assert_eq!(to_ones_complement(-32),  "11011111".to_string());
        assert_eq!(to_ones_complement(-42),  "11010101".to_string());
        assert_eq!(to_ones_complement(-127), "10000000".to_string());
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
    }

    #[test]
    fn to_excess_64_positive() {
        assert_eq!(to_excess(64, 35), Ok("01100011".to_string()));
    }

    #[test]
    fn to_excess_64_zero() {
        assert_eq!(to_excess(64, 0), Ok("01000000".to_string()));
    }

    #[test]
    fn to_excess_64_negative() {
        assert_eq!(to_excess(64, -22), Ok("00101010".to_string()));
        assert_eq!(to_excess(64, -37), Ok("00011011".to_string()));
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
        assert_eq!(from_twos_complement("1011011"), Ok(-37));
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
        assert_eq!(from_ones_complement("110"), -1);
        assert_eq!(from_ones_complement("10000000"), -127);
    }

    #[test]
    fn from_ones_complement_zero() {
        assert_eq!(from_ones_complement("0"), 0);
    }

    #[test]
    fn from_unsigned_positive() {
        assert_eq!(from_unsigned("110"), 6);
        assert_eq!(from_unsigned("00000001"), 1);
        assert_eq!(from_unsigned("11111111"), 255);
    }

    #[test]
    fn from_unsigned_zero() {
        assert_eq!(from_unsigned("0"), 0);
    }
}
