use add_two;

mod common;

#[test]
fn it_work() {
    assert_eq!(9, add_two() + common::utils::to())
}
