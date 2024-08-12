use std::str::FromStr;

// fn main() {
//     // Parsing two large numbers from strings with scientific notation
//     let num1 = ScaledNumber::from_str("1.23e28").unwrap();
//     let num2 = ScaledNumber::from_str("9.87e29").unwrap();

//     // Parsing two more large numbers from strings with suffixes
//     let num3 = ScaledNumber::from_str("2.0T").unwrap();
//     let num4 = ScaledNumber::from_str("500M").unwrap();

//     // Perform addition on two ScaledNumber instances
//     let sum = num1 + num2;
//     // Print the formatted result of the addition, which scales the result to the nearest appropriate suffix
//     println!("Sum: {}", sum.format(1)); // Expected Output: "Sum: 1.0No"

//     // Perform subtraction on two ScaledNumber instances
//     let difference = num2 - num1;
//     // Print the formatted result of the subtraction
//     println!("Difference: {}", difference.format(1)); // Expected Output: "Difference: 9.7Oc"

//     // Perform multiplication on two ScaledNumber instances
//     let product = num3 * num4;
//     // Print the formatted result of the multiplication
//     println!("Product: {}", product.format(1)); // Expected Output: "Product: 1.0Q"

//     // Perform division on two ScaledNumber instances
//     let quotient = num2 / num3;
//     // Print the formatted result of the division
//     println!("Quotient: {}", quotient.format(1)); // Expected Output: "Quotient: 493.5B"
// }

// Enum to represent different scales like K (Thousand), M (Million), B (Billion), etc.
#[derive(Debug, Clone, Copy)]
enum Scale {
    No,   // Nonillion (1e30)
    Oc,   // Octillion (1e27)
    Se,   // Septillion (1e24)
    Sx,   // Sextillion (1e21)
    Qn,   // Quintillion (1e18)
    Q,    // Quadrillion (1e15)
    T,    // Trillion (1e12)
    B,    // Billion (1e9)
    M,    // Million (1e6)
    K,    // Thousand (1e3)
    None, // No scaling (used for numbers less than 1,000)
}

impl Scale {
    // Returns the string suffix associated with each scale (e.g., "M" for million)
    fn suffix(&self) -> &'static str {
        match self {
            Scale::No => "No", // Suffix for Nonillion
            Scale::Oc => "Oc", // Suffix for Octillion
            Scale::Se => "Se", // Suffix for Septillion
            Scale::Sx => "Sx", // Suffix for Sextillion
            Scale::Qn => "Qn", // Suffix for Quintillion
            Scale::Q => "Q",   // Suffix for Quadrillion
            Scale::T => "T",   // Suffix for Trillion
            Scale::B => "B",   // Suffix for Billion
            Scale::M => "M",   // Suffix for Million
            Scale::K => "K",   // Suffix for Thousand
            Scale::None => "", // No suffix for small numbers (below 1,000)
        }
    }

    // Returns the numeric value associated with each scale (e.g., 1e6 for million)
    fn value(&self) -> f64 {
        match self {
            Scale::No => 1e30,  // Nonillion
            Scale::Oc => 1e27,  // Octillion
            Scale::Se => 1e24,  // Septillion
            Scale::Sx => 1e21,  // Sextillion
            Scale::Qn => 1e18,  // Quintillion
            Scale::Q => 1e15,   // Quadrillion
            Scale::T => 1e12,   // Trillion
            Scale::B => 1e9,    // Billion
            Scale::M => 1e6,    // Million
            Scale::K => 1e3,    // Thousand
            Scale::None => 1.0, // No scaling (for numbers less than 1,000)
        }
    }

    // Determines the appropriate scale based on the numeric value of the input
    fn from_value(value: f64) -> Scale {
        match value {
            v if v >= 1e30 => Scale::No, // Use Nonillion scale for values >= 1e30
            v if v >= 1e27 => Scale::Oc, // Use Octillion scale for values >= 1e27
            v if v >= 1e24 => Scale::Se, // Use Septillion scale for values >= 1e24
            v if v >= 1e21 => Scale::Sx, // Use Sextillion scale for values >= 1e21
            v if v >= 1e18 => Scale::Qn, // Use Quintillion scale for values >= 1e18
            v if v >= 1e15 => Scale::Q,  // Use Quadrillion scale for values >= 1e15
            v if v >= 1e12 => Scale::T,  // Use Trillion scale for values >= 1e12
            v if v >= 1e9 => Scale::B,   // Use Billion scale for values >= 1e9
            v if v >= 1e6 => Scale::M,   // Use Million scale for values >= 1e6
            v if v >= 1e3 => Scale::K,   // Use Thousand scale for values >= 1e3
            _ => Scale::None,            // No scaling needed for values < 1e3
        }
    }
}

