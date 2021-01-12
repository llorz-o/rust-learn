use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");
        sender.send(val).unwrap();
        // val 值在消耗过后将不允许再次使用
        // send 函数获取其参数的所有权并移动这个值归接收者所有。
        // 这可以防止在发送后再次意外地使用这个值；所有权系统检查一切是否合乎规则。
    });

    let received = receiver.recv().unwrap(); // recv 这个方法会阻塞主线程执行直到从通道中接收一个值。
                                             // try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。
    println!("Got {}", received);

    let (sender, receiver) = mpsc::channel();

    let sender2 = mpsc::Sender::clone(&sender);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            String::from("-_-!"),
        ];

        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi2"),
            String::from("from2"),
            String::from("the2"),
            String::from("thread2"),
            String::from("-_-!2"),
        ];

        for val in vals {
            sender2.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in receiver {
        println!("Got: {}", received);
    }
}
