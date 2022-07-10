fn main() {
    let my_number /*  middle comment  */ = 100; // Rust always chooses i32 for integers. if you don't tell it to use a different type 
    println!("{}", my_number as u8 as char ) ; //  100  -> d (unicode)
    
    let u8_number :u8 = 100;
    println!("{}", u8_number  ) ;  // 100

    // usize is used to represent the size of a memory
    // 또한 index에서도 사용.  32비트 컴퓨터에서는 64비트 주소를 사용 할수 없으니 usize (양수의 32비트 or 64비트)

    // chars use 4 bytes, 각 char에 맞게 최소 사이즈만 사용.
    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());


    let slice = "Hello!";
    println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
    let slice2 = "안녕!";
    println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());

    
}
