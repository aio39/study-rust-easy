fn main() {
    let my_number;
    my_number = 8;

    let my_number_2 = 8;
    // my_number = 10; error !

    let mut mutable_number = 8;
    mutable_number = 10;

    let mut my_variable = 8; // it is now an i32. That can't be changed
                             // my_variable = "Hello, world!"; // error !

    // shadowing
    let my_number = 8; // This is an i32
    println!("{}", my_number); // prints 8
    let my_number = 9.2; // This is an f64 with the same name. But it's not the first my_number - it is completely different!
    println!("{}", my_number); // Prints 9.2

    // "code block" block shadowing
    let my_number = 8; // This is an i32
    println!("{}", my_number); // prints 8
    {
        let my_number = 9.2; // This is an f64. It is not my_number - it is completely different!
        println!("{}", my_number) // Prints 9.2
                                  // But the shadowed my_number only lives until here.
                                  // The first my_number is still alive!
    }
    println!("{}", my_number); // prints 8
}
