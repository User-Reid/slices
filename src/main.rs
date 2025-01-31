fn main() {
    let mut my_array: [i32; 5] = [10, 15, 20, 25, 30];

    let my_slice: &mut [i32] = &mut my_array[2..4];
    println!("{my_slice:?}");

    my_slice[0] = 100;
    println!("{my_slice:?}");
    println!("{my_array:?}")
}
