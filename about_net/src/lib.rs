pub mod udp_t {
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
            let (re, _) = socket.recv_from(&mut buffer).expect("read udp stream failed!");

            let buffer = &mut buffer[..re];

            println!("client response:{}", str::from_utf8(buffer).unwrap());
        }
    }
}

/**
 Mio 是一个非阻塞的事件循环库：：eventloop
 可用于 io 密集型场景
*/
pub mod mio_t {
    use mio::net::{TcpListener, TcpStream};
    use mio::{Events, Interest, Poll, Token, Registry};
    use std::net::{SocketAddr};
    use std::error::Error;
    use std::io::{Write, Read, self, ErrorKind};
    use std::collections::HashMap;
    use std::thread;
    use mio::event::Event;
    use std::str::from_utf8;

    pub fn start() -> io::Result<()> {
        // 使用 Token 用以区分不同的 Socket 连接
        const SERVER: Token = Token(0);

        const DATA: &[u8] = b"Hello world!\n";

        // 创建 Poll 实例
        let mut poll = Poll::new()?;
        // 创建一个长度为 128 的空事件列表
        let mut events = Events::with_capacity(128);

        // 创建 TcpListener 实例 server，监听 9000 端口
        let addr = "127.0.0.1:7890".parse().unwrap();
        let mut server = TcpListener::bind(addr)?;

        // 将 server 注册到 poll 对象中，监听可读事件
        poll.registry()
            .register(&mut server, SERVER, Interest::READABLE)?;

        // 存储 <Token, TcpStream> 映射
        let mut connections = HashMap::new();
        // 自增的唯一 Token
        let mut unique_token = Token(SERVER.0 + 1);

        println!("You can connect to the server using `nc`:");
        println!(" $ nc 127.0.0.1 9000");
        println!("You'll see our welcome message and anything you type we'll be printed here.");

        loop {
            // 无限循环，poll 等待事件，None 表示不超时
            poll.poll(&mut events, None)?;

            // 迭代事件
            for event in events.iter() {
                // 根据事件对应的 token 区分事件并做相应处理
                match event.token() {
                    // 如果是 SERVER，则说明是 CLIENT 请求连接
                    SERVER => loop {
                        // accept 获取连接 TcpStream 对象及其地址
                        let (mut connection, address) = match server.accept() {
                            Ok((connection, address)) => (connection, address),
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                // `WouldBlock` 表示误报，实际上并没有连接
                                break;
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        };

                        println!("Accepted connection from: {}", address);

                        // 使用唯一的 token 注册
                        let token = next(&mut unique_token);
                        poll.registry().register(
                            &mut connection,
                            token,
                            Interest::READABLE.add(Interest::WRITABLE),
                        )?;

                        connections.insert(token, connection);
                    },
                    // 如果是其他连接的事件
                    token => {
                        // 从映射中获取对应的连接 connection
                        let done = if let Some(connection) = connections.get_mut(&token) {
                            // 调用函数处理读写
                            handle_connection_event(poll.registry(), connection, event)?
                        } else {
                            false
                        };
                        if done {
                            // 及时删除无效连接
                            connections.remove(&token);
                        }
                    }
                }
            }
        }


        fn next(current: &mut Token) -> Token {
            let next = current.0;
            current.0 += 1;
            Token(next)
        }

        /// 如果连接结束返回 true
        fn handle_connection_event(
            registry: &Registry,
            connection: &mut TcpStream,
            event: &Event,
        ) -> io::Result<bool> {
            if event.is_writable() {
                // 如果是可写事件，则写入 DATA
                match connection.write(DATA) {
                    // 这里期望一次写完，如果没有写完会返回错误
                    Ok(n) if n < DATA.len() => return Err(io::ErrorKind::WriteZero.into()),
                    Ok(_) => {
                        // 完整写完后，重新注册该连接，只关注可读事件
                        registry.reregister(connection, event.token(), Interest::READABLE)?
                    }
                    // WouldBlock 表示仍没有准备好，直接跳过
                    Err(ref err) if would_block(err) => {}
                    // 中断则直接递归再重试一次
                    Err(ref err) if interrupted(err) => {
                        return handle_connection_event(registry, connection, event);
                    }
                    // 其他错误直接返回
                    Err(err) => return Err(err),
                }
            }

            if event.is_readable() {
                // 如果是可读事件
                let mut connection_closed = false;
                let mut received_data = vec![0; 4096];
                let mut bytes_read = 0;
                // 循环读取
                loop {
                    match connection.read(&mut received_data[bytes_read..]) {
                        Ok(0) => {
                            // 返回 0 表示连接已关闭
                            connection_closed = true;
                            break;
                        }
                        Ok(n) => {
                            // 正常读取到 n 字节
                            bytes_read += n;
                            if bytes_read == received_data.len() {
                                received_data.resize(received_data.len() + 1024, 0);
                            }
                        }
                        // 错误处理与上方一致
                        Err(ref err) if would_block(err) => break,
                        Err(ref err) if interrupted(err) => continue,
                        Err(err) => return Err(err),
                    }
                }

                // 打印读取到的字节流
                if bytes_read != 0 {
                    let received_data = &received_data[..bytes_read];
                    if let Ok(str_buf) = from_utf8(received_data) {
                        println!("Received data: {}", str_buf.trim_end());
                    } else {
                        println!("Received (none UTF-8) data: {:?}", received_data);
                    };
                }

                // 如果连接关闭，返回 true
                if connection_closed {
                    println!("Connection closed");
                    return Ok(true);
                }
            }

            Ok(false)
        }

        fn would_block(err: &io::Error) -> bool {
            err.kind() == io::ErrorKind::WouldBlock
        }

        fn interrupted(err: &io::Error) -> bool {
            err.kind() == io::ErrorKind::Interrupted
        }
    }

