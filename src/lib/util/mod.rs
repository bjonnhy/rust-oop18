pub mod read;
pub mod write;

pub fn valid_name(name: &str) -> bool {
    name.chars().all(char::is_alphabetic)
}

pub fn valid_number(number: &str) -> bool {
    number.chars().all(char::is_numeric)
}

pub fn valid_address(address: &str) -> bool {
    true
}
pub fn eq_ignore_case(left: &str, right: &str) -> bool {
    left.to_lowercase() == right.to_lowercase()
}