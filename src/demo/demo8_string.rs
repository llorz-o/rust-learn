pub fn run() {
    let mut s = String::new();
    let str1 = "hello world!";
    s = str1.to_string(); // to_string 方法，它能用于任何实现了 Display trait 的类型
    let mut str1 = String::from("hello world!");
    let str2 = "s2";
    str1.push_str(" aaa");
    str1.push_str(str2); // 实现采用slice类型,不会产生所有权移交
    str1.push('f'); // 添加单个字符
    let str3 = str1 + &str2; // 运算符使用了 add 函数签名 fn add(self, s: &str) -> String
                             // str1 会失去所有权
    let str4 = format!("{}-{}-{}", "1", "2", "3"); // 使用宏拼接字符
    println!("{}", str4);
    // String 内部是一个 Vec(u8) 的实现,无法通过索引访问单个字符
    let len = String::from("Hello").len(); // => 5 返回 Vec 的长度

    // 因为每个 Unicode标量值需要两个字节存储,因此一个字符串字节值的索引并不总是对应一个有效的 Unicode 标量值。
    let len = String::from("Здравствуйте").len(); // => 24 [224,....23]
                                                  // 如果使用下标来获取有效字符,那么rust将始终检索 vector的长度,来确定有效字符,且索引返回的值类型通常无法确定

    // 使用字符串切片时需要明确指定合法的字符下标
    let s = &str3[0..4];

    for c in str3.chars() {
        println!("char:{}", c)
    }

    for b in str3.bytes() {
        println!("bete:{}", b)
    }
}
