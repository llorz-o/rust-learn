use std::fs::{read_to_string, File};
use std::io::{self, ErrorKind};

pub fn run() {
    let f = File::open("./index.txt");

    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("./index.txt") {
                Ok(fc) => fc,
                Err(_) => panic!("文件创建失败"),
            },
            _ => panic!("文件打开失败!"),
        },
    };

    // unwrap 在一个返回 Result 的枚举上调用,当结果为 Ok 时返回结果值,当返回 Err 时直接 panic!
    // let f = File::open("./index.md").unwrap();
    // let f = File::open("./index.md").expect("输出错误信息");

    match read_username_from_file(String::from("./index.txt")) {
        Ok(str) => println!("{}", str),
        _ => (),
    }

    // panic! 将导致程序直接退出
}

fn read_username_from_file(path: String) -> Result<String, io::Error> {
    match read_to_string(path) {
        Ok(file) => Ok(file.parse().unwrap()),
        Err(err) => return Err(err),
    }
}
