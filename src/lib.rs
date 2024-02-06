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
    println!("1's complement: {:#b}", to_ones_complement(n));
    println!("2's complement: {:#b}", to_twos_complement(n));
    println!("Excess-32:      {}",    e32_output);
    println!("Excess-64:      {}",    e64_output);
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

fn to_ones_complement(n: i128) -> i128 {
    if n.is_positive() || n == 0 { return n; }

    let unsigned_bit_string: String = build_unsigned_bit_string(n);
    let num_unsigned_bits = unsigned_bit_string.len();

    let flipped_bit_string: String = unsigned_bit_string.chars()
        .take(num_unsigned_bits)
        .map(|c| {
            if c == '0' { return '1' }
            '0'
        })
        .collect::<String>();

    let bit_string_1c = format!("1{flipped_bit_string}");

    // minus one because this number is read as 2c, which is 1c + 1
    i128::from_str_radix(&bit_string_1c, 2).unwrap() - 1
}

fn to_twos_complement(n: i128) -> i128 {
    // don't swap to `is_positive`: `is_negative` auto-handles input of zero
    if n.is_negative() {
        return to_ones_complement(n) + 1;
    }
    n
}

fn build_unsigned_bit_string(n: i128) -> String {
    if n.is_positive() { return format!("{n:b}") }
    // format!("{:b}", !n);

    // highest power of 2 that "fits in" n,
    // n of 65 returns value of 6
    let num_bits = {
        let mut i: u32 = 0;
        loop {
            if n == 0 || 2i128.pow(i) > n.abs() { break i + 1 };
            i += 1;
        }
    };
    dbg!(num_bits);

    let mut remaining_value = n;
    let bit_string = {
        let mut working_bit_string = String::new();

        for bit in (1..=num_bits).rev() {
            let bit_value = 2i128.pow(bit);

            if bit_value <= remaining_value {
                working_bit_string.push('1');
                remaining_value -= bit_value;
            }
            working_bit_string.push('0');
        }

        working_bit_string
    };

    bit_string
}

fn build_signed_bit_string(n: i128) -> String {
    let unsigned_bit_string = build_unsigned_bit_string(n);

    if n.is_negative() {
        return format!("1{}", unsigned_bit_string);
    }

    unsigned_bit_string
}

/// Converts a value `n` to excess `e`.
pub fn to_excess(e: i128, n: i128) -> Result<String, String> {
    if n.abs() > e {
        return Err(format!("input {n} too large for Excess-{e}"))
    }
    let total_bits = i128::ilog2(e) as usize + 1;
    let unpadded_bit_string = build_unsigned_bit_string(n + e);
    dbg!(&n);
    dbg!(&total_bits);
    dbg!(&unpadded_bit_string);
    let padding = "0".repeat(total_bits - unpadded_bit_string.len());
    Ok(format!("{}{}", padding, unpadded_bit_string))
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
    fn build_unsigned_bit_string_positive() {
        assert_eq!(build_unsigned_bit_string(37), "100101")
    }

    #[test]
    fn ones_complement_zero() {
        assert_eq!(to_ones_complement(0), 0b0);
    }

    #[test]
    fn ones_complement_positive() {
        assert_eq!(to_ones_complement(25), 0b0011001);
        assert_eq!(to_ones_complement(35), 0b0100011);
    }

    #[test]
    fn ones_complement_negative() {
        assert_eq!(to_ones_complement(-35), 0b101_1100);
        assert_eq!(to_ones_complement(-90), 0b10100101);
        assert_eq!(to_ones_complement(-22), 0b10_1001);
        assert_eq!(to_ones_complement(-42), 0b101_0101);
    }

    #[test]
    fn twos_complement_zero() {
        assert_eq!(to_twos_complement(0), 0b0);
    }

    #[test]
    fn twos_complement_negative() {
        assert_eq!(to_twos_complement(-90), 0b10100110);
        assert_eq!(to_twos_complement(-2), 0b1110);
        assert_eq!(to_twos_complement(-32), 0b110_0000);
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
