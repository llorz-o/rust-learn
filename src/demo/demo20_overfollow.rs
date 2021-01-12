pub fn run() {
    use std::cell::RefCell;
    use std::rc::Rc;

    use List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("引用计数 a: {}", Rc::strong_count(&a));
    println!("a 的内容: {:?}", a);

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("引用计数 a: {}", Rc::strong_count(&a));
    println!("引用计数 b: {}", Rc::strong_count(&b));
    println!("b 的内容: {:?}", b);

    /*     这段代码将产生一个引用循环
    if let Some(link) = &a.tail() {
        *link.borrow_mut() = Rc::clone(&b)
    }

    println!("引用计数 a: {}", Rc::strong_count(&a));
    println!("引用计数 b: {}", Rc::strong_count(&b));
    println!("a 的内容: {:?}", a);
    println!("b 的内容: {:?}", b); */

    /*
        如何避免引用循环:
            1. 应该使用自动化测试、代码评审和其他软件开发最佳实践来使其最小化。
            2. 重新组织数据结构，使得一部分引用拥有所有权而另一部分没有。
               换句话说，循环将由一些拥有所有权的关系和一些无所有权的关系组成，只有所有权关系才能影响值是否可以被丢弃。

        Week<T>

        使用Week弱引用
        &<Rc> 引用类型Rc实例
        Rc::downgrade(&<Rc>) -> Weak<T>
        Weak<T> 调用 weak_count 记录存在的weak reference(弱引用)
        weak_count 无需引用计数归零就可被清理
        /// 任何弱引用的循环会在其相关的强引用计数为 0 时被打断。
        /// 所以在使用 Weak 时需要调用 upgrade 方法来取一个 Option<Rc<T>> 的结果 Some(Rc<T>) Or None
    */

    use std::rc::Weak;

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
        // 父节点拥有子节点,如果父节点被丢弃了，其子节点也应该被丢弃。
        // 然而子节点不应该拥有其父节点：如果丢弃子节点，其父节点应该依然存在。这正是弱引用的例子！
        parent: RefCell<Weak<Node>>,
    }

    let leaf = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // 在添加父节点后,打印出的值为 parent: RefCell { value: (Weak), } 表明这段代码并没有造成引用循环
        println!("子节点:{:#?}", leaf);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    /*
        leaf -> branch 父节点拥有子节点leaf , leaf 拥有自己,这将产生2个引用,
        但这不是双向的,因为leaf 无法引用 branch,

        增加指向父节点的引用 parent
    */
}
