// 在rust中所有的声明都默认为私有的 使用 pub 公开访问

// lib.rs 与 main.rs 为模块树的隐式根crate
// 使用 mod 声明模块层级(模块树)
// 父模块不能使用子模块的私有项,子模块可以使用父模块的私有项
pub mod front_of_house {
    mod hosting {
        pub fn add_to_waitlist() {}
    }

    pub mod serving {
        pub fn task_order() {
            // super 回退上一级模块,相当于 ../
            // super 为相对调用,便于模块内的灵活性
            super::hosting::add_to_waitlist()
        }
    }

    // 结构体的属性需要访问描述符
    pub struct demo {
        pub name: String,
        age: u8,
    }
    // 枚举的字段则视当前的枚举而定,枚举为公开则字段为公开
    pub enum demo2 {
        Name,
        Age,
    }
}

pub use crate::front_of_house::serving; // 使用 pub use 将导入重导出

// use front_of_house::serving;
use front_of_house::serving as s;
use std::fmt;
use std::io;

// 如果当前函数随意移动那么使用绝对路径将不用改动
// 如果模块与调用函数一起移动至另一个模块下那么相对路径不用改动
pub fn eat_at_restaurant() {
    // crate 为隐式的根,当前为绝对路径引用[以 crate 名或者字面值 crate 开头]
    // crate::front_of_house::hosting::add_to_waitlist(); // 私有访问报错

    // 相对路径引用[以 self、super 或当前模块的标识符开头]
    // front_of_house::hosting::add_to_waitlist(); // 私有访问报错

    crate::front_of_house::serving::task_order();
    front_of_house::serving::task_order(); // 为模块与模块内的方法提供公开属性即可外部访问

    serving::task_order();
    s::task_order();
}

// 当导入具有相同命名时可使用父模块引用
// fn functional1() -> fmt::Result {}
// fn functional2() -> io::Result<()> {}
