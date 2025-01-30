fn main() {
    let food: &str = "🍕";
    println!("{}", food.len());
    let pizza_slice: &str = &food[0..4];
    println!("{pizza_slice}")
}
