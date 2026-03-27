pub fn get_nth_line(text: &[u8], line: usize) -> String {
    unsafe {
        String::from_utf8_unchecked(text.split(|c| *c == b'\n').nth(line - 1).unwrap().to_vec())
    }
}
