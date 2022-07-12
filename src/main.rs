fn main() {
    // 타입은 지정하지 않아도 자동으로 된다.
    //  정수는 기본적으로 i32
    let small_number: u8 = 10;
    let small_number2 = 10u8; // 숫자 뒤에 타입 지정해도됨
    let big_number = 100_000_000_i32; // _는 무시됨.

    // let my_float = 5.;  // .를 붙이면 float로 인식  float는 f32, f64(기본)
    let my_float: f64 = 5.0; // This is an f64
    let my_other_float: f32 = 8.5; // This is an f32 , f32와 f64는 더 할수 없다.

    let third_float = my_float + my_other_float as f64; // I can casting 

    // Rust 컴파일러가 자동으로 추측해서 my_other_float를 f32로 인식해줌. 기본은 f64이지만
    let my_float: f32 = 5.0;
    let my_other_float = 8.5; // Usually Rust would choose f64,
    let third_float = my_float + my_other_float; 

    println!("{}", third_float);
}
