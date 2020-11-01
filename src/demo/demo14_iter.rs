struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn run() {
    let v = vec![1, 2, 3];
    // 迭代器中每次调用 next 方法都将消费迭代器,因为next方法将改变当前下标的记录
    // map 方法为迭代器适配器,他可以产生一个新的迭代器,但是该方法是一个惰性的方法,即本身并不消费迭代器,该方法将在collect之后调用
    // 使用 collect 消费一个迭代器并将结果收集产生一个新的数据结构
    let v: Vec<_> = v.iter().map(|x| x + 1).collect();

    println!("{:?}", v);

    let mut c = Counter::new();

    for _ in 0..6 {
        let v = c.next();
        println!("{:?}", v);
    }
}
