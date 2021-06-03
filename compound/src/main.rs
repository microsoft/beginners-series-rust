fn main() {
    let array = [1u32, 2, 3];
    // let array: [u32; 3] = [1u32, 2, 3];
    // let array: [u32; 3] = [1u32, 2, 3, 4];

    let first_element = array[0];
    // let warning = array[100];

    let length = "Some text".len();
    [1][length];

    // let array = [1, 2, true];

    let tuple = (1u32, 2, true);
    // let tuple: (u32, i8, bool) = (1u32, 2, true);

    let first_element = (1, 2, true).0;
    // let error = (1, 2, true).100;
}
