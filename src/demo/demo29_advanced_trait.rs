pub fn run() {
    /*
        由于孤儿规则（orphan rule）,我们无法对类型实现 trait,
        它说明只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该 trait。

        一个绕开这个限制的方法是使用 newtype 模式（newtype pattern）
        它涉及到在一个元组结构体中创建一个新类型。
        这个元组结构体带有一个字段作为希望实现 trait 的类型的简单封装。
        接着这个封装类型对于 crate 是本地的，这样就可以在这个封装上实现 trait。
        Newtype 是一个源自 （U.C.0079，逃） Haskell 编程语言的概念。

        使用这个模式没有运行时性能惩罚，这个封装类型在编译时就被省略了。
    */

    use std::fmt;
    use std::ops::Deref;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(","))
        }
    }

    // 使用智能指针直接使用 Vec 的方法
    impl Deref for Wrapper {
        type Target = Vec<String>;
        fn deref(&self) -> &Vec<String> {
            &(self.0)
        }
    }

    let w = Wrapper(vec![String::from("Hello"), String::from("world")]);

    for s in w.iter() {
        println!("{}", s);
    }

    println!("w = {}, w_len= {}", w, w.len())

    // 此方法的缺点是，因为 Wrapper 是一个新类型，它没有定义于其值之上的方法；
    // 必须直接在 Wrapper 上实现 Vec<T> 的所有方法，这样就可以代理到self.0 上 —— 这就允许我们完全像 Vec<T> 那样对待 Wrapper。
    // 如果希望新类型拥有其内部类型的每一个方法，为封装类型实现 Deref trait并返回其内部类型是一种解决方案。
    // 如果不希望封装类型拥有所有内部类型的方法 —— 比如为了限制封装类型的行为 —— 则必须只自行实现所需的方法。
}
