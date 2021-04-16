use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::io::{self, Read, Write};
use about_net::{udp_t, mio_t, pnet_demo};

fn main() {

    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();

    let val = buffer.as_str().trim_end().parse::<i32>().unwrap();

    match val {
        1 => {
            udp_t::create_udp_server()
        }
        2 => {
            udp_t::create_udp_client();
        }
        3 => {
            mio_t::start();
        }
        4 => {
            match mio_t::t2() {
                Ok(_) => {
                    println!("Ok");
                },
                Err(e) => {
                    eprintln!("{}",e.as_ref());
                }
            }
        }
        5 => {
            pnet_demo::show_networks();
        }
        _ => {
            println!("please type success number!")
        }
    }
}

