/// Converts a roman numeral string to a number.
///
/// # Examples
///
/// ```
/// use roman_numerals::FromRoman;
/// assert_eq!(u64::from_roman("ABC"), None);
/// assert_eq!(u64::from_roman("nulla"), Some(0));
/// assert_eq!(u64::from_roman("I"), Some(1));
/// ```
pub trait FromRoman {
    fn from_roman(number: &str) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum RomanDigit {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

macro_rules! impl_from_roman {
    ($t: tt) => {
        impl FromRoman for $t {
            fn from_roman(number: &str) -> Option<$t> {
                // convert to upper case
                let number = number.to_uppercase();

                // test if zero
                if number == "NULLA" {
                    return Some(0);
                }

                // check that only valid characters are used
                if !number
                    .chars()
                    .all(|c| matches!(c, 'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M'))
                {
                    return None;
                }

                // create an accumulator
                let mut accumulator: $t = 0;

                // convert the string to a peekable iterator
                // (reversed because we want to start from the units place)
                let mut digits = number
                    .chars()
                    .map(|c| match c {
                        'I' => RomanDigit::I,
                        'V' => RomanDigit::V,
                        'X' => RomanDigit::X,
                        'L' => RomanDigit::L,
                        'C' => RomanDigit::C,
                        'D' => RomanDigit::D,
                        'M' => RomanDigit::M,
                        _ => unreachable!(),
                    })
                    .rev()
                    .peekable();

                // for every digit in the string
                while let Some(digit) = digits.next() {
                    // add the current digit to the accumulator
                    accumulator += match digit {
                        RomanDigit::I => 1,
                        RomanDigit::V => 5,
                        RomanDigit::X => 10,
                        RomanDigit::L => 50,
                        RomanDigit::C => 100,
                        RomanDigit::D => 500,
                        RomanDigit::M => 1_000,
                    };

                    // if there is a next digit ...
                    if let Some(&next_digit) = digits.peek() {
                        // ... which is smaller than the current digit
                        if next_digit < digit {
                            // subtract the next digit from the accumulator
                            accumulator -= match next_digit {
                                RomanDigit::I => 1,
                                RomanDigit::V => 5,
                                RomanDigit::X => 10,
                                RomanDigit::L => 50,
                                RomanDigit::C => 100,
                                RomanDigit::D => 500,
                                RomanDigit::M => 1_000,
                            };

                            // skip the next digit
                            digits.next();
                        }
                    }
                }

                // return the accumulator
                Some(accumulator)
            }
        }
    };
}

// Define a macro to implement FromRoman for all integer types.
macro_rules! impl_from_roman_for_ints {
    ($($t: ty),+) => {
        $(impl_from_roman!($t);)+
    };
}

// Implement FromRoman for all integer types.
impl_from_roman_for_ints![u16, u32, u64, u128, usize];

/// Implement FromRoman for u8.
/// This had to be done separately because u8 cannot store the values of 500 and 1000.
impl FromRoman for u8 {
    fn from_roman(number: &str) -> Option<Self> {
        u16::from_roman(number).map(|n| n.try_into().ok()).flatten()
    }
}
