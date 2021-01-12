use std::thread;
use std::time::Duration;

pub fn run() {
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("当前线程:{}", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..10 {
        println!("当前主循环:{}", i);
        // thread::sleep 强制线程时间内暂停
        thread::sleep(Duration::from_millis(1));
    }

    // 调用句柄的 join 方法来确保在主程序结束前等待线程完成,join 会阻塞当前主线程
    // 阻塞（Blocking） 线程意味着阻止该线程执行工作或退出。
    handle.join().unwrap();

    let v = vec![1, 2, 3];
    // 因为线程执行是平行的,无法确定执行时长,所以线程中无法借用变量
    let handle = thread::spawn(move || {
        println!("在线程中打印出v:{:?}", v);
    });

    // drop(v); 借用规则会被检测,当前drop逻辑无法通过编译

    handle.join().unwrap();
}
