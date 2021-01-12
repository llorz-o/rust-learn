pub fn run() {
    println!("**栈结构**");
    // 栈结构
    {
        use std::option::Option::{None, Some};

        #[derive(Debug)]
        struct Stack<T> {
            top: Option<Box<StackNode<T>>>,
        }

        #[derive(Clone, Debug)]
        struct StackNode<T> {
            val: T,
            next: Option<Box<StackNode<T>>>,
        }

        impl<T> Stack<T> {
            pub fn new() -> Stack<T> {
                Stack { top: None }
            }
            pub fn push(&mut self, val: T) {
                let top = self.top.take();
                let stack_node = StackNode {
                    val: val,
                    next: top,
                };
                self.top = Some(Box::new(stack_node));
            }
            pub fn pop(&mut self) -> Option<T> {
                let val = self.top.take();
                match val {
                    None => None,
                    Some(mut x) => {
                        self.top = x.next.take();
                        Some(x.val)
                    }
                }
            }
        }

        let mut a = Stack::<i32>::new();

        a.push(1);
        a.push(2);
        a.push(3);

        println!("{:?}", a);

        println!("{:?}", a.pop());
        println!("{:?}", a.pop());
        println!("{:?}", a.pop());
    }
    println!("**队列结构**");
    // 先进先出,队列结构
    {
        #[derive(Debug)]
        struct Queue<T> {
            qdata: Vec<T>,
        }

        impl<T> Queue<T> {
            pub fn new() -> Queue<T> {
                Queue { qdata: vec![] }
            }
            pub fn push(&mut self, val: T) {
                self.qdata.push(val);
            }
            pub fn pop(&mut self) {
                if let Some(_) = self.qdata.get_mut(0) {
                    self.qdata.remove(0);
                };
            }
        }

        let mut q = Queue::<i32>::new();
        q.push(1);
        q.push(2);
        q.push(3);

        println!("Queue:{:?}", q);

        q.pop();
        q.pop();
        q.pop();
        q.pop();

        println!("pop after Queue:{:?}", q);
    }
    println!("**二叉树**");
    // 二叉树
    {
        type TreeNode<K, V> = Option<Box<Node<K, V>>>;

        #[derive(Debug)]
        struct Node<K, V: std::fmt::Display> {
            left: TreeNode<K, V>,
            right: TreeNode<K, V>,
            key: K,
            value: V,
        }

        impl<K, V: std::fmt::Display> Node<K, V> {
            fn new(key: K, value: V) -> Self {
                Node {
                    left: None,
                    right: None,
                    value,
                    key,
                }
            }
        }

        trait BinaryTree<K, V> {
            fn pre_order(&self);
            fn in_order(&self);
            fn pos_order(&self);
        }
        trait BinarySearchTree<K: PartialOrd, V>: BinaryTree<K, V> {
            fn insert(&mut self, k: K, v: V);
        }

        impl<K, V: std::fmt::Display> BinaryTree<K, V> for Node<K, V> {
            fn pre_order(&self) {
                println!("value:{}", self.value);

                if let Some(ref left) = self.left {
                    left.pre_order();
                }
                if let Some(ref right) = self.right {
                    right.pre_order();
                }
            }
            fn in_order(&self) {
                if let Some(ref left) = self.left {
                    left.in_order();
                }
                println!("{}", self.value);
                if let Some(ref right) = self.right {
                    right.in_order();
                }
            }
            fn pos_order(&self) {
                if let Some(ref left) = self.left {
                    left.pos_order();
                }
                if let Some(ref right) = self.right {
                    right.pos_order();
                }
                println!("{}", self.value);
            }
        }

        impl<K: PartialOrd, V: std::fmt::Display> BinarySearchTree<K, V> for Node<K, V> {
            fn insert(&mut self, key: K, value: V) {
                if self.key < key {
                    if let Some(ref mut right) = self.right {
                        right.insert(key, value);
                    } else {
                        self.right = Some(Box::new(Node::new(key, value)));
                    }
                } else {
                    if let Some(ref mut left) = self.left {
                        left.insert(key, value);
                    } else {
                        self.left = Some(Box::new(Node::new(key, value)));
                    }
                }
            }
        }

        let mut n = Node::new('a', "a");
        n.insert('b', "b");
        n.insert('c', "c");
        n.insert('d', "d");
        n.insert('e', "e");
        n.insert('f', "f");

        n.in_order();
        n.pre_order();
        n.pos_order();
    }
    println!("**优先队列**");
    // 优先队列
    {
        #[derive(Debug)]
        struct PriorityQueue<T> {
            pq: Vec<T>,
        }

        impl<T> PriorityQueue<T>
        where
            T: PartialOrd + Clone,
        {
            fn new() -> Self {
                PriorityQueue { pq: vec![] }
            }

            fn len(&self) -> usize {
                self.pq.len()
            }

            fn is_empty(&self) -> bool {
                self.pq.len() == 0
            }

            fn min_index(&self) -> usize {
                let mut min = 0;
                let len = self.len();
                for i in 1..len {
                    if self.pq[i] < self.pq[min] {
                        min = i
                    }
                }
                min
            }

            fn max_index(&self) -> usize {
                let mut max = 0;
                let len = self.len();
                for i in 1..len {
                    if self.pq[i] > self.pq[max] {
                        max = i
                    }
                }
                max
            }

            fn delete_max(&mut self) -> Option<T> {
                if self.is_empty() {
                    return None;
                }
                let max = self.max_index();
                Some(self.pq.remove(max))
            }

            fn delete_min(&mut self) -> Option<T> {
                if self.is_empty() {
                    return None;
                }
                let min = self.min_index();
                Some(self.pq.remove(min))
            }

            fn insert(&mut self, val: T) {
                self.pq.push(val)
            }
        }

        let mut a = PriorityQueue::<i32>::new();
        a.insert(4);
        a.insert(3);
        a.insert(4);
        a.insert(1);
        println!("a:{:?}", a);
        a.delete_max();
        println!("a:{:?}", a);
        a.delete_min();
        println!("a:{:?}", a);
    }
    println!("**链表**");
    // 链表
    {
        use std::fmt::Display;

        #[derive(Debug)]
        enum List<T: Display> {
            Cons(T, Box<List<T>>),
            Nil,
        }

        impl<T: Display> List<T> {
            pub fn new() -> Self {
                List::Nil
            }
            pub fn prepend(self, val: T) -> Self {
                List::Cons(val, Box::new(self))
            }
            pub fn len(&self) -> usize {
                match *self {
                    List::Cons(_, ref next) => 1 + next.len(),
                    List::Nil => 0,
                }
            }
            pub fn stringlfy(&self) -> String {
                match *self {
                    List::Cons(ref val, ref next) => format!("{},{}", val, next.stringlfy()),
                    List::Nil => format!("{}", "Nil"),
                }
            }
        }

        let a = List::<i32>::new();
        let a = a.prepend(1);
        let a = a.prepend(2);
        let a = a.prepend(3);
        let a = a.prepend(4);
        println!("List len:{}", a.len());
        println!("List stringlfy:{}", a.stringlfy());
    }
    println!("**图**");
    // 图结构
    {
        #[derive(Debug)]
        struct Node {
            nodeid: usize,
            nodename: String,
        }

        impl Node {
            fn new(nodeid: usize, nodename: String) -> Self {
                Node { nodeid, nodename }
            }
        }

        #[derive(Debug, Clone)]
        struct Edge {
            edge: bool,
        }

        impl Edge {
            fn new() -> Self {
                Edge { edge: false }
            }
            fn have_edge() -> Edge {
                Edge { edge: true }
            }
        }

        #[derive(Debug)]
        struct Graphadj {
            nodenums: usize,
            graphadj: Vec<Vec<Edge>>,
        }

        impl Graphadj {
            fn new(nodenums: usize) -> Self {
                Graphadj {
                    nodenums,
                    graphadj: vec![vec![Edge::new(); nodenums]; nodenums],
                }
            }
            fn insert_edge(&mut self, v1: Node, v2: Node) {
                match v1.nodeid < self.nodenums && v2.nodeid < self.nodenums {
                    true => {
                        self.graphadj[v1.nodeid][v2.nodeid] = Edge::have_edge();
                        //下面这句注释去掉相当于把图当成无向图
                        // self.graphadj[v2.nodeid][v1.nodeid] = Edge::have_edge();
                    }
                    false => {
                        panic!("your nodeid is bigger than nodenums!");
                    }
                }
            }

            fn stringlfy(&self) {
                let mut header_str = format!("  ");
                for i in 0..self.nodenums {
                    header_str = format!("{} {}", header_str, i);
                }
                println!("{}", header_str);

                for (index, line) in self.graphadj.iter().enumerate() {
                    let mut line_str = format!("{} ", index);
                    for i in line.iter() {
                        let n = if i.edge { '1' } else { '-' };
                        line_str = format!("{} {}", line_str, n);
                    }
                    println!("{}", line_str);
                }
            }
        }

        let mut g = Graphadj::new(10);
        let v1 = Node::new(2, "节点1".to_string());
        let v2 = Node::new(8, "节点2".to_string());
        let v3 = Node::new(6, "节点3".to_string());
        let v4 = Node::new(7, "节点4".to_string());
        g.insert_edge(v1, v2);
        g.insert_edge(v3, v4);
        g.stringlfy();
        // 用边描述点
    }
}
