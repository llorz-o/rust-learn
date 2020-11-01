pub fn run() -> i32 {
    9
}

fn privatefn() -> i32 {
    10
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(10, privatefn());
    }
}