// Struct to represent a number that has been scaled to a particular magnitude
#[derive(Debug, Clone, Copy)]
struct ScaledNumber {
    value: f64,   // The numeric value after scaling (e.g., 1.23)
    scale: Scale, // The scale associated with this value (e.g., Million, Billion)
}

impl ScaledNumber {
    // Constructor for ScaledNumber, automatically determines the appropriate scale for the given value
    fn new(value: f64) -> Self {
        let scale = Scale::from_value(value); // Determine the scale based on the value
        let scaled_value = value / scale.value(); // Scale the value to the appropriate magnitude
        ScaledNumber {
            value: scaled_value, // Store the scaled value
            scale,               // Store the determined scale
        }
    }

    // Formats the scaled number into a human-readable string, with a specified number of decimal places
    fn format(self, decimal_places: usize) -> String {
        // Round the scaled value to the specified number of decimal places
        let rounded_value = (self.value * 10_f64.powi(decimal_places as i32)).round()
            / 10_f64.powi(decimal_places as i32);
        // Return the formatted string with the scale suffix
        format!("{:.1}{}", rounded_value, self.scale.suffix())
    }
}

// Implementing the Add trait to allow addition of two ScaledNumber instances
impl std::ops::Add for ScaledNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        // Convert both numbers to their full values, add them, and then re-scale the result
        ScaledNumber::new(self.value * self.scale.value() + other.value * other.scale.value())
    }
}

// Implementing the Sub trait to allow subtraction of two ScaledNumber instances
impl std::ops::Sub for ScaledNumber {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        // Convert both numbers to their full values, subtract them, and then re-scale the result
        ScaledNumber::new(self.value * self.scale.value() - other.value * other.scale.value())
    }
}

// Implementing the Mul trait to allow multiplication of two ScaledNumber instances
impl std::ops::Mul for ScaledNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        // Convert both numbers to their full values, multiply them, and then re-scale the result
        ScaledNumber::new(self.value * self.scale.value() * other.value * other.scale.value())
    }
}

// Implementing the Div trait to allow division of two ScaledNumber instances
impl std::ops::Div for ScaledNumber {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        // Convert both numbers to their full values, divide them, and then re-scale the result
        ScaledNumber::new(self.value * self.scale.value() / other.value * other.scale.value())
    }
}

// Implementing FromStr trait to allow parsing a string into a ScaledNumber
impl FromStr for ScaledNumber {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim(); // Remove any leading or trailing whitespace from the string
                          // Define suffixes and their corresponding scales
        let suffix_multipliers = [
            ("No", Scale::No),
            ("Oc", Scale::Oc),
            ("Se", Scale::Se),
            ("Sx", Scale::Sx),
            ("Qn", Scale::Qn),
            ("Q", Scale::Q),
            ("T", Scale::T),
            ("B", Scale::B),
            ("M", Scale::M),
            ("K", Scale::K),
        ];

        // Check if the string ends with any of the known suffixes
        for &(suffix, scale) in &suffix_multipliers {
            if s.ends_with(suffix) {
                // If a suffix is found, extract the numeric part and scale it appropriately
                let number_part = &s[..s.len() - suffix.len()];
                return Ok(ScaledNumber {
                    value: f64::from_str(number_part)?, // Parse the numeric part
                    scale,                              // Use the identified scale
                });
            }
        }

        // If no suffix is found, parse the string as a plain number and scale it accordingly
        Ok(ScaledNumber::new(f64::from_str(s)?))
    }
}
