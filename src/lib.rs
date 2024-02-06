// TODO: ask if I can use native binary representations (do NOT make this a GitHub issue)

// Binary representations of integers are in twos complement.
// That's confusing! So we convert the number to standard [un]signed magnitude
// before making conversions (if necessary).

pub fn run(n: i128) {
    let e32_output = match to_excess(32, n) {
        Ok(bit_string) => bit_string,
        Err(msg) => msg
    };

    let e64_output = match to_excess(64, n) {
        Ok(bit_string) => bit_string,
        Err(msg) => msg
    };

    println!("Evaluating decimal {n}...");
    println!("Unsigned:       {}", build_unsigned_bit_string(n));
    println!("1's complement: {}", to_ones_complement(n));
    println!("2's complement: {}", to_twos_complement(n));
    println!("Excess-32:      {e32_output}");
    println!("Excess-64:      {e64_output}");
}

/// A dummy, lightweight, non-`clap` main function.
/// I have to demonstrate this code in class, but Rust Playground
/// does not support command line arguments.
/// This main function expects input via stdin.
pub fn dummy_main() {
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
    let unsigned_bit_string: String = build_unsigned_bit_string(n);

    if !n.is_negative() { return unsigned_bit_string }

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
    if !n.is_negative() { return build_unsigned_bit_string(n) }

    let as_ones_comp = to_ones_complement(n);

    // gotta add 1 to 1c form
    let magnitude_part = &as_ones_comp[1..];

    let Some(position_of_smallest_zero) = magnitude_part.rfind('0') else {
        todo!();
    };

    let mut working_string = String::new();

    for (i, value) in magnitude_part.chars().enumerate() {
        dbg!((i, value));
        if i == position_of_smallest_zero { working_string.push('1'); continue; }
        if i > position_of_smallest_zero  { working_string.push('0') ; continue; }
        if i < position_of_smallest_zero  { working_string.push(value) }
    }

    format!("1{working_string}")
}

fn build_unsigned_bit_string(n: i128) -> String {
    let n = n.abs();

    if n == 0 { return "0".to_string(); }

    // highest power of 2 that "fits in" n,
    // n of 65 returns value of 6
    let num_bits = {
        let mut i: u32 = 0;
        loop {
            if 2i128.pow(i) > n { break i };
            i += 1;
        }
    };

    let mut remaining_value = n;

    {
        let mut working_bit_string = String::new();

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
    if n.abs() > e {
        return Err(format!("input {n} too large for Excess-{e}"))
    }

    let total_bits = i128::ilog2(e) as usize + 1;
    let unpadded_bit_string = build_unsigned_bit_string(n + e);
    
    let padding = "0".repeat(total_bits - unpadded_bit_string.len());

    Ok(format!("{padding}{unpadded_bit_string}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radix_test() {
        assert_eq!(i128::from_str_radix("11011101", 2).unwrap(), 221);
    }

    #[test]
    fn build_unsigned_bit_string_zero() {
        assert_eq!(build_unsigned_bit_string(0), "0")
    }

    #[test]
    fn build_unsigned_bit_string_negative() {
        assert_eq!(build_unsigned_bit_string(-90), "1011010")
    }

    #[test]
    fn build_unsigned_bit_string_positive() {
        assert_eq!(build_unsigned_bit_string(37), "100101")
    }

    #[test]
    fn ones_complement_zero() {
        assert_eq!(to_ones_complement(0), "0".to_string());
    }

    #[test]
    fn ones_complement_positive() {
        assert_eq!(to_ones_complement(25), "11001".to_string());
        assert_eq!(to_ones_complement(35), "100011".to_string());
    }

    #[test]
    fn ones_complement_negative() {
        assert_eq!(to_ones_complement(-35), "1011100".to_string());
        assert_eq!(to_ones_complement(-90), "10100101".to_string());
        assert_eq!(to_ones_complement(-22), "101001".to_string());
        assert_eq!(to_ones_complement(-42), "1010101".to_string());
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
