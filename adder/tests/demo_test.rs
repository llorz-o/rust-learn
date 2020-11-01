extern crate adder;

#[test]
fn it_eq_really() {
    adder::demo();
    assert_eq!(8, 9)
}
