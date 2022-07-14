fn main() {
    print!("\t Start with a tab\nand move to a new line");
    println!(
        "Inside quotes
you can write over
many lines
and it will print just fine."
    );

    println!("He said, \"You can find the file at c:\\files\\my_documents\\file.txt.\" Then I found the file."); // We used \ five times here
                                                                                                                 // r# is used to escape the string literal.
    println!(
        r#"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."#
    );

    let my_string = "'Ice to see you,' he said."; // single quotes
    let quote_string = r#""Ice to see you," he said."#; // double quotes
    let hashtag_string = r##"The hashtag #IceToSeeYou had become very popular."##; // Has one # so we need at least ##
    let many_hashtags =
        r####""You don't have to type ### to use a hashtag. You can just use #.""####; // Has three ### so we need at least ####

    println!(
        "{}\n{}\n{}\n{}\n",
        my_string, quote_string, hashtag_string, many_hashtags
    );

    let r#let = 6; // The variable's name is let
    let mut r#mut = 10; // This variable's name is mut

    let my_number = r#return();
    println!("{}", my_number);

    println!("{:?}", b"This will look like numbers"); // b -> $str or chat to bo number

    println!("{:X}", '행' as u32); // Cast char as u32 to get the hexadecimal value
    println!("{:X}", 'H' as u32);
    println!("{:X}", '居' as u32);
    println!("{:X}", 'い' as u32);

    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}"); // Try printing them with unicode escape \u

    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);

    let number = 555;
    println!(
        "Binary: {:b}, hexadecimal: {:x}, octal: {:o}",
        number, number, number
    );

    let father_name = "Vlad";
    let son_name = "Adrian Fahrenheit";
    let family_name = "Țepeș";
    println!(
        "This is {1} {2}, son of {0} {2}.",
        father_name, son_name, family_name
    );

    println!(
        "{city1} is in {country} and {city2} is also in {country},
but {city3} is not in {country}.",
        city1 = "Seoul",
        city2 = "Busan",
        city3 = "Tokyo",
        country = "Korea"
    );

    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // no variable name, pad with -, put in centre, 30 characters long
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 characters each, one to the left, one to the right
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b); // variable names city1 and city2, pad with -, one to the left, one to the right
}

fn r#return() -> u8 {
    // 함수도 r# 가능
    println!("Here is your number.");
    8
}
