fn main() {
    let action_hero: String = String::from("Arnold Banana");

    let first_name: &str = &action_hero[..6];
    println!("{first_name}");

    let last_name: &str = &action_hero[7..];
    println!("{last_name}");

    let full_name: &str = &action_hero[..];
}
