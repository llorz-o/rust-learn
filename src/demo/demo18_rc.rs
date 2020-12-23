use std::rc::Rc;

pub fn run() {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let a = Cons(1, Box::new(Cons(0, Box::new(Nil))));
    let b = Cons(2, Box::new(a));
    // let c = Cons(3, Box::new(a)); // 不允许共享所有权

    enum List2 {
        Cons2(i32, Rc<List2>), // Rc 仅允许多个不可变引用
        Nil2,
    }

    use List2::{Cons2, Nil2};

    // Rc<List2> 中数据的引用计数都会增加，直到有零个引用之前其数据都不会被清理
    let a = Rc::new(Cons2(1, Rc::new(Cons2(0, Rc::new(Nil2)))));
    // 也可以调用 a.clone() 而不是 Rc::clone(&a)，不过在这里 Rust 的习惯是使用 Rc::clone。
    // Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。
    // Rc::clone 只会增加引用计数，这并不会花费多少时间。

    println!("引用计数:{}", Rc::strong_count(&a));

    let b = Rc::new(Cons2(2, Rc::clone(&a)));

    println!("引用计数:{}", Rc::strong_count(&a));

    let c = Rc::new(Cons2(3, Rc::clone(&a)));

    println!("引用计数:{}", Rc::strong_count(&a));
}
