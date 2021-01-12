use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // with_capacity，它与 Vec::new 做了同样的工作，不过有一个重要的区别：它为 vector 预先分配空间。
        // 预先进行分配比仅仅 Vec::new 要稍微有效率一些
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        //
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // for worker in &mut self.workers {
        //     println!("Shutting down worker {}", worker.id);
        //     if let Some(thread) = worker.join_handle.take() {
        //         // 我们需要使用join函数等待所有线程结束,but join 函数需要实例拥有所有权
        //         // 为了解决这个问题，需要一个方法将 thread 移动出拥有其所有权的 Worker 实例以便 join 可以消费这个线程。
        //         // 如果 Worker 存放的是 Option<thread::JoinHandle<()>，就可以在 Option 上调用 take 方法将值从 Some 成员中移动出来而对 None 成员不做处理。

        //         thread.join().unwrap();

        //         // 用 join 并不会关闭线程，因为他们一直 loop 来寻找任务。
        //         // 如果采用这个实现来尝试丢弃 ThreadPool ，则主线程会永远阻塞在等待第一个线程结束上。
        //         // 所以需要一个通知线程退出的消息
        //     }
        // }

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.join_handle.take() {
                thread.join().unwrap();
            }
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    join_handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // 这里，首先在 receiver 上调用了 lock 来获取互斥器，接着 unwrap 在出现任何错误时 panic。
            // 如果互斥器处于一种叫做 被污染（poisoned）的状态时获取锁可能会失败，这可能发生于其他线程在持有锁时 panic 了且没有释放锁。
            // 如果锁定了互斥器，接着调用 recv 从通道中接收 Job。
            // 调用 recv 会阻塞当前线程，所以如果还没有任务，其会等待直到有可用的任务。
            // Mutex<T> 确保一次只有一个 Worker 线程尝试请求任务。
            let message = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });
        Worker {
            id,
            join_handle: Some(thread),
        }
    }
}

/*
    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || {
                while let Ok(job) = receiver.lock().unwrap().recv() {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
            });

            Worker {
                id,
                thread,
            }
        }
    }

    这段代码可以编译和运行，但是并不会产生所期望的线程行为：一个慢请求仍然会导致其他请求等待执行。
    其原因有些微妙：Mutex 结构体没有公有 unlock 方法， 因为锁的所有权依赖 lock 方法返回的 LockResult<MutexGuard<T>> 中 MutexGuard<T> 的生命周期。
    这允许借用检查器在编译时确保绝不会在没有持有锁的情况下访问由 Mutex 守护的资源， 不过如果没有认真的思考 MutexGuard<T> 的生命周期的话，也可能会导致比预期更久的持有锁。
    因为 while 表达式中的值在整个块一直处于作用域中，job() 调用的过程中其仍然持有锁，这意味着其他 worker 不能接收任务。

    相反通过使用 loop 并在循环块之内而不是之外获取锁和任务，lock 方法返回的 MutexGuard 在 let job 语句结束之后立刻就被丢弃了。
    这确保了 recv 调用过程中持有锁，而在 job() 调用前锁就被释放了，这就允许并发处理多个请求了。


*/
