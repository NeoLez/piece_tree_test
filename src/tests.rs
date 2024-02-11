use crate::fixed_length_string::FixedSizeString;

#[test]
fn empty_state() {
    let str = FixedSizeString::new(4);
    let l1 = str.len();
    let slice1 = str.full_string();
    let lines = &str.line_starts();

    assert_eq!(l1, 0);
    assert!(slice1.is_empty());
    assert!(lines.is_empty());
}

#[test]
fn append_whole() {
    let mut s = FixedSizeString::new(10);
    if let Err(()) = s.append_whole(&"123".to_string()) {panic!("Unable to append")}

    assert_eq!(s.len(), 3);
    assert_eq!(s.full_string(), "123");
    assert!(s.line_starts().is_empty());
    
    if let Err(()) = s.append_whole(&"4567\n".to_string()) {panic!("Unable to append")}

    assert_eq!(s.len(), 8);
    assert_eq!(s.full_string(), "1234567\n");
    assert_eq!(s.line_starts()[0],8);

    if let Ok(()) = s.append_whole(&"910".to_string()) {panic!("Appended when it shouldn't have")}

    assert_eq!(s.len(), 8);
    assert_eq!(s.full_string(), "1234567\n");
    assert_eq!(s.line_starts()[0],8);

    if let Err(()) = s.append_whole(&"8\n".to_string()) {panic!("Unable to append")}

    assert_eq!(s.len(), 10);
    assert_eq!(s.full_string(), "1234567\n8\n");
    assert_eq!(s.line_starts()[0],8);
    assert_eq!(s.line_starts()[1],10);
}
#[test]
fn append() {
    let mut s = FixedSizeString::new(10);
    s.append(&"ab本".to_string());
    let l1 = s.len();
    let slice1 = s.full_string();

    assert_eq!(l1, 3);
    assert_eq!(slice1, "ab本");

    s.append(&"fgh".to_string());
    let l1 = s.len();
    let slice1 = s.full_string();

    assert_eq!(l1, 6);
    assert_eq!(slice1, "ab本fgh");

    let bw = s.append(&"i愛".to_string());
    let l1 = s.len();
    let slice1 = s.full_string();

    assert_eq!(l1, 7);
    assert_eq!(slice1, "ab本fghi");
    assert_eq!(bw,1);

    let bw = s.append(&"a".to_string());
    let l1 = s.len();
    let slice1 = s.full_string();

    assert_eq!(l1, 8);
    assert_eq!(slice1, "ab本fghia");
    assert_eq!(bw,1);
}

#[test]
fn creation_from_string() {
    let mut s = FixedSizeString::from_string("Texto de ejemplo".to_string());
    if let Ok(()) = s.append_whole(&"a".to_string()){panic!("Appended when it shouldn't")}
    assert_eq!(s.len(), 16);
    assert_eq!(s.full_string(), "Texto de ejemplo");
}

#[test]
fn str_slice_graphemes() {
    let s = FixedSizeString::from_string("नमस्ते de e本jemplo".to_string());
    
    assert_eq!(s.string_slice_graphemes((0, 3)), "नमस्");
    assert_eq!(s.string_slice_graphemes((0, 1)), "न");
    assert_eq!(s.string_slice_graphemes((9, 2)), "本j");
    assert_eq!(s.string_slice_graphemes((9, 7)), "本jemplo")
}
#[test]
fn str_slice_chars() {
    let s = FixedSizeString::from_string("Text de e本jeनमस्ते".to_string());
    
    assert_eq!(s.string_slice_chars((0, 3)), "Tex");
    assert_eq!(s.string_slice_chars((0, 1)), "T");
    assert_eq!(s.string_slice_chars((9, 2)), "本j");
    assert_eq!(s.string_slice_chars((9, 9)), "本jeनमस्ते");
    assert_eq!(s.string_slice_chars((15, 3)), "्ते");
}