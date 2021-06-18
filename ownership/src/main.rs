fn main() {
    let mut say = String::from("Ca");

    say.push_str("t");

    let say2 = say;

    // This line will return an error
    // println!("{}", say); 

    println!("{}", say2);

}
