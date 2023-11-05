pub trait ToRoman {
    /// Convert a number to roman numerals.
    ///
    /// ### Example
    ///
    /// ```
    /// use roman_numerals::ToRoman;
    /// assert_eq!(1u32.to_roman(), "I");
    /// ```
    fn to_roman(self) -> String;
}

macro_rules! impl_to_roman {
    ($t: tt) => {
        impl ToRoman for $t {
            fn to_roman(mut self) -> String {
                let mut roman = String::new();

                let mut level = 0;
                let mut macro_level = 0;
                let mut previous_macro_level_string = "".to_string();
                let mut macro_level_string = "".to_string();

                if self == 0 {
                    return "nulla".to_string();
                }

                while self != 0 {
                    match (macro_level, level) {
                        (0, 0) => match self % 10 {
                            1 => roman.insert_str(0, &format!("I{macro_level_string}")),
                            2 => roman.insert_str(0, &format!("I{macro_level_string}I{macro_level_string}")),
                            3 => roman.insert_str(0, &format!("I{macro_level_string}I{macro_level_string}I{macro_level_string}")),
                            4 => roman.insert_str(0, &format!("I{macro_level_string}V{macro_level_string}")),
                            5 => roman.insert_str(0, &format!("V{macro_level_string}")),
                            6 => roman.insert_str(0, &format!("V{macro_level_string}I{macro_level_string}")),
                            7 => roman.insert_str(0, &format!("V{macro_level_string}I{macro_level_string}I{macro_level_string}")),
                            8 => roman.insert_str(
                                0,
                                &format!("V{macro_level_string}I{macro_level_string}I{macro_level_string}I{macro_level_string}"),
                            ),
                            9 => roman.insert_str(0, &format!("I{macro_level_string}X{macro_level_string}")),
                            _ => (),
                        },
                        (_, 0) => match self % 10 {
                            1 => roman.insert_str(0, &format!("M{previous_macro_level_string}")),
                            2 => roman.insert_str(0, &format!("M{previous_macro_level_string}M{previous_macro_level_string}")),
                            3 => roman.insert_str(0, &format!("M{previous_macro_level_string}M{previous_macro_level_string}M{previous_macro_level_string}")),
                            4 => roman.insert_str(0, &format!("M{previous_macro_level_string}V{macro_level_string}")),
                            5 => roman.insert_str(0, &format!("V{macro_level_string}")),
                            6 => roman.insert_str(0, &format!("V{macro_level_string}M{previous_macro_level_string}")),
                            7 => roman.insert_str(0, &format!("V{macro_level_string}M{previous_macro_level_string}M{previous_macro_level_string}")),
                            8 => roman.insert_str(
                                0,
                                &format!("V{macro_level_string}M{previous_macro_level_string}M{previous_macro_level_string}M{previous_macro_level_string}"),
                            ),
                            9 => roman.insert_str(0, &format!("M{previous_macro_level_string}X{macro_level_string}")),
                            _ => (),
                        },
                        (_, 1) => match self % 10 {
                            1 => roman.insert_str(0, &format!("X{macro_level_string}")),
                            2 => roman.insert_str(0, &format!("X{macro_level_string}X{macro_level_string}")),
                            3 => roman.insert_str(0, &format!("X{macro_level_string}X{macro_level_string}X{macro_level_string}")),
                            4 => roman.insert_str(0, &format!("X{macro_level_string}L{macro_level_string}")),
                            5 => roman.insert_str(0, &format!("L{macro_level_string}")),
                            6 => roman.insert_str(0, &format!("L{macro_level_string}X{macro_level_string}")),
                            7 => roman.insert_str(0, &format!("L{macro_level_string}X{macro_level_string}X{macro_level_string}")),
                            8 => roman.insert_str(
                                0,
                                &format!("L{macro_level_string}X{macro_level_string}X{macro_level_string}X{macro_level_string}"),
                            ),
                            9 => roman.insert_str(0, &format!("X{macro_level_string}C{macro_level_string}")),
                            _ => (),
                        },
                        (_, 2) => match self % 10 {
                            1 => roman.insert_str(0, &format!("C{macro_level_string}")),
                            2 => roman.insert_str(0, &format!("C{macro_level_string}C{macro_level_string}")),
                            3 => roman.insert_str(0, &format!("C{macro_level_string}C{macro_level_string}C{macro_level_string}")),
                            4 => roman.insert_str(0, &format!("C{macro_level_string}D{macro_level_string}")),
                            5 => roman.insert_str(0, &format!("D{macro_level_string}")),
                            6 => roman.insert_str(0, &format!("D{macro_level_string}C{macro_level_string}")),
                            7 => roman.insert_str(0, &format!("D{macro_level_string}C{macro_level_string}C{macro_level_string}")),
                            8 => roman.insert_str(
                                0,
                                &format!("D{macro_level_string}C{macro_level_string}C{macro_level_string}C{macro_level_string}"),
                            ),
                            9 => roman.insert_str(0, &format!("C{macro_level_string}M{macro_level_string}")),
                            _ => (),
                        },
                        _ => unreachable!(),
                    }

                    if level < 2 {
                        level += 1;
                    } else {
                        level = 0;
                        macro_level += 1;
                        previous_macro_level_string = macro_level_string.clone();
                        macro_level_string = format!("\u{304}{macro_level_string}");
                    }

                    self /= 10;
                }

                // return the number in roman numerals
                roman
            }
        }
    }
}

macro_rules! impl_to_roman_for_ints {
    ($($t: ty),+) => {
        $(impl_to_roman!($t);)+
    };
}

impl_to_roman_for_ints![u8, u16, u32, u64, u128, usize];
