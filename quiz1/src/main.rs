fn main() {
    let one = calculate_price_of_apples(35);
    let two = calculate_price_of_apples(40);
    let three = calculate_price_of_apples(41);
    let four = calculate_price_of_apples(65);

    println!("{one}");
    println!("{two}");
    println!("{three}");
    println!("{four}");
}

fn calculate_price_of_apples(quantity: u32) -> u32 {
    // An apple costs 2, unless 40 or more are purchased in 1 transaction, then 1 each
    if quantity < 40 {
        quantity * 2
    } else {
        quantity * 1
    }
}