use std::env;
use std::process;
#[allow(unused_variables,unused_imports)]
use command::{run, Config};

mod io2;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|_err| {
    //     eprintln!("问题解析参数:{}", _err);
    //     process::exit(1);
    // });

    // if let Err(e) = run(config) {
    //     eprintln!("应用程序错误:{}", e);
    //     process::exit(1);
    // };

    let _config = io2::Config::new(env::args()).unwrap_or_else(|_err| {
        eprintln!("参数解析错误:{}",_err);
        process::exit(1);
    });

    io2::run(_config).unwrap_or_else(|_err| {
        eprintln!("搜索失败:{}",_err);
        process::exit(1);
    })
}
