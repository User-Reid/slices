fn main() {
    let values: [i32; 6] = [4, 8, 15, 16, 23, 42];

    let my_slice: &[i32] = &values[0..3];

    println!("{my_slice:?}");

    let my_slice: &[i32] = &values[2..4];
    println!("{my_slice:?}")
}
