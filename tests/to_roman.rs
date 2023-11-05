use roman_numerals::ToRoman;

/// Test if the basics numbers are covered by the `to_roman` method.
#[test]
fn basics() {
    assert_eq!(0u64.to_roman(), "nulla");
    assert_eq!(1u64.to_roman(), "I");
    assert_eq!(5u64.to_roman(), "V");
    assert_eq!(10u64.to_roman(), "X");
    assert_eq!(50u64.to_roman(), "L");
    assert_eq!(100u64.to_roman(), "C");
    assert_eq!(500u64.to_roman(), "D");
    assert_eq!(1_000u64.to_roman(), "M");
}

/// Test if the convertion works for numbers between 0 and 100u64.
#[test]
fn zero_to_one_hundred() {
    assert_eq!(0u64.to_roman(), "nulla");
    assert_eq!(1u64.to_roman(), "I");
    assert_eq!(2u64.to_roman(), "II");
    assert_eq!(3u64.to_roman(), "III");
    assert_eq!(4u64.to_roman(), "IV");
    assert_eq!(5u64.to_roman(), "V");
    assert_eq!(6u64.to_roman(), "VI");
    assert_eq!(7u64.to_roman(), "VII");
    assert_eq!(8u64.to_roman(), "VIII");
    assert_eq!(9u64.to_roman(), "IX");
    assert_eq!(10u64.to_roman(), "X");
    assert_eq!(11u64.to_roman(), "XI");
    assert_eq!(12u64.to_roman(), "XII");
    assert_eq!(13u64.to_roman(), "XIII");
    assert_eq!(14u64.to_roman(), "XIV");
    assert_eq!(15u64.to_roman(), "XV");
    assert_eq!(16u64.to_roman(), "XVI");
    assert_eq!(17u64.to_roman(), "XVII");
    assert_eq!(18u64.to_roman(), "XVIII");
    assert_eq!(19u64.to_roman(), "XIX");
    assert_eq!(20u64.to_roman(), "XX");
    assert_eq!(21u64.to_roman(), "XXI");
    assert_eq!(22u64.to_roman(), "XXII");
    assert_eq!(23u64.to_roman(), "XXIII");
    assert_eq!(24u64.to_roman(), "XXIV");
    assert_eq!(25u64.to_roman(), "XXV");
    assert_eq!(26u64.to_roman(), "XXVI");
    assert_eq!(27u64.to_roman(), "XXVII");
    assert_eq!(28u64.to_roman(), "XXVIII");
    assert_eq!(29u64.to_roman(), "XXIX");
    assert_eq!(30u64.to_roman(), "XXX");
    assert_eq!(31u64.to_roman(), "XXXI");
    assert_eq!(32u64.to_roman(), "XXXII");
    assert_eq!(33u64.to_roman(), "XXXIII");
    assert_eq!(34u64.to_roman(), "XXXIV");
    assert_eq!(35u64.to_roman(), "XXXV");
    assert_eq!(36u64.to_roman(), "XXXVI");
    assert_eq!(37u64.to_roman(), "XXXVII");
    assert_eq!(38u64.to_roman(), "XXXVIII");
    assert_eq!(39u64.to_roman(), "XXXIX");
    assert_eq!(40u64.to_roman(), "XL");
    assert_eq!(41u64.to_roman(), "XLI");
    assert_eq!(42u64.to_roman(), "XLII");
    assert_eq!(43u64.to_roman(), "XLIII");
    assert_eq!(44u64.to_roman(), "XLIV");
    assert_eq!(45u64.to_roman(), "XLV");
    assert_eq!(46u64.to_roman(), "XLVI");
    assert_eq!(47u64.to_roman(), "XLVII");
    assert_eq!(48u64.to_roman(), "XLVIII");
    assert_eq!(49u64.to_roman(), "XLIX");
    assert_eq!(50u64.to_roman(), "L");
    assert_eq!(51u64.to_roman(), "LI");
    assert_eq!(52u64.to_roman(), "LII");
    assert_eq!(53u64.to_roman(), "LIII");
    assert_eq!(54u64.to_roman(), "LIV");
    assert_eq!(55u64.to_roman(), "LV");
    assert_eq!(56u64.to_roman(), "LVI");
    assert_eq!(57u64.to_roman(), "LVII");
    assert_eq!(58u64.to_roman(), "LVIII");
    assert_eq!(59u64.to_roman(), "LIX");
    assert_eq!(60u64.to_roman(), "LX");
    assert_eq!(61u64.to_roman(), "LXI");
    assert_eq!(62u64.to_roman(), "LXII");
    assert_eq!(63u64.to_roman(), "LXIII");
    assert_eq!(64u64.to_roman(), "LXIV");
    assert_eq!(65u64.to_roman(), "LXV");
    assert_eq!(66u64.to_roman(), "LXVI");
    assert_eq!(67u64.to_roman(), "LXVII");
    assert_eq!(68u64.to_roman(), "LXVIII");
    assert_eq!(69u64.to_roman(), "LXIX");
    assert_eq!(70u64.to_roman(), "LXX");
    assert_eq!(71u64.to_roman(), "LXXI");
    assert_eq!(72u64.to_roman(), "LXXII");
    assert_eq!(73u64.to_roman(), "LXXIII");
    assert_eq!(74u64.to_roman(), "LXXIV");
    assert_eq!(75u64.to_roman(), "LXXV");
    assert_eq!(76u64.to_roman(), "LXXVI");
    assert_eq!(77u64.to_roman(), "LXXVII");
    assert_eq!(78u64.to_roman(), "LXXVIII");
    assert_eq!(79u64.to_roman(), "LXXIX");
    assert_eq!(80u64.to_roman(), "LXXX");
    assert_eq!(81u64.to_roman(), "LXXXI");
    assert_eq!(82u64.to_roman(), "LXXXII");
    assert_eq!(83u64.to_roman(), "LXXXIII");
    assert_eq!(84u64.to_roman(), "LXXXIV");
    assert_eq!(85u64.to_roman(), "LXXXV");
    assert_eq!(86u64.to_roman(), "LXXXVI");
    assert_eq!(87u64.to_roman(), "LXXXVII");
    assert_eq!(88u64.to_roman(), "LXXXVIII");
    assert_eq!(89u64.to_roman(), "LXXXIX");
    assert_eq!(90u64.to_roman(), "XC");
    assert_eq!(91u64.to_roman(), "XCI");
    assert_eq!(92u64.to_roman(), "XCII");
    assert_eq!(93u64.to_roman(), "XCIII");
    assert_eq!(94u64.to_roman(), "XCIV");
    assert_eq!(95u64.to_roman(), "XCV");
    assert_eq!(96u64.to_roman(), "XCVI");
    assert_eq!(97u64.to_roman(), "XCVII");
    assert_eq!(98u64.to_roman(), "XCVIII");
    assert_eq!(99u64.to_roman(), "XCIX");
    assert_eq!(100u64.to_roman(), "C");
}

