pub fn read_string() -> String {
    let mut read_str = String::new();
    std::io::stdin().read_line(&mut read_str);
    String::from(read_str.trim())
}

pub fn read_number(min: u16, max: u16) -> usize {
    0
}