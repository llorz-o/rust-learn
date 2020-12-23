use ::std::mem::drop;

pub fn run() {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("清除当前数据 `{}` !", self.data);
        }
    }

    let a = CustomSmartPointer {
        data: String::from("this a"),
    };

    // 当我们希望在作用域结束之前就强制释放变量的话，我们应该使用的是由标准库提供的 std::mem::drop。
    // a.drop(); 实例上的 drop 方法并不能直接调用,Rust 不允许我们显式调用 drop 因为 Rust 仍然会在 main 的结尾对值自动调用 drop，
    // 这会导致一个 double free 错误，因为 Rust 会尝试清理相同的值两次。
    drop(a); // 调用 drop 提前清理

    let b = CustomSmartPointer {
        data: String::from("this b"),
    };

    println!("当前实例已创建!");

    // 实现 Drop 后,在引用销毁时将会执行 drop 方法
}
