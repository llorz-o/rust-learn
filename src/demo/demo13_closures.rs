use std::mem;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(i32) -> i32,
{
    calculation: T,
    value: Option<i32>,
}

impl<T> Cacher<T>
where
    T: Fn(i32) -> i32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn run() {
    // ** 闭包的完整类型说明
    // Fn：表示捕获方式为通过引用（&T）的闭包  [不可变引用]
    // FnMut：表示捕获方式为通过可变引用（&mut T）的闭包  [可变引用]
    // FnOnce：表示捕获方式为通过值（T）的闭包   [所有权]

    fn generate_workout<F>(intensity: i32, random_number: i32, f: &mut F)
    where
        F: FnMut(i32) -> i32, // 闭包的类型说明
    {
        if intensity < 25 {
            println!("今天,做{}个俯卧撑", f(intensity));
            println!("接下来,做{}个仰卧起坐", f(intensity));
        } else {
            if random_number == 3 {
                println!("今天休息一下！记住要保持水分！");
            } else {
                println!("今天,运动{}分钟", f(intensity));
            }
        }
    }

    // 使用缓存,缓存计算结果减少计算量
    let mut cache = Cacher::new(|num| {
        // 闭包
        println!("计算缓慢...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // 当闭包中出现了一个可变引用类型那么这个闭包本身就是可变引用类型
    generate_workout(30, 4, &mut |num| cache.value(num));
    generate_workout(30, 4, &mut |num| cache.value(num));

    let movable = Box::new(3);

    let cl = move || {
        // drop 方法将获得所有权
        mem::drop(movable); // 当前闭包失去所有权
    };

    cl();
    // cl(); // 使用所有权转移的方法在使用后将销毁变量,所以无法二次调用

    let a = 12;
    let b = String::from("String");
    let v = vec![1, 2];
    // 即使强制一个可复制(拷贝)类型移交所有权
    let pt = move || {
        println!("a:{}b:{}v:{:?}", a, b, v);
    };

    pt();
    pt();

    // println!("a:{}b:{}v:{:?}", a, b, v); b 和 v 都没有实现 Copy trait 所有
}
