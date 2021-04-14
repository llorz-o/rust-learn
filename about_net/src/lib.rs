pub mod udpT {
    use std::net::UdpSocket;
    use std::{io, str};

    pub fn create_udp_server() {
        let mut socket = UdpSocket::bind("127.0.0.1:34567").expect("create udp server failed");

        loop {
            let mut buf = [0; 10];
            let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");

            let buf = &mut buf[..number_of_bytes];

            // buf.reverse();

            let str = str::from_utf8(buf).unwrap_or_else(|e| {
                eprintln!("utf8 buf to str failed:{}", e);
                "??"
            });

            let pt_str: String = str.chars().rev().collect();

            println!("server received data and reverse data:{:?}", pt_str);

            socket.send_to(pt_str.as_bytes(), src_addr).expect("udp send failed!");
        }
    }

    pub fn create_udp_client() {
        let socket = UdpSocket::bind("127.0.0.1:34568").expect("create udp client failed");
        socket.connect("127.0.0.1:34567").expect("udp connect failed!");

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            socket.send(input.as_bytes()).expect("udp send failed!");

            let mut buffer = [0; 10];
            let (re, addr) = socket.recv_from(&mut buffer).expect("read udp stream failed!");

            let buffer = &mut buffer[..re];

            println!("client response:{}", str::from_utf8(buffer).unwrap());
        }
    }
}

/**
 Mio 是一个非阻塞的事件循环库：：eventloop
 可用于 io 密集型场景
*/
pub mod MioT {
    use mio::net::{TcpListener, TcpStream};
    use mio::{Events, Interest, Poll, Token};
    use std::net::{SocketAddr};
    use std::error::Error;
    use std::io::{Write, Read, self};
    use std::collections::HashMap;
    use std::thread;

    // 一些标记使我们能够确定哪个事件是针对哪个套接字的。
    const SERVER: Token = Token(0);
    const CLIENT: Token = Token(1);

    pub fn create_addr_poll() -> (SocketAddr, Poll, Events) {
        let addr = "127.0.0.1:8080".parse().expect("parse address failed!");
        // 为事件创建一个储存
        let events = Events::with_capacity(128);
        // 创建一个 poll 实例
        let poll = Poll::new().expect("new poll failed!");
        (addr, poll, events)
    }

    pub fn start() -> Result<(), Box<dyn Error>> {
        let (addr, mut poll, mut events) = create_addr_poll();

        // 设置服务器套接字
        let mut server = TcpListener::bind(addr).expect("bind address failed!");
        // 开始监听传入的连接
        poll.registry().register(&mut server, SERVER, Interest::READABLE).expect("register server failed!");

        // 设置客户端套接字
        let mut client = TcpStream::connect(addr).expect("bind address failed!");
        // 注册套接字
        poll.registry().register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE).expect("register client failed!");

        fn would_block(err: &io::Error) -> bool {
            err.kind() == io::ErrorKind::WouldBlock
        }


