// 使用 注解可在输出中打印格式化的值
#[derive(Debug)]
struct Ipv4Addr {}

#[derive(Debug)]
enum Demo {}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    Ipv4(Ipv4Addr),
    Demo(Demo),         // 任意类型的值甚至枚举
    V6(String),         // 枚举成员的关联值
    IP(u8, u8, u8, u8), // 枚举成员支持任意结构体
}
// 为枚举定义方法
impl IpAddrKind {
    fn call(&self) {
        println!("running fn call");
    }
}

pub fn run() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6(String::from("127.0.0.1"));
    let ip = IpAddrKind::IP(127, 0, 0, 1);
    ip.call();

    // rust 中没有内置的空类型,而是提供内置的可空类型的枚举
    // name 并不能直接用于运算
    let mut name: Option<String> = Some(String::from("nihao"));
    name = None;
    // 可空枚举类型确保数据的安全性
    match name {
        Option::Some(val) => println!("{:?}", val),
        Option::None => (),
    }

    // 枚举默认是穷尽的,即你需要列出所有可匹配的枚举类型
    match v4 {
        IpAddrKind::V4 => (),
        _ => (), // 或者使用通配
    }

    // 或者简写为 if let 即仅匹配需要的,忽略其他的
    if let IpAddrKind::V4 = v4 {
        println!("if let")
    }
    // else 是可选的
    else {
        println!("else")
    }
}
