use roman_numerals::FromRoman;

#[test]
fn invalids() {
    assert_eq!(u64::from_roman("a"), None);
    assert_eq!(u64::from_roman("B"), None);
    assert_eq!(u64::from_roman("abc"), None);
    assert_eq!(u32::from_roman("ABC"), None);
}

#[test]
fn uppercase_lowercase() {
    assert_eq!(u64::from_roman("NULLA"), u64::from_roman("nulla"));
    assert_eq!(u64::from_roman("I"), u64::from_roman("i"));
    assert_eq!(u64::from_roman("V"), u64::from_roman("v"));
    assert_eq!(u64::from_roman("X"), u64::from_roman("x"));
    assert_eq!(u64::from_roman("L"), u64::from_roman("l"));
    assert_eq!(u64::from_roman("C"), u64::from_roman("c"));
    assert_eq!(u64::from_roman("D"), u64::from_roman("d"));
    assert_eq!(u64::from_roman("M"), u64::from_roman("m"));
}

#[test]
fn basics() {
    assert_eq!(u64::from_roman("nulla"), Some(0));
    assert_eq!(u64::from_roman("I"), Some(1));
    assert_eq!(u64::from_roman("V"), Some(5));
    assert_eq!(u64::from_roman("X"), Some(10));
    assert_eq!(u64::from_roman("L"), Some(50));
    assert_eq!(u64::from_roman("C"), Some(100));
    assert_eq!(u64::from_roman("D"), Some(500));
    assert_eq!(u64::from_roman("M"), Some(1_000));
}