        // 开始事件循环
        loop {
            // 轮询Mio是否有事件，直到我们收到事件为止一直阻塞
            poll.poll(&mut events, None).unwrap();

            // 处理每个事件
            for event in events.iter() {
                // 我们可以使用之前提供的令牌来“注册”到
                // 确定事件在哪个套接字上。
                match event.token() {
                    SERVER => {
                        //如果这是服务器的事件，则表示连接
                        //已准备好被接受。
                        //
                        //接受连接并立即将其删除。 这会
                        //关闭套接字，并将EOF通知客户端。
                        // let connection = server.expect();
                        // drop(connection);
                        println!("server recv a connection!");
                        match server.accept() {
                            Ok((connection, addr)) => {
                                println!("got a connection from: {},peer addr:{:?}", addr, connection.peer_addr());
                            }
                            Err(ref err) if would_block(err) => break,
                            Err(err) => return Err(Box::new(err))
                        }
                    }
                    CLIENT => {
                        if event.is_writable() {
                            // 我们可以（可能）写入套接字而不会阻塞。
                            println!("the connect is writable!");
                            // let response = "HTTP/1.1 200 OK\r\n\r\n";
                            // let response = format!("{}{}", response, "body main");
                            //
                            // match client.write(response.as_bytes()) {
                            //     Ok(_) => println!("Response sent"),
                            //     Err(e) => println!("Failed sending response: {}", e),
                            // }
                            //
                            // client.shutdown(Shutdown::Both);
                        }

                        if event.is_readable() {
                            let mut read = [];
                            client.read(&mut read).unwrap();
                            println!("the connection read: {:?}", read);
                        }

                        //由于服务器只是关闭了连接，所以让我们
                        //退出事件循环。
                        // println!("ok");
                        // return Ok(());
                    }
                    _ => {
                        // 除了我们提供的令牌外，我们不希望发生任何其他带有令牌的事件。
                        println!("err");
                        unreachable!()
                    }
                }
            }
        }
    }

    pub fn t2() -> Result<(), Box<dyn Error>> {
        const MAX_SOCKETS: usize = 32;
        const LISTENER: Token = Token(1024);

        // 存储套接字句柄
        let mut sockets = HashMap::new();
        // 存储请求上下文
        let mut handles: HashMap<Token, Context> = HashMap::new();

        struct Context {
            write_done: bool,
            read_done: bool,
            writable: bool,
            readable: bool,
            response_body: String,
            request_query: Vec<u8>,
        }

        let mut next_socket_index = 0;
        let mut poll = Poll::new().unwrap();

        let mut listener = TcpListener::bind("127.0.0.1:7890".parse().expect("prase addrs failed")).expect("bind addrs failed");

        poll.registry().register(&mut listener, LISTENER, Interest::READABLE).expect("register listener failed");

        let mut events = Events::with_capacity(1024);

        let mut buf = [0; 256];

        let mut write_count = 0;

        loop {
            poll.poll(&mut events, None).expect("poll err");

            for event in &events {
                match event.token() {
                    LISTENER => {
                        loop {
                            match listener.accept() {
                                Ok((mut socket, _)) => {
                                    println!("{}", next_socket_index);
                                    // 当前请求上限
                                    // if next_socket_index >= MAX_SOCKETS {
                                    //     return Ok(());
                                    // }

                                    // 当前套接字的token
                                    let tk = Token(next_socket_index);
                                    next_socket_index += 1;

                                    // 为套接字注册事件
                                    poll.registry().register(&mut socket, tk, Interest::READABLE | Interest::WRITABLE).expect("register failed");
                                    // 保存套接字句柄
                                    sockets.insert(tk, socket);
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    break;
                                }
                                e => panic!("err = {:?}", e)
                            }
                        }
                    }
                    token => {
                        println!("new token event");

                        // 获取当前token的套接字句柄
                        let socket = sockets.get_mut(&token).unwrap();

                        // 获取/创建当前token的上下文
                        let ctx = handles.entry(token).or_insert(Context {
                            write_done: false,
                            read_done: false,
                            writable: false,
                            readable: false,
                            response_body: "".to_string(),
                            request_query: vec![],
                        });

                        if event.is_readable() {
                            println!("readable");
                            ctx.readable = true;
                        }

                        if event.is_writable() {
                            println!("writable");
                            ctx.writable = true;
                        }

                        loop {
                            if ctx.readable {
                                match socket.read(&mut buf) {
                                    Ok(0) => {
                                        // 这里表示什么也没读到
                                        // 当buf大小为0时，或是可读流中没有任何数据将会返回 OK(0)
                                        println!("read ok 0");
                                        break;
                                    }
                                    Ok(x) => {
                                        println!("reading len:{}", x);

                                        // 每次读到数据将写入当前套接字到上下文中
                                        ctx.request_query.append(&mut (&buf[..x]).to_vec());

                                        // 清除缓冲区
                                        buf.fill(0);

                                        // 当前读完毕
                                        if x < buf.len() { ctx.read_done = true }
                                    }
                                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                        println!("read done");
                                        ctx.read_done = true;
                                    }
                                    e => panic!("err={:?}", e),
                                }
                            } else {
                                break;
                            }

                            if ctx.writable {
                                if ctx.read_done {
                                    // 去除空白填充
                                    ctx.response_body = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", String::from_utf8_lossy(&ctx.request_query));
                                    match socket.write(ctx.response_body.as_bytes()) {
                                        Ok(0) => {
                                            println!("write ok 0");
                                            break;
                                        }
                                        Ok(x) if x == ctx.response_body.len() => {
                                            println!("write done");
                                            // 当前内容已写完
                                            ctx.write_done = false;
                                            // 断开连接
                                            handles.remove(&token);
                                            sockets.remove(&token);
                                            break;
                                        }
                                        Ok(x) => println!("Response sent:{}", x),
                                        Err(e) => println!("Failed sending response: {}", e),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}