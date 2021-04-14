// Updated example from http://rosettacode.org/wiki/Hello_world/Web_server#Rust
// to work with Rust 1.0 beta

use hello::ThreadPool;
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::vec::Vec;

#[derive(Debug)]
enum Methods {
    GET,
    POST,
    NONE,
}

#[derive(Debug)]
struct Request {
    http_version: String,
    method: Methods,
    path: String,
}

struct Response {
    schema: String,
    state_code: i32,
    status: String,
    header: Vec<String>,
}

#[derive(Debug)]
struct Router {
    request: Option<Request>,
    // response: Response,
}

impl Router {
    fn parse(mut stream: &TcpStream) -> Option<Request> {
        let mut buf = [0u8; 4096];
        match stream.read(&mut buf) {
            Ok(_) => {
                let req_str = String::from_utf8_lossy(&buf);
                let request: Vec<&str> = req_str.split("\r\n\r\n").collect();
                let request_header = &request[0];
                let request_body = &request[1];
                let hyper_texts: Vec<&str> = request_header.split("\r\n").collect();
                let schema: &str = &hyper_texts[0];
                let schemas: Vec<&str> = schema.split(" ").collect();

                let method: &str = &schemas[0];
                let path: &str = &schemas[1];
                let http_version: &str = &schemas[2];

                let method = if method == "GET" {
                    Methods::GET
                } else if method == "POST" {
                    Methods::POST
                } else {
                    Methods::NONE
                };

                Some(Request {
                    http_version: http_version.to_string(),
                    method: method,
                    path: path.to_string(),
                })
            }
            Err(e) => {
                println!("读取请求流失败:{}", e);
                None
            }
        }
    }

    fn new(mut stream: &TcpStream) -> Router {
        Router {
            request: Self::parse(stream),
        }
    }
}

fn handle_write(mut stream: TcpStream, path: String) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let read_html = fs::read_to_string(path).unwrap();
    let read_html_str = read_html.as_str();
    let response = format!("{}{}", response, read_html_str);

    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    let router = Router::new(&stream);
    match router.request {
        Some(request) => {
            match (request.method) {
                Methods::GET => {
                    if request.path == "/" {
                        handle_write(stream, String::from("index.html"));
                    } else if request.path == "/sleep" {
                        thread::sleep(Duration::from_secs(5));
                        handle_write(stream, String::from("sleep.html"));
                    } else {
                        handle_write(stream, String::from("404.html"));
                    }
                }
                _ => {
                    handle_write(stream, String::from("404.html"));
                }
            };
        }
        _ => {
            // 请求解析失败
            println!("请求解析失败")
        }
    };
}

/*

使用线程池改善吞吐量

线程池（thread pool）是一组预先分配的等待或准备处理任务的线程。
当程序收到一个新任务，线程池中的一个线程会被分配任务，这个线程会离开并处理任务。
其余的线程则可用于处理在第一个线程处理任务的同时处理其他接收到的任务。
当第一个线程处理完任务时，它会返回空闲线程池中等待处理新任务。
线程池允许我们并发处理连接，增加 server 的吞吐量。

我们会将池中线程限制为较少的数量，以防拒绝服务（Denial of Service， DoS）攻击；
如果程序为每一个接收的请求都新建一个线程，某人向 server 发起千万级的请求时会耗尽服务器的资源并导致所有请求的处理都被终止。

不同于分配无限的线程，线程池中将有固定数量的等待线程。当新进请求时，将请求发送到线程池中做处理。
线程池会维护一个接收请求的队列。每一个线程会从队列中取出一个请求，处理请求，接着向对队列索取另一个请求。
通过这种设计，则可以并发处理 N 个请求，其中 N 为线程数。
如果每一个线程都在响应慢请求，之后的请求仍然会阻塞队列，不过相比之前增加了能处理的慢请求的数量。

这个设计仅仅是多种改善 web server 吞吐量的方法之一。
其他可供探索的方法有 fork/join 模型和单线程异步 I/O 模型。
如果你对这个主题感兴趣，则可以阅读更多关于其他解决方案的内容并尝试用 Rust 实现他们；
对于一个像 Rust 这样的底层语言，所有这些方法都是可能的。

在开始之前，让我们讨论一下线程池应用看起来怎样。
当尝试设计代码时，首先编写客户端接口确实有助于指导代码设计。
以期望的调用方式来构建 API 代码的结构，接着在这个结构之内实现功能，而不是先实现功能再设计公有 API。

类似于第十二章项目中使用的测试驱动开发。这里将要使用编译器驱动开发（compiler-driven development）。
我们将编写调用所期望的函数的代码，接着观察编译器错误告诉我们接下来需要修改什么使得代码可以工作。

*/

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listening for connections on port {}", 7878);

    let thread_pool = ThreadPool::new(100);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => thread_pool.execute(|| {
                handle_client(stream);
            }),
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
