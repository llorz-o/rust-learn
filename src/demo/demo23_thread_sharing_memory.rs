use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn run() {
    // 互斥锁
    let m = Mutex::new(5);

    {
        // 使用 lock 方法获取锁，以访问互斥器中的数据。这个调用会阻塞当前线程，直到我们拥有锁为止。
        // 注意 m 是一个 Mutex<i32> 实例,这将确保程序类型系统限制用户对数据的操作,必须 获取锁才能使用这个 i32 值
        // Mutex 提供内部可变性
        let mut num = m.lock().unwrap();
        // lock 调用 返回 一个叫做 MutexGuard 的智能指针。
        // 这个智能指针实现了 Deref 来指向其内部数据；其也提供了一个 Drop 实现当 MutexGuard 离开作用域时自动释放锁

        *num = 6;
    }

    println!("m = {:?}", m);

    /*
        What's a Arc?
        Arc 类似于 Rc,但是Arc可用于线程之间的共享,它实现了线程共享时必要的 trait => Send 这是确保所使用的类型可以用于并发环境的 trait 之一。
        同时 Arc 是一个原子性类型,一个可安全的用于并发环境的类型
        线程安全带有性能惩罚，我们希望只在必要时才为此买单。
        如果只是在单线程中对值进行操作，原子性提供的保证并无必要，代码可以因此运行的更快。
    */

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Mutex 依然无法规避逻辑上的死锁

    let a1 = Arc::new(Mutex::new(1));
    let a2 = Arc::new(Mutex::new(2));

    let b1 = Arc::clone(&a1);
    let b2 = Arc::clone(&a2);

    // 使用 lock 方法获取锁，以访问互斥器中的数据。这个调用会阻塞当前线程，直到我们拥有锁为止。
    let handle1 = thread::spawn(move || {
        let mut num = a1.lock().unwrap(); // A
        thread::sleep(Duration::from_millis(10));
        let mut num = a2.lock().unwrap(); // B ,当前 B 已被占用,线程阻塞
    });

    let handle2 = thread::spawn(move || {
        let mut num = b2.lock().unwrap(); // B
        thread::sleep(Duration::from_millis(10));
        let mut num = b1.lock().unwrap(); // A ,当前 A 已被占用,线程阻塞
    });

    // 线程阻塞时无法自动drop变量导致无法自动解锁,互斥锁将锁死主进程

    // 以上代码将产生互斥锁死锁
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("app end!!")
}