    pub fn t2() -> Result<(), Box<io::Error>> {
        const MAX_SOCKETS: usize = 32;
        const LISTENER: Token = Token(1024);

        // 存储套接字句柄
        let mut sockets = HashMap::new();
        let mut events = Events::with_capacity(1024);
        let mut next_socket_index = 0;
        let mut poll = Poll::new().unwrap();
        let mut listener = TcpListener::bind("127.0.0.1:7890".parse().expect("prase addrs failed")).expect("bind addrs failed");

        poll.registry().register(&mut listener, LISTENER, Interest::READABLE).expect("register listener failed");

        loop {
            poll.poll(&mut events, None).expect("poll err");

            for event in &events {
                match event.token() {
                    LISTENER => {
                        loop {
                            match listener.accept() {
                                Ok((mut socket, _)) => {
                                    // 当前套接字的token
                                    let token = Token(next_socket_index);
                                    next_socket_index += 1;

                                    // 为套接字注册事件
                                    poll.registry().register(&mut socket, token, Interest::READABLE | Interest::WRITABLE).expect("register failed");
                                    // 保存套接字句柄
                                    sockets.insert(token, socket);
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    break;
                                }
                                Err(e) => {
                                    return Err(Box::new(e));
                                }
                            }
                        }
                    }
                    token => {

                        // 获取当前token的套接字句柄
                        let socket = sockets.get_mut(&token).unwrap();
                        let mut is_closed = false;

                        if event.is_readable() {
                            let mut buf = vec![0; 4096];
                            let mut read_index = 0;

                            loop {
                                match socket.read(&mut buf[read_index..]) {
                                    Ok(0) => {
                                        is_closed = true;
                                        break;
                                    }
                                    Ok(n) => {
                                        read_index += n;

                                        if read_index == buf.len() {
                                            buf.resize(read_index + 1024, 0)
                                        }
                                    }
                                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                        break;
                                    }
                                    Err(e) => {
                                        return Err(Box::new(e));
                                    }
                                }
                            }

                            if read_index > 0 {
                                let data = &buf[..read_index];

                                loop {
                                    match socket.write(format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", String::from_utf8_lossy(data)).as_bytes()) {
                                        Ok(n) if n < data.len() => {}
                                        Ok(_) => {
                                            is_closed = true;
                                        }
                                        Err(e) => return Err(Box::new(e)),
                                    }
                                }
                            } else {
                                sockets.remove(&token);
                            }
                        }

                        if is_closed {
                            sockets.remove(&token);
                        }
                    }
                }
            }
        }
    }
}

/**
 一个底层网络包，
 数据层，网络层，传输层

*/
pub mod pnet_demo {
    use pnet::packet::ethernet::{EthernetPacket, EtherTypes, self};
    use pnet::packet::ipv4::{Ipv4Packet};
    use pnet::packet::tcp::{TcpPacket};
    use pnet::packet::ip::{IpNextHeaderProtocols};
    use pnet::datalink::*;
    use pnet::datalink::Channel::Ethernet;
    use std::io::Error;

    pub fn show_networks() {
        let all_interfaces = interfaces();

        let default_interface = all_interfaces
            .iter()
            .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

        for interface in default_interface {
            println!("interface: {}", interface);
        }

        if let Some(interface) = default_interface {
            let (tx, mut rx) = match channel(&interface, Default::default()) {
                Ok(Ethernet(tx, rx)) => (tx, rx),
                Ok(_) => panic!("Other"),
                Err(e) => panic!("error: {}", e)
            };

            loop {
                match rx.next() {
                    Ok(packet) => {
                        if let Some(ether) = EthernetPacket::new(packet) {
                            handle_packet(ether, packet);
                        }
                    }
                    Err(e) => {
                        println!("Some error: {}", e);
                    }
                }
            }
        }

        fn handle_packet(ether: EthernetPacket, packet: &[u8]) {
            match ether.get_ethertype() {
                EtherTypes::Ipv4 => {
                    if let Some(ipv4_packet) = Ipv4Packet::new(packet) {
                        match ipv4_packet.get_next_level_protocol() {
                            IpNextHeaderProtocols::Tcp => {
                                if let Some(tcp_packet) = TcpPacket::new(packet) {
                                    println!("Tcp packet {} : {} to {} : {}",
                                             ipv4_packet.get_source(),
                                             tcp_packet.get_source(),
                                             ipv4_packet.get_destination(),
                                             tcp_packet.get_destination()
                                    );
                                }
                            }
                            protocol => println!("ignoring ip next level protocol: {}",protocol)
                        }
                    }
                }
                proto => println!("ignoring ethernet type:{}",proto)
            }
        }
    }
}