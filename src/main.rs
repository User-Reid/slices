fn print_length(x: &[i32]) {
    println!("{}", x.len());
}

fn main() {
    let values: [i32; 6] = [4, 8, 15, 16, 23, 42];

    let regular_array: &[i32; 6] = &values;

    let slice_of_three: &[i32] = &values[..3];

    print_length(&values);
}
