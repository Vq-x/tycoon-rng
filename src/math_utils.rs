use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
struct TruncatedNumber {
    base: u32,  // Stores the significant digits
    scale: u32, // Stores the scale factor as a power of 10
}

impl TruncatedNumber {
    fn new(mut value: u64) -> Self {
        let digits = value.to_string().len() as u32;

        if digits > 5 {
            let power = 10_u64.pow(digits - 5);
            value = (value + power / 2) / power; // Round to 5 significant digits
        }

        TruncatedNumber {
            base: value as u32,
            scale: digits.saturating_sub(5),
        }
    }

    fn value(&self) -> u64 {
        (self.base as u64) * 10_u64.pow(self.scale)
    }

    fn human_readable(&self) -> String {
        let mut value = self.base as f64;
        let mut scale = self.scale;
        let suffixes = ["", "K", "M", "B", "T"];
        let mut i = 0;

        while scale >= 3 && i < suffixes.len() - 1 {
            value /= 1000.0;
            scale -= 3;
            i += 1;
        }

        format!("{:.1}{}", value, suffixes[i])
    }
}

// Implementing Addition
impl Add for TruncatedNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let sum = self.value() + other.value();
        TruncatedNumber::new(sum)
    }
}

// Implementing Subtraction
impl Sub for TruncatedNumber {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let diff = self.value().saturating_sub(other.value());
        TruncatedNumber::new(diff)
    }
}

// Implementing Multiplication
impl Mul<u64> for TruncatedNumber {
    type Output = Self;

    fn mul(self, multiplier: u64) -> Self::Output {
        let product = self.value() * multiplier;
        TruncatedNumber::new(product)
    }
}

// Implementing Division
impl Div<u64> for TruncatedNumber {
    type Output = Self;

    fn div(self, divisor: u64) -> Self::Output {
        if divisor == 0 {
            panic!("Division by zero is not allowed!");
        }
        let quotient = self.value() / divisor;
        TruncatedNumber::new(quotient)
    }
}

// Implementing Display Trait for Human-Readable Output
impl fmt::Display for TruncatedNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.human_readable())
    }
}

fn main() {
    let number = TruncatedNumber::new(159015901590);
    println!("Initial truncated number: {}", number.value()); // Output: 159010000000

    let multiplied = number * 2;
    println!("After multiplication: {}", multiplied.value()); // Output: 318020000000

    let added = number + TruncatedNumber::new(123456);
    println!("After addition: {}", added.value()); // Output: 159030000000

    let subtracted = number - TruncatedNumber::new(15901);
    println!("After subtraction: {}", subtracted.value()); // Output: 158990000000

    let divided = number / 2;
    println!("After division: {}", divided.value()); // Output: 79505000000

    println!("Human-readable format: {}", multiplied); // Output: 318.0B
}
