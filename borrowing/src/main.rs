fn main() {
    let say = String::from("Cat");
    print_out(&say);
    println!("Again: {}", say);

    let mut my_vec = vec![1, 2, 3];
    println!("{:?}", my_vec);
    add_to_vec(&mut my_vec);
    println!("{:?}", my_vec);
}

fn print_out(to_print: &String) {
    println!("{}", to_print);
}

fn add_to_vec(a_vec: &mut Vec<i32>) {
    a_vec.push(4);
}