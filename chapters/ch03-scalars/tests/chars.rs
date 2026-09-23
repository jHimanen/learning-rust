use ch03_scalars::*;

#[test]
fn digit_value_works() {
    assert_eq!(digit_value('0'), 0);
    assert_eq!(digit_value('7'), 7);
    assert_eq!(digit_value('9'), 9);
}

#[test]
fn ascii_to_upper_works() {
    assert_eq!(ascii_to_upper(b'a'), b'A');
    assert_eq!(ascii_to_upper(b'm'), b'M');
    assert_eq!(ascii_to_upper(b'z'), b'Z');
}

#[test]
fn utf8_len_works() {
    assert_eq!(utf8_len('a'), 1); // ASCII
    assert_eq!(utf8_len('é'), 2); // Latin-1 supplement
    assert_eq!(utf8_len('€'), 3);
    assert_eq!(utf8_len('🦀'), 4); // outside the Basic Multilingual Plane
}
