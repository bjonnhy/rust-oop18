pub fn read_string() -> String {
    let mut read_str = String::new();
    std::io::stdin().read_line(&mut read_str).unwrap();
    String::from(read_str.trim())
}

pub fn _read_number(_min: u16, _max: u16) -> usize {
    0
}