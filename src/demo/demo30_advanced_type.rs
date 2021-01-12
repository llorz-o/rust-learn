pub fn run() {
    type Kilometers = i32; // 类型别名的主要用途是减少重复。缩短长类型
    type Thunk = Box<dyn Fn() + Send + 'static>;

    // Rust 有一个叫做 ! 的特殊类型。在类型理论术语中，它被称为 empty type，因为它没有值。我们更倾向于称之为 never type。
    // 不返回的函数被称为 发散函数（diverging functions）。不能创建 ! 类型的值，所以 函数 也不可能返回值。

    // Sized 是一个特殊的 Trait 用来约束类型对于编译时可知大小的限定,与之相对的是动态类型 非Sized类型 (dyn Trait,&str)
    // Rust 隐式的为每一个泛型函数增加了 Sized bound。
    fn test<T: Sized>(t: T) {}
    // ?Sized 意为 T 可能是 Sized 也可能不是 Sized的
    fn test2<T: ?Sized>(t: &T) {}

    /*
        对于 &str 我们可以有如下理解
        对于每个相同类型编译器需要确定的知道他们的大小
        i32,u32,bool等
        但是对于字符串类型 (str 并不可以直接用于字符串类型,这对于编译器是不通过的,因为字符串天然为动态且不确定大小的)

        let a:str = "aaaaa";
        let b:str = "bbbbbbbbbbbbbbbbbbbbbbbbb";

        字符串是由slice实现,其内部包含了一个字符的起始位和一个长度的信息
        所以&str是由一个 str的起始地址和其长度组成的引用

        所以动态大小只能以引用的方式表现
        这引出了动态大小类型的黄金规则：必须将动态大小类型的值置于某种指针之后。

    */
}
