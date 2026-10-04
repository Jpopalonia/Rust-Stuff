fn main() {
    // shows variable ownership

    //let s1 = String::from("hello");
    //let s2 = s1;

    //println!("{s1}, world!");


    // shows function ownership with strings vs integers

    let mut s = String::from("hello2!");

    //change_string(&mut s, &mut s);
    
    //s = take_ownership(s);
    
    println!("{s}");
    // bruh ownership in Rust is hella weird, kinda makes sense but it's confusing

    //let x = 5;
    //make_copy(x);
    //println!("{x}");
}

fn change_string(string1: &mut String, string2: &mut String) {
    string1.push_str(", world!");
    string2.push_str(", world!");
}

// strings live in the heap so any string passed to this function is passed by reference, then dropped after this function call is completed
fn take_ownership(some_string: String) -> String {
    println!("{some_string}");
    some_string
}

// integers live on the stack so any integer passed to this function is copied
fn make_copy(some_integer: i32) {
        println!("{some_integer}");
}