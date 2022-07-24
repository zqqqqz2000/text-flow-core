pub fn b<T>(i: T) -> Box<T> {
    Box::new(i)
}

pub fn slice_end_str(s: String, start: usize, pos_to_end: usize) -> String {
    let str_len = s.len();
    assert!(str_len >= start && str_len >= pos_to_end);
    s[start..str_len - pos_to_end].to_string()
}

#[test]
fn test_slice_end_str() {
    assert_eq!("asdf", slice_end_str("1asdf2".to_string(), 1, 1))
}

#[test]
fn test_b() {
    assert_eq!(Box::new("asdf"), b("asdf"))
}
