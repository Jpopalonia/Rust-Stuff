fn main() {
    // shows variable ownership

    //let s1 = String::from("hello");
    //let s2 = s1;

    //println!("{s1}, world!");


    // shows function ownership with strings vs integers

    let s = String::from("hello2!");

    take_ownership(s);
    
    println!("{s}"); // shouldn't work because s is owned by previous function?
    // bruh ownership in Rust is hella weird, kinda makes sense but it's confusing

    let x = 5;

    makes_copy(x);

    println!("{x}");
}

// strings live in the heap so any string passed to this function is passed by reference, then dropped after this function call is completed
fn take_ownership(some_string: String) {
    println!("{some_string}");
}

// integers live on the stack so any integer passed to this function is copied
fn makes_copy(some_integer: i32) {
        println!("{some_integer}");
}