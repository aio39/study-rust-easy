fn main() {
    let name = "서태지"; // $str is very fast
    let other_name = String::from("Adrian Fahrenheit Țepeș"); // String is a bit slower, but more function / Heap / owned type / 24 bytes

    // str is a dynamically sized type
    // str은 가변 길이 이기때문에 pointer로 접근해야한다 (포인터는 사이즈가 같음.)

    let a = "This is the string text".to_string();

    let my_name = "Billybrobby";
    let my_country = "USA";
    let my_home = "Korea";

    let together = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );

    let my_string: String = "Try to make this a String".into(); // String::from()과는 다르게, into()는 변형 가능한 타입이 다양하므로 명시적으로 적어줘야한다.
}
