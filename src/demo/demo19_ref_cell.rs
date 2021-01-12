pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error:您已经超出配额!")
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("警告:你已经超出限额的90%!")
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("警告:你已经超出限额的75%!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // sent_messenges: Vec<String>,
        sent_messenges: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messenges:vec![],
                sent_messenges: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // &self 为不可变引用
        // 这里如果改为&mut self 将不符合 trait Messager 的定义,如果引用第三方库,势必会有这种情况
        fn send(&self, message: &str) {
            // self.sent_messenges.push(String::from(message));
            self.sent_messenges.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        // 相当于 RefCell<T> 代理了值的借用,为每一个 borrow 执行借用记录
        // 就像编译时借用规则一样，RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用。
        assert_eq!(mock_messenger.sent_messenges.borrow().len(), 1);
    }
}

pub fn run() {
    /*
        基于Rc 和 RefCell 的特征,我们可以实现 多可变引用 的特性
    */
    use std::cell::RefCell;
    use std::rc::Rc;
    use List::*;

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    let value = Rc::new(RefCell::new(0));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Rc::new(Cons(Rc::new(RefCell::new(2)), Rc::clone(&a)));
    let c = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)));

    println!("a: `{:?}`", a);
    println!("b: `{:?}`", b);
    println!("c: `{:?}`", c);

    *value.borrow_mut() += 10;

    println!("a: `{:?}`", a);
    println!("b: `{:?}`", b);
    println!("c: `{:?}`", c);

    match &(*b) {
        Cons(v, _) => println!("{:?}", v),
        _ => println!("nil"),
    };
}
