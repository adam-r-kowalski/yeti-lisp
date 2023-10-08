use rug::Float;

pub fn decimal_digits_to_bits(decimal_digits: usize) -> u32 {
    (decimal_digits as f64 * 3.322).ceil() as u32
}

pub fn bits_to_decimal_digits(bits: u32) -> usize {
    (bits as f64 / 3.322).floor() as usize
}

pub fn string_to_float(number: &str) -> Float {
    let offset = if number.starts_with("-") { 2 } else { 1 };
    let digits = number.len() - offset;
    let bits = decimal_digits_to_bits(digits);
    let parsed = Float::parse(number).unwrap();
    Float::with_val(bits, parsed)
}
