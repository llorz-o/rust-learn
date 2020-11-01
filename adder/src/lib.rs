pub fn demo() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // 相等，不等断言
        assert_eq!(2 + 2, 4);
        assert_ne!(2, 3);
    }

    #[test]
    fn it_eq() {
        // 真假断言
        assert!(false, "1 等于 `{}`", 2);
    }

    #[test]
    #[should_panic]
    fn panic() {
        panic!();
    }
}
