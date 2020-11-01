pub fn run() {
    // 创建一个空的 vector 类型
    let v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3]; // vector 中仅允许一种类型的数据,具有类型推断
    let mut v2 = vec![1, 2, 3];
    v2.push(4); // 可变类型

    let third: &i32 = &v2[2]; // 使用下标取值有可能导致 panic!,可使用 get(i) 取值

    // 当你在使用借用时,在 vector 的结尾增加新元素时，
    // 在没有足够空间将所有所有元素依次相邻存放的情况下，
    // 可能会要求分配新内存并将老的元素拷贝到新的空间中。
    // 这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
    // let second = &v2[1];
    // v2.push(5);
    // println!("the second vector item:{}", second); // 这将无法通过编译

    for i in &mut v2 {
        // * 解引用操作符
        // 修改可变引用指向的值之前必须使用解引用运算符获取i中的值
        *i += 10;
        println!("i:{}", i)
    }

    // 使用枚举可在vector中储存多种类型的数据
    enum Test {
        Age(i8),
        Name(String),
    };
    let v3 = vec![Test::Age(1), Test::Name(String::from("name"))];
}
