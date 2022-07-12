
fn number() -> i32 {
    8 // ; 가 없어야함.
}

fn multiply(number_one: i32, number_two: i32) { // Two i32s will enter the function. We will call them number_one and number_two.
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
}

fn main() { // code block start
    println!("Hello, world! {}", number()); // println is a macro  , ! is means that it is a macro
    multiply(3, 5);

    let my_number = {
    let second_number = 8;
        second_number + 9 // No semicolon ;, so the code block returns 8 + 9.
                          // It works just like a function
    };

    println!("My number is: {}", my_number);
} 

