fn main() {
    let text = String::from("Hello\nworld\n!");
    let upper = text.to_uppercase();
    let stripped = upper.strip_prefix("HELLO\n").unwrap();

    println!("{}", first_line(&stripped))
}

// pub fn first_line(string: String) -> String {
//     string.lines().next().unwrap().to_owned()
// }

pub fn first_line(string: &str) -> &str {
    string.lines().next().unwrap()
}
