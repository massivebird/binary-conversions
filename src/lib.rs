// Binary representations of integers are in twos complement.
// That's confusing! So we convert the number to standard [un]signed magnitude
// before making conversions (if necessary).

pub fn to_ones_complement(n: i32) -> i32 {
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
    i32::from_str_radix(&bit_string_1c, 2).unwrap() - 1
}

pub fn to_twos_complement(n: i32) -> i32 {
    // don't swap to `is_positive`: `is_negative` auto-handles input of zero
    if n.is_negative() {
        return to_ones_complement(n) + 1;
    }
    n
}

pub fn to_excess_64(n: i32) -> i32 {
    assert!(n < 127 - 64, "E64: input too large");
    // TODO: why? why does this work? What the fuck?
    n + 64
}

fn build_unsigned_bit_string(n: i32) -> String {
    if n.is_positive() { return format!("{n:b}") }
    format!("{:b}", !n)
}

fn build_signed_bit_string(n: i32) -> String {
    format!("1{}", build_unsigned_bit_string(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radix_test() {
        assert_eq!(i32::from_str_radix("11011101", 2).unwrap(), 221);
    }

    #[test]
    fn test_ones_complement_zero() {
        assert_eq!(to_ones_complement(0), 0b0);
    }

    #[test]
    fn test_ones_complement_p_0() {
        assert_eq!(to_ones_complement(25), 0b0011001);
    }

    #[test]
    fn test_ones_complement_p_1() {
        assert_eq!(to_ones_complement(35), 0b0100011);
    }

    #[test]
    fn test_ones_complement_n_0() {
        assert_eq!(to_ones_complement(-22), 0b10_1001);
    }

    #[test]
    fn test_ones_complement_n_1() {
        assert_eq!(to_ones_complement(-42), 0b101_0101);
    }

    #[test]
    fn test_ones_complement_n_2() {
        assert_eq!(to_ones_complement(-35), 0b101_1100);
    }

    #[test]
    fn test_ones_complement_n_3() {
        assert_eq!(to_ones_complement(-90), 0b10100101);
    }

    #[test]
    fn test_twos_complement_zero() {
        assert_eq!(to_twos_complement(0), 0b0);
    }

    #[test]
    fn test_twos_complement_n_0() {
        assert_eq!(to_twos_complement(-90), 0b10100110);
    }

    #[test]
    fn test_excess_64_p_0() {
        assert_eq!(to_excess_64(35), 0b110_0011);
    }

    #[test]
    fn test_excess_64_zero() {
        assert_eq!(to_excess_64(0), 0b1000000);
    }

    #[test]
    #[should_panic]
    fn test_excess_64_p_1() {
        assert_eq!(to_excess_64(125), 0b1111_1101);
        
    }

    #[test]
    fn test_excess_64_n_0() {
        assert_eq!(to_excess_64(-22), 0b0101010);
    }
}
