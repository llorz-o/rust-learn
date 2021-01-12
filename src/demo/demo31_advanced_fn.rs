pub fn run() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let anser = do_twice(add_one, 5);

    println!("{}", anser);

    // 函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce），所以总是可以在调用期望闭包的函数时传递函数指针作为参数。
    // 倾向于编写使用泛型和闭包 trait 的函数，这样它就能接受函数或闭包作为参数。
    let list_of_numbers = vec![1, 2, 3, 4];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    enum Status {
        Value(u32),
        Stop,
    }
    // 快速产生一些类型的值
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    // 如何返回一个闭包
    fn returns_closure(y: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |x| x + y)
    }

    let add = returns_closure(12);

    let some = &add(12);

    println!("{}", some); // => 24
}
