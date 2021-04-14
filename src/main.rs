mod demo;
mod exercise;

use crate::demo::*;
use crate::exercise::*;

fn main() {
    // println("函数调用"); // 函数调用
    println!("宏调用"); // 调用了一个 Rust 宏（macro）,带!的方式代表宏调用
    demo32_macro::run();
}
