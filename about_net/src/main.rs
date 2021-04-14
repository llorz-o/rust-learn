use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::io::{self, Read, Write};
use about_net::{udpT, MioT};

fn main() {

    match MioT::t2() {
        Ok(_) => {
            println!("Ok");
        },
        Err(e) => {
            eprintln!("{}",e.as_ref());
        }
    }

    // let stdin = io::stdin();
    // let mut buffer = String::new();
    // stdin.read_line(&mut buffer).unwrap();
    //
    // let val = buffer.as_str().trim_end().parse::<i32>().unwrap();
    //
    // match val {
    //     1 => {
    //         udpT::create_udp_server()
    //     }
    //     2 => {
    //         udpT::create_udp_client();
    //     }
    //     3 => {
    //         MioT::start();
    //     }
    //     4 => {
    //         match MioT::t2() {
    //             Ok(_) => {
    //                 println!("Ok");
    //             },
    //             Err(e) => {
    //                 eprintln!("{}",e.as_ref());
    //             }
    //         }
    //     }
    //     _ => {
    //         println!("please type success number!")
    //     }
    // }
}

