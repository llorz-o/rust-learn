pub fn run() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<i32, _> = "32".parse();

    /* if else 逻辑语句 */

    if let Some(color) = favorite_color {
        println!("喜爱的颜色:{}", color)
    } else if is_tuesday {
        println!("今天星期二")
    } else if let Ok(age) = age {
        if age > 20 {
            println!("成年人模式")
        } else {
            println!("老年人模式")
        }
    } else {
        println!("这儿什么也没有!")
    }

    /* while 循环语句 */

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);

    while let Some(top) = stack.pop() {
        println!("{}", top)
    }

    /* for 循环 */

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index)
    }

    /* 元组解构 */

    let (a, b, c) = (1, 2, 3);
    // let (a, b) = (1, 2, 3); // 必须等量解构
    println!("元组结构 a:{} b:{} c:{}", a, b, c);

    /* 函数参数解构 */

    fn print_coordinates(&(a, b): &(i32, i32)) {
        println!("{}-{}", a, b)
    }

    print_coordinates(&(1, 2));

    /*
        模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）。
        能匹配任何传递的可能值的模式被称为是 不可反驳的（irrefutable） let x = 2
        对某些可能的值进行匹配会失败的模式被称为是 可反驳的（refutable）。if let Some(x) = a {}
    */

    /* match 匹配 */
    let x = 1;
    match x {
        1 => println!("one"),
        _ => println!("anything"),
    }

    // 分支将命中Some(y),因为Some(y)将在当前作用域创建变量y,来接收Some的值
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("x:{}", 50),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    // 1 or 2
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 1-5
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // char 类型的范围匹配
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    /* 结构体解构 */
    struct Point {
        x: i32,
        y: i32,
    }

    let a = Point { x: 1, y: 2 };
    // 解构与重命名
    let Point { x, y: local_y } = a;
    match a {
        Point { x, y: 0 } => println!("On the x axis at {}", x), // 这里y 会匹配 y:0
        Point { x: 0, y } => println!("On the y axis at {}", y), // 这里x 会匹配 x:0
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    /* 枚举解构 */
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // 嵌套结构体和枚举
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    // 复杂的嵌套结构体或元组
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // 下滑线开头的变量名不会产生未使用变量警告
    let _x = 1;
    let y = 2;

    // 丢弃其它值
    let Point { x, .. } = Point { x: 1, y: 2 };

    // 匹配守卫
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    //
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // 
    enum Message3 {
        Hello { id: i32 },
    }
    let msg = Message3::Hello { id: 5 };
    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7, // id_variable 匹配 id 为 3-7 的范围并将值绑定至 id_variable 变量中
        } => println!("Found an id in range: {}", id_variable),
        Message3::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}
