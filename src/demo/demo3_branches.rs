pub fn run() {
    // 在rust中if判断只能使用 bool 值
    if true {
        println!("true");
    }

    let mut count = if true { 1 } else { 5 }; // 使用表达式

    loop {
        count += 1;
        println!("loop 循环");
        if count >= 5 {
            break;
        };
    }

    while count != 0 {
        println!("while 循环");
        count -= 1;
    }

    let a = [1, 2, 3, 4, 5];

    for elem in a.iter() {
        println!("for 循环 :{}", elem);
    }

    for elem in (1..9) {}
}
