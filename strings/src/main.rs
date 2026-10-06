fn main() {
    let mut needs_trimmed = "Hello!     ".to_string();
    trim_me(&needs_trimmed);
    println!("{needs_trimmed}");

    let mut needs_composed = "Hello".to_string();
    needs_composed = compose_me(&needs_composed);
    println!("{needs_composed}");

    let mut needs_replaced = "I think cars are cool".to_string();
    replace_me(&needs_replaced);
    println!("{needs_replaced}");
}

fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.

    input.trim()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.

    format!("{input} world!")
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".

    input.replace("cars", "balloons")
}