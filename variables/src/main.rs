fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 21;
    println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1; // 'shadows' first declaration

    {
        let x = x * 2; // 'shadows' second declararion, is lost when this scope ends
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}