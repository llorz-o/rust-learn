// *********************  Generic 泛型

struct Point1<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

enum Result<T, U> {
    Ok(T),
    Err(U),
}

// **************************  trait
// trait 类似于 interface 但是又有独特性
trait Summary {
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String {
        // 也可以定义默认实现
        format!("summarize_author::{}", &self.summarize())
    }
}

pub struct A1 {
    pub headline: String,
    pub author: String,
}

impl Summary for A1 {
    fn summarize(&self) -> String {
        format!("{}-{}", &self.headline, &self.author)
    }
}

pub struct A2 {
    pub username: String,
    pub content: String,
}

impl Summary for A2 {
    fn summarize(&self) -> String {
        format!("{}-{}", &self.username, &self.content)
    }
}

trait T2 {
    fn t2() {}
}

// 参数限定需要实现 Summary
fn notify(item: impl Summary) {}
// 限定参数1和2为相同类型
fn notify2<T: Summary>(item: T, item2: T) {}
// 限定同时实现两个特性
fn notify3(item: impl Summary + T2) -> impl Summary {
    A2 {
        username: String::from("name"),
        content: String::from("content"),
    }
}
// 使用泛型限定
fn notify4<T: Summary + T2, U: T2>(item: T, item2: U) {}
// 使用 where 从句,缩减函数签名
fn notity5<T, U>(item: T, item2: U)
where
    T: Summary + T2,
    U: T2,
{
}

struct P2<T> {
    x: T,
    y: T,
}

// 对于任意类型实现 new 方法 [[对结构体实现]]
impl<T> P2<T> {
    fn new(x: T, y: T) {}
}
// 当结构体的类型为同时实现了 Summary 和 T2 的类型时,才会实现该方法 [[对结构体实现]]
impl<T: Summary + T2> P2<T> {
    fn st() {}
}

// 结构体P2 实现了特征 T2 [[对特征实现]]
impl T2 for P2<i32> {}

// 为任何实现了 T2 的类型实现 Summary特征, 上面的 P2 只有i32类型实现了 T2,也就是仅有 i32类型的结构体P2可以被实现 Summary
impl<T: T2> Summary for T {
    fn summarize(&self) -> String {
        format!("为任何实现了T2类型的类型实现 Summary类型")
    }
}

// ************** life cycle 生命周期

pub fn run() {
    let a = A1 {
        headline: String::from("今日头条"),
        author: String::from("jojo"),
    };
    println!("{}", a.summarize_author());

    let p = P2 { x: 12, y: 23 };
    P2::new(12, 34);
    p.summarize();

    // {
    //     // 这行代码产生了垂悬引用
    //     let a; // 初始化
    //     {
    //         let b = 23;
    //         a = &b;// `b` does not live long enough 变量b的寿命不足
    //     } // 在离开作用域时 引用 b 将失去有效期
    //
    //     println!("{}", a); // 使用了一个没有赋值的变量
    // }

    // 生命周期注解就是为了避免垂悬引用的,rust 无法直接分析出函数调用时的所有生命周期
    // 当一个函数的参数拥有 'a 与 'b 两个生命周期且 'a > 'b 那么,
    // 在函数的返回值与参数具有相关性时,函数返回值的生命周期将始终是 'b

    let rs: &str;
    // 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），
    // 而返回值的生命周期被称为 输出生命周期（output lifetimes）。
    // 生命周期写法
    // &str (引用变量)
    // &'a str (生命周期引用)
    // &'a mut str (生命周期可变引用)
    // 生命周期是一种泛型,只不过是标注引用有效性而不是用作参数类型约束
    fn t<'a, 'b>(a: &'a str, b: &'b str) -> &'a str {
        a
    }

    let a = String::from("a");
    {
        let b = String::from("b");
        // 方法 t 中同时声明了 生命周期 'a 'b 与返回值生命周期 'a,
        // 即返回值与参数 a 具有相同的声明周期(变量有效作用域)
        rs = t(&a, &b);
    }

    println!("{}", rs);

    // 当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。如果返回的引用 没有 指向任何一个参数，
    // 那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用，因为它将会在函数结束时离开作用域。

    // rust 可以自动推断某些函数的生命周期
    // 1. 每一个是引用的参数都有它自己的生命周期参数(有一个引用参数的函数有一个生命周期参数,引用参数才有生命周期,借用将移交所有权)
    // 2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
    // 3. 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，
    // 说明是个对象的方法(method), 那么所有输出生命周期参数被赋予 self 的生命周期。

    // 'static 静态生命周期 字符串字面量具有天然的静态生命周期
    // 'static 生命周期将贯穿整个程序,因此不要随意使用

}
