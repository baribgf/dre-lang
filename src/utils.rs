pub fn bytes_to_string(bytes: &[u8]) -> String {
    unsafe { String::from_utf8_unchecked(bytes.to_vec()) }
}

pub fn get_nth_line(text: &[u8], line: usize) -> String {
    bytes_to_string(text.split(|c| *c == b'\n').nth(line - 1).unwrap())
}
