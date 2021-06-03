fn main() {
    last_char(String::from("Hello"));
}

fn last_char(string: String) -> char {
    if string.is_empty() {
        return '🚨';
    }
    string.chars().next_back().unwrap()
}
