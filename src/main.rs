fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day!")
}

fn main() {
    let action_hero: String = String::from("Arnold Banana");
    let another_action_hero: &str = "Taco bell";

    do_hero_stuff(&action_hero);
    do_hero_stuff(another_action_hero);
}
