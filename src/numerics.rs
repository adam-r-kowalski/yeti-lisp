use rug::{self, float::OrdFloat, Rational};

use crate::Expression;

pub fn decimal_digits_to_bits(decimal_digits: usize) -> u32 {
    (decimal_digits as f64 * 3.322).ceil() as u32
}

pub fn bits_to_decimal_digits(bits: u32) -> usize {
    (bits as f64 / 3.322).floor() as usize
}

#[derive(Clone, PartialEq)]
pub struct Float(rug::Float);

impl Eq for Float {}

impl core::hash::Hash for Float {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        OrdFloat::from(self.0.clone()).hash(state)
    }
}

impl Float {
    pub fn from_str(s: &str) -> Float {
        let offset = if s.starts_with("-") { 2 } else { 1 };
        let digits = s.len() - offset;
        let bits = decimal_digits_to_bits(digits);
        let parsed = rug::Float::parse(s).unwrap();
        let float = rug::Float::with_val(bits, parsed);
        Float(float)
    }

    pub fn to_f64(&self) -> f64 {
        self.0.to_f64()
    }

    pub fn from_f64(f: f64) -> Float {
        let bits = f64::MANTISSA_DIGITS;
        let float = rug::Float::with_val(bits, f);
        Float(float)
    }
}

impl core::fmt::Debug for Float {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bits = self.0.prec();
        let digits = bits_to_decimal_digits(bits);
        write!(f, "{:.*}", digits, self.0)
    }
}

impl core::fmt::Display for Float {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bits = self.0.prec();
        let digits = bits_to_decimal_digits(bits);
        write!(f, "{:.*}", digits, self.0)
    }
}

pub fn ratio(rational: Rational) -> Expression {
    if rational.is_integer() {
        Expression::Integer(rational.numer().clone())
    } else {
        Expression::Ratio(rational)
    }
}
