fn main() {
    
    let mut i = 1;
    let something = loop {
        i *= 2;
        if i > 100 {
            break i;
        }
    };
    assert_eq!(something, 128);

    let mut counter = 0;

    while counter < 10 {
        println!("hello");
        counter = counter + 1;
    }

    for item in 0..5 {
        println!("{}", item*2);
    }
}