#[test]
fn zero_to_one_hundred() {
    assert_eq!(u64::from_roman("nulla"), Some(0));
    assert_eq!(u64::from_roman("I"), Some(1));
    assert_eq!(u64::from_roman("II"), Some(2));
    assert_eq!(u64::from_roman("III"), Some(3));
    assert_eq!(u64::from_roman("IV"), Some(4));
    assert_eq!(u64::from_roman("V"), Some(5));
    assert_eq!(u64::from_roman("VI"), Some(6));
    assert_eq!(u64::from_roman("VII"), Some(7));
    assert_eq!(u64::from_roman("VIII"), Some(8));
    assert_eq!(u64::from_roman("IX"), Some(9));
    assert_eq!(u64::from_roman("X"), Some(10));
    assert_eq!(u64::from_roman("XI"), Some(11));
    assert_eq!(u64::from_roman("XII"), Some(12));
    assert_eq!(u64::from_roman("XIII"), Some(13));
    assert_eq!(u64::from_roman("XIV"), Some(14));
    assert_eq!(u64::from_roman("XV"), Some(15));
    assert_eq!(u64::from_roman("XVI"), Some(16));
    assert_eq!(u64::from_roman("XVII"), Some(17));
    assert_eq!(u64::from_roman("XVIII"), Some(18));
    assert_eq!(u64::from_roman("XIX"), Some(19));
    assert_eq!(u64::from_roman("XX"), Some(20));
    assert_eq!(u64::from_roman("XXI"), Some(21));
    assert_eq!(u64::from_roman("XXII"), Some(22));
    assert_eq!(u64::from_roman("XXIII"), Some(23));
    assert_eq!(u64::from_roman("XXIV"), Some(24));
    assert_eq!(u64::from_roman("XXV"), Some(25));
    assert_eq!(u64::from_roman("XXVI"), Some(26));
    assert_eq!(u64::from_roman("XXVII"), Some(27));
    assert_eq!(u64::from_roman("XXVIII"), Some(28));
    assert_eq!(u64::from_roman("XXIX"), Some(29));
    assert_eq!(u64::from_roman("XXX"), Some(30));
    assert_eq!(u64::from_roman("XXXI"), Some(31));
    assert_eq!(u64::from_roman("XXXII"), Some(32));
    assert_eq!(u64::from_roman("XXXIII"), Some(33));
    assert_eq!(u64::from_roman("XXXIV"), Some(34));
    assert_eq!(u64::from_roman("XXXV"), Some(35));
    assert_eq!(u64::from_roman("XXXVI"), Some(36));
    assert_eq!(u64::from_roman("XXXVII"), Some(37));
    assert_eq!(u64::from_roman("XXXVIII"), Some(38));
    assert_eq!(u64::from_roman("XXXIX"), Some(39));
    assert_eq!(u64::from_roman("XL"), Some(40));
    assert_eq!(u64::from_roman("XLI"), Some(41));
    assert_eq!(u64::from_roman("XLII"), Some(42));
    assert_eq!(u64::from_roman("XLIII"), Some(43));
    assert_eq!(u64::from_roman("XLIV"), Some(44));
    assert_eq!(u64::from_roman("XLV"), Some(45));
    assert_eq!(u64::from_roman("XLVI"), Some(46));
    assert_eq!(u64::from_roman("XLVII"), Some(47));
    assert_eq!(u64::from_roman("XLVIII"), Some(48));
    assert_eq!(u64::from_roman("XLIX"), Some(49));
    assert_eq!(u64::from_roman("L"), Some(50));
    assert_eq!(u64::from_roman("LI"), Some(51));
    assert_eq!(u64::from_roman("LII"), Some(52));
    assert_eq!(u64::from_roman("LIII"), Some(53));
    assert_eq!(u64::from_roman("LIV"), Some(54));
    assert_eq!(u64::from_roman("LV"), Some(55));
    assert_eq!(u64::from_roman("LVI"), Some(56));
    assert_eq!(u64::from_roman("LVII"), Some(57));
    assert_eq!(u64::from_roman("LVIII"), Some(58));
    assert_eq!(u64::from_roman("LIX"), Some(59));
    assert_eq!(u64::from_roman("LX"), Some(60));
    assert_eq!(u64::from_roman("LXI"), Some(61));
    assert_eq!(u64::from_roman("LXII"), Some(62));
    assert_eq!(u64::from_roman("LXIII"), Some(63));
    assert_eq!(u64::from_roman("LXIV"), Some(64));
    assert_eq!(u64::from_roman("LXV"), Some(65));
    assert_eq!(u64::from_roman("LXVI"), Some(66));
    assert_eq!(u64::from_roman("LXVII"), Some(67));
    assert_eq!(u64::from_roman("LXVIII"), Some(68));
    assert_eq!(u64::from_roman("LXIX"), Some(69));
    assert_eq!(u64::from_roman("LXX"), Some(70));
    assert_eq!(u64::from_roman("LXXI"), Some(71));
    assert_eq!(u64::from_roman("LXXII"), Some(72));
    assert_eq!(u64::from_roman("LXXIII"), Some(73));
    assert_eq!(u64::from_roman("LXXIV"), Some(74));
    assert_eq!(u64::from_roman("LXXV"), Some(75));
    assert_eq!(u64::from_roman("LXXVI"), Some(76));
    assert_eq!(u64::from_roman("LXXVII"), Some(77));
    assert_eq!(u64::from_roman("LXXVIII"), Some(78));
    assert_eq!(u64::from_roman("LXXIX"), Some(79));
    assert_eq!(u64::from_roman("LXXX"), Some(80));
    assert_eq!(u64::from_roman("LXXXI"), Some(81));
    assert_eq!(u64::from_roman("LXXXII"), Some(82));
    assert_eq!(u64::from_roman("LXXXIII"), Some(83));
    assert_eq!(u64::from_roman("LXXXIV"), Some(84));
    assert_eq!(u64::from_roman("LXXXV"), Some(85));
    assert_eq!(u64::from_roman("LXXXVI"), Some(86));
    assert_eq!(u64::from_roman("LXXXVII"), Some(87));
    assert_eq!(u64::from_roman("LXXXVIII"), Some(88));
    assert_eq!(u64::from_roman("LXXXIX"), Some(89));
    assert_eq!(u64::from_roman("XC"), Some(90));
    assert_eq!(u64::from_roman("XCI"), Some(91));
    assert_eq!(u64::from_roman("XCII"), Some(92));
    assert_eq!(u64::from_roman("XCIII"), Some(93));
    assert_eq!(u64::from_roman("XCIV"), Some(94));
    assert_eq!(u64::from_roman("XCV"), Some(95));
    assert_eq!(u64::from_roman("XCVI"), Some(96));
    assert_eq!(u64::from_roman("XCVII"), Some(97));
    assert_eq!(u64::from_roman("XCVIII"), Some(98));
    assert_eq!(u64::from_roman("XCIX"), Some(99));
    assert_eq!(u64::from_roman("C"), Some(100));
}

#[test]
fn do_not_fit() {
    assert_eq!(u8::from_roman("D"), None);
    assert_eq!(u8::from_roman("M"), None);
    assert_eq!(u16::from_roman("L̄X̄X̄"), None);
    assert_eq!(u16::from_roman("X̄̄"), None);
    assert_eq!(u64::from_roman("X̄̄"), None);
}
