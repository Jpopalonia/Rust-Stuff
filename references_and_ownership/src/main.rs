// bruh ownership in Rust is hella weird
// kinda makes sense but it's confusing too

fn main() {
    // this breaks because you cannot shallow copy any data that lives in the heap
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");
    
    // this breaks because you cannot create more than one mutable reference to any data at a time, it's okay to make one, free the reference, then create another
    let mut s = String::from("hello2!");

    change_string(&mut s, &mut s);

    // this function takes ownership of 's', then frees it when the function call is finished
    // any use of 's' after this function call won't work
    let free_string = String::from("I'm free!");

    take_ownership(free_string);

    // this function takes ownership of the string that is passed in, then returns it
    let 2ndfree_string = take_and_return(free_string);
    
    // this breaks because dangle() tries to return a pointer to memory that was freed at the end of the function call
    let fakestring = dangle();

    println!("{fakestring}");

    // this works fine because integers are of a known size, live on the stack, and implement the 'Copy' property
    let x = 5;
    make_copy(x);
    println!("{x}");
}

// returns a reference to memory that gets freed after function call
fn dangle() -> &String {
    let refString = String::from("refHello!");

    &refString
}

// append to a string twice with separate references
fn change_string(string1: &mut String, string2: &mut String) {
    string1.push_str(", world!");
    string2.push_str(", world!");
}

// takes ownership of a string that is passed in
fn take_ownership(some_string: String) {
    println!("{some_string}");
}

// takes ownership of a string that is passed in, then returns ownership back to the caller
fn take_and_return(other_string: String) -> String {
    println!("{other_string}");

    other_string
}

// makes a copy of an integer that is passed in since integers live on the stack and can be copied easily
fn make_copy(some_integer: i32) {
    println!("{some_integer}");
}