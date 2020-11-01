pub fn run() {
    // 1/Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
    // 2.值在任一时刻有且只有一个所有者。
    // 3.当所有者（变量）离开作用域，这个值将被丢弃。

    // hello 被称为字面量,字面量是可以直接硬编码的,是不可变的
    // 但是字符串的值并不一定是从字面量中得到的(用户输入,网络获取...)
    // 字符串想要可变,且在编译时不确定,那么必须要为字符串分配一块内存
    {
        let s = String::from("hello"); // 1
        let s2 = s; // 2 rust 为了保证内存回收的安全性,在s赋值给s2后将去除自身的引用,所有权转移
        let s3 = s2.clone(); // 使用clone 函数可以保证数据的深拷贝,即拷贝堆中的数据,当然深拷贝的性能更差

        // println!(s); // 即你无法使用s变量

        let a = 1;
        let b = a;

        // 所有权移交只出现在引用类型中,即数据储存在堆中的数据类型
        // 简单类型的数据使用了Copy注解,来拷贝栈中的数据
        // 实现 Drop 注解的类型无法实现 Copy 注解
        // 元组包含的数据类型是简单类型时,也是Copy的,反之则不是
        println!("a:{}  b:{}", a, b);

        let s4 = takes_ownership(s3); // 引用类型将所有权移交给函数的参数,函数执行结束时 drop 了该内存块
                                      // println!("{}", s3); // 所以这里的 s2 无法再次引用,而简单类型则不会产生这种问题
    } // 3 在代码块的结尾处 s 会被系统自动调用的drop回收

    // 函数的所有权移交在实际使用时会显得太过繁琐,因为需要每次都去接收
    let s = String::from("test reference");
    reference_variable(&s);
    println!("使用引用时无需移交所有权:{}", s);

    let mut s2 = String::from("init");
    can_cange_reference_variable(&mut s2);
    let s3 = &mut s2;
    let s4 = &mut s2;
    let s5 = &s2;
    println!("s2:{}", s2);
    // 在特定作用域中的特定数据只能有一个可变引用
    // 也不能在拥有不可变引用的同时拥有可变引用
    // 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    // 为了避免数据竞态
    // println!("s3:{}", s3)

    // Slice 类型,没有所有权的数据类型,引用类型
    // slice 允许你引用集合中一段连续的元素序列,而不用引用整个集合
    let slice = &s[0..5];
    let slice2 = &s[0..];
    let slice3 = &s[..s.len()];
    let slice4 = &s[..];

    println!("first ward:{}", &s[0..find_first_ward(&s)]);
}

fn takes_ownership(str: String) {
    println!("{}", str);
    // 返回参数可移交所有权
    // ("12",str); 使用元组返回多个参数
}

fn reference_variable(str: &String) {
    println!("{}", str);
}

fn can_cange_reference_variable(str: &mut String) {
    str.push_str("ss");
}

fn dangling() -> String {
    let s = String::from("string");
    // 引用类型的源在当前作用域下创建,但是返回引用时,当前作用域会drop引用源
    // 所以这样返回引用类型是无效的,这被称为悬垂引用
    // 你可以选择返回引用源
    // &s
    s
}

// 找到第一个单词并返回
fn find_first_ward(str: &String) -> usize {
    let bytes = str.as_bytes();
    for (i, &v) in bytes.iter().enumerate() {
        if v == b' ' {
            return i;
        }
    }
    str.len()
}