/// Test if the extended numbers are covered by the to_roman method.
/// This includes notations up to billions..
#[test]
fn extended() {
    // thousands
    assert_eq!(1_000u64.to_roman(), "M");
    assert_eq!(2_000u64.to_roman(), "MM");
    assert_eq!(3_000u64.to_roman(), "MMM");
    assert_eq!(4_000u64.to_roman(), "MV̄");
    assert_eq!(5_000u64.to_roman(), "V̄");
    assert_eq!(6_000u64.to_roman(), "V̄M");
    assert_eq!(7_000u64.to_roman(), "V̄MM");
    assert_eq!(8_000u64.to_roman(), "V̄MMM");
    assert_eq!(9_000u64.to_roman(), "MX̄");

    // tens of thousands
    assert_eq!(10_000u64.to_roman(), "X̄");
    assert_eq!(20_000u64.to_roman(), "X̄X̄");
    assert_eq!(30_000u64.to_roman(), "X̄X̄X̄");
    assert_eq!(40_000u64.to_roman(), "X̄L̄");
    assert_eq!(50_000u64.to_roman(), "L̄");
    assert_eq!(60_000u64.to_roman(), "L̄X̄");
    assert_eq!(70_000u64.to_roman(), "L̄X̄X̄");
    assert_eq!(80_000u64.to_roman(), "L̄X̄X̄X̄");
    assert_eq!(90_000u64.to_roman(), "X̄C̄");

    // hundreds of thousands
    assert_eq!(100_000u64.to_roman(), "C̄");
    assert_eq!(200_000u64.to_roman(), "C̄C̄");
    assert_eq!(300_000u64.to_roman(), "C̄C̄C̄");
    assert_eq!(400_000u64.to_roman(), "C̄D̄");
    assert_eq!(500_000u64.to_roman(), "D̄");
    assert_eq!(600_000u64.to_roman(), "D̄C̄");
    assert_eq!(700_000u64.to_roman(), "D̄C̄C̄");
    assert_eq!(800_000u64.to_roman(), "D̄C̄C̄C̄");
    assert_eq!(900_000u64.to_roman(), "C̄M̄");

    // millions
    assert_eq!(1_000_000u64.to_roman(), "M̄");
    assert_eq!(2_000_000u64.to_roman(), "M̄M̄");
    assert_eq!(3_000_000u64.to_roman(), "M̄M̄M̄");
    assert_eq!(4_000_000u64.to_roman(), "M̄V̄̄");
    assert_eq!(5_000_000u64.to_roman(), "V̄̄");
    assert_eq!(6_000_000u64.to_roman(), "V̄̄M̄");
    assert_eq!(7_000_000u64.to_roman(), "V̄̄M̄M̄");
    assert_eq!(8_000_000u64.to_roman(), "V̄̄M̄M̄M̄");
    assert_eq!(9_000_000u64.to_roman(), "M̄X̄̄");

    // tens of millions
    assert_eq!(10_000_000u64.to_roman(), "X̄̄");
    assert_eq!(20_000_000u64.to_roman(), "X̄̄X̄̄");
    assert_eq!(30_000_000u64.to_roman(), "X̄̄X̄̄X̄̄");
    assert_eq!(40_000_000u64.to_roman(), "X̄̄L̄̄");
    assert_eq!(50_000_000u64.to_roman(), "L̄̄");
    assert_eq!(60_000_000u64.to_roman(), "L̄̄X̄̄");
    assert_eq!(70_000_000u64.to_roman(), "L̄̄X̄̄X̄̄");
    assert_eq!(80_000_000u64.to_roman(), "L̄̄X̄̄X̄̄X̄̄");
    assert_eq!(90_000_000u64.to_roman(), "X̄̄C̄̄");

    // hundreds of millions
    assert_eq!(100_000_000u64.to_roman(), "C̄̄");
    assert_eq!(200_000_000u64.to_roman(), "C̄̄C̄̄");
    assert_eq!(300_000_000u64.to_roman(), "C̄̄C̄̄C̄̄");
    assert_eq!(400_000_000u64.to_roman(), "C̄̄D̄̄");
    assert_eq!(500_000_000u64.to_roman(), "D̄̄");
    assert_eq!(600_000_000u64.to_roman(), "D̄̄C̄̄");
    assert_eq!(700_000_000u64.to_roman(), "D̄̄C̄̄C̄̄");
    assert_eq!(800_000_000u64.to_roman(), "D̄̄C̄̄C̄̄C̄̄");
    assert_eq!(900_000_000u64.to_roman(), "C̄̄M̄̄");

    // billion
    assert_eq!(1_000_000_000u64.to_roman(), "M̄̄");

    // trillion
    assert_eq!(1_000_000_000_000u64.to_roman(), "M̄̄̄");
}

#[test]
fn random_values() {
    assert_eq!(1_999u64.to_roman(), "MCMXCIX");
}
