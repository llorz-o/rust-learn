use ::std::ops::Deref;

pub fn run() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // assert_eq!(5, y); // err 当前y为引用类型无法与5进行比较
    assert_eq!(5, *y);

    let y = Box::new(x);

    assert_eq!(5, *y);

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    let y = MyBox::new(x);

    // assert_eq!(5, *y); // 无法解引用 为了启用 * 运算符的解引用功能，需要实现 Deref trait
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            // 通过实现 deref 后解引用将会解引用 deref 方法的返回,不实现 Deref trait 解引用编译时仅作用于 & 这种引用类型
            &self.0
        }
    }

    let y = MyBox::new(x);

    assert_eq!(5, *y); //  ==== > assert_eq!(5,*(y.deref()));

    // 解引用强制多态的加入使得 Rust 程序员编写函数和方法调用时无需增加过多显式使用 & 和 * 的引用和解引用。
    // 这个功能也使得我们可以编写更多同时作用于引用或智能指针的代码。

    fn hello(name: &str) {
        println!("Hello ,{} !", name);
    }

    let name = MyBox::new(String::from("jojo"));

    // 因为解引用多态,即使 当前的引用为 &Mybox<String> ,也能根据 deref 的实现转换为 &String
    // String 在标准库中实现了 Deref 将会转换为 &str
    // 如果没有解引用多态,实现 hello 的调用将会变得负责 hello(&(*name)[..])
    // 解引用的解析发生在编译时,没有运行时惩罚
    hello(&name);

    /*
    类似于如何使用 Deref trait 重载不可变引用的 * 运算符，Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符。

    Rust 在发现类型和 trait 实现满足三种情况时会进行解引用强制多态：

    当 T: Deref<Target=U> 时从 &T 到 &U。// 有一个 &T，而 T 实现了返回 U 类型的 Deref，则可以直接得到 &U
    当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。


    /// 第三个情况有些微妙：Rust 也会将可变引用强转为不可变引用。
    /// 但是反之是 不可能 的：不可变引用永远也不能强转为可变引用。
    /// 因为根据借用规则，如果有一个可变引用，其必须是这些数据的唯一引用（否则程序将无法编译）。
    /// 将一个可变引用转换为不可变引用永远也不会打破借用规则。
    /// 将不可变引用转换为可变引用则需要数据只能有一个不可变引用，而借用规则无法保证这一点。
    /// 因此，Rust 无法假设将不可变引用转换为可变引用是可能的。
    当 T: Deref<Target=U> 时从 &mut T 到 &U。

     */
}
