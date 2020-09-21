use rand::Rng;
use std::cmp::Ordering;
use std::io; // 引入模块
use std::io::copy; // 也可已引入方法

fn main() {
    println!("猜测这个数值");

    // 接下来在 Cargo.toml 中添加 rand 包,使用 cargo build 来安装该包
    // thread_rng 线程随机数生成器,位于当前的执行线程的本地环境,从操作系统获取seed种子
    // gen_range 方法生成一个基于当前传入值区间与类型的随机数 [1,101) 左闭右开
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    // println!("当前的随机数:{}", secret_number);

    loop {
        // rust 变量默认为不可变类型
        // 创建可变变量,使用 mut 声明可变;
        // String 类型的关联函数 new 创建了一个该类型的实例;
        // :: 是关联函数的表示方法,也被称为静态方法

        let mut guess = String::new();

        println!("请输入你的猜测值");

        // & 符号表示这个值为一个引用值 reference;mut 代表当前变量为可变类型
        // read_line 方法将返回一个Result 类型的值,该类型是一个枚举类型
        // 枚举一个 Ok,Err ,如果结果为 Ok 将直接返回,但是Err必须被处理
        // 使用 expect 将直接抛出异常,中断程序
        io::stdin().read_line(&mut guess).expect("读取行失败");

        // println!("你的猜测值:{}", guess);

        // 类型转换,可以使用当前的命名覆盖旧的命名
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // match 表达式由分支构成
        match guess.cmp(&secret_number) {
            // 一个分支包含一个 模式（pattern）和表达式开头的值与分支模式相匹配时应该执行的代码
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
        }
    }
}
