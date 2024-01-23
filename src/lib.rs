pub fn run(left: usize, right: usize) -> usize {
    left + right
}

fn to_ones_complement(n: i32) -> i32 {
    if n.is_positive() || n == 0 { return n; }

    let num_unsigned_bits = format!("{:b}", !n).len();

    let bit_string: String = format!("{:b}", !n);

    let flipped_bit_string: String = bit_string.chars()
        .take(num_unsigned_bits)
        .map(|c| {
            if c == '0' { return '1' }
            '0'
        })
        .collect::<String>();

    let bit_string_1c = format!("1{flipped_bit_string}");

    i32::from_str_radix(&bit_string_1c, 2).unwrap() - 1
}

fn to_twos_complement(n: i32) -> i32 {
    if n.is_negative() {
        return to_ones_complement(n) + 1;
    }
    n
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
}
