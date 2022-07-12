fn main() {
    //  stack - is very super fast
    //  exact size를 알 경우에만 사용 가능
    //  heap - is not so fst
    //  any size of data
    //  pointer -> stack -> heap
    //  pointer is called a reference

    let my_number = 15; // This is an i32
    let single_reference = &my_number; //  This is a &i32
    let double_reference = &single_reference; // This is a &&i32
    let five_references = &&&&&my_number; // This is a &&&&&i32
}
