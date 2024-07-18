// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
    // main1();
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}

// fn main1() {
//     let s = String::from("Hello, world!");
//     let s_ref: &String = &s; // &String 类型的引用
//     let str_slice: &str = &s; // &str 类型的引用

//     let str_literal: &str = "Hello, world!"; // 字面量字符串，&str 类型

//     println!("s_ref: {}", s_ref);
//     println!("str_slice: {}", str_slice);
//     println!("str_literal: {}", str_literal);

//     // `String` 的一些操作
//     let mut s2 = String::from("Hello");
//     s2.push_str(", world!");
//     println!("s2: {}", s2);
//     println!("s_ref: {}", s_ref);
//     println!("s_ref: {}", s_ref);
//     println!("str_slice: {}", str_slice);
// }
