pub fn run() {
    /*
        状态模式（state pattern）是一个面向对象设计模式。该模式的关键在于一个值有某些内部状态，
        体现为一系列的 状态对象，同时值的行为随着其内部状态而改变。
        状态对象共享功能：当然，在 Rust 中使用结构体和 trait 而不是对象和继承。
        每一个状态对象负责其自身的行为，以及该状态何时应当转移至另一个状态。
        持有一个状态对象的值对于不同状态的行为以及何时状态转移毫不知情。
    */

    // 使用状态模式意味着当程序的业务需求改变时，无需改变值持有状态或者使用值的代码。
    // 我们只需更新某个状态对象中的代码来改变其规则，或者是增加更多的状态对象。
    // 让我们看看一个有关状态模式和如何在 Rust 中使用它的例子。

    // 为了探索这个概念，我们将实现一个增量式的发布博文的工作流。这个博客的最终功能看起来像这样：

    // 博文从空白的草案开始。
    // 一旦草案完成，请求审核博文。
    // 一旦博文过审，它将被发表。
    // 只有被发表的博文的内容会被打印，这样就不会意外打印出没有被审核的博文的文本。

    pub mod Blog {
        pub struct Post {
            state: Option<Box<dyn State>>,
            content: String,
        }
        impl Post {
            pub fn new() -> Post {
                Post {
                    state: Some(Box::new(Draft {})),
                    content: String::new(),
                }
            }
            pub fn add_text(&mut self, text: &str) {
                if self.state.as_ref().unwrap().can_add_text() {
                    self.content.push_str(text);
                }
            }
            pub fn content(&self) -> &str {
                // as_ref 方法是因为需要 Option 中值的引用而不是获取其所有权。
                // 因为 state 是一个 Option<Box<State>>，调用 as_ref 会返回一个 Option<&Box<State>>。
                // 如果不调用 as_ref，将会得到一个错误，因为不能将 state 移动出借用的 &self 函数参数。
                self.state.as_ref().unwrap().content(self)
            }
            pub fn request_review(&mut self) {
                // 调用 take 方法将 state 字段中的 Some 值取出并留下一个 None,因为 Rust 不允许在结构体中存在空的字段。
                if let Some(s) = self.state.take() {
                    self.state = Some(s.request_review());
                }
            }
            pub fn approve(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.approve());
                }
            }
            pub fn reject(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.reject())
                }
            }
        }
        // 当前博文的状态的 trait
        trait State {
            // self: Box<Self> 这个语法意味着这个方法调用只对这个类型的 Box 有效。
            fn request_review(self: Box<Self>) -> Box<dyn State>;
            fn approve(self: Box<Self>) -> Box<dyn State>;
            fn reject(self: Box<Self>) -> Box<dyn State>;
            fn can_add_text(&self) -> bool {
                false
            }
            // content 方法拥有默认实现
            fn content<'a>(&self, post: &'a Post) -> &'a str {
                ""
            }
        }
        // 草案状态的state 结构体
        struct Draft {}
        impl State for Draft {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                // 实现状态的审核方法,审核将导致状态变化,maybe not change
                Box::new(PendingReview { approve_count: 0 })
            }
            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn reject(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn can_add_text(&self) -> bool {
                true
            }
        }
        // 等待审核状态的state 结构体
        struct PendingReview {
            approve_count: i32,
        }
        impl State for PendingReview {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn approve(self: Box<Self>) -> Box<dyn State> {
                let mut _self = self;
                _self.approve_count += 1;
                if _self.approve_count >= 2 {
                    Box::new(Published {})
                } else {
                    _self
                }
            }
            fn reject(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }
        // 发布状态的state 结构体
        struct Published {}
        impl State for Published {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }
            // 只有发布后的博文能够返回具体内容
            // 这里获取 post 的引用作为参数，并返回 post 一部分的引用，所以返回的引用的生命周期与 post 参数相关。
            fn content<'a>(&self, post: &'a Post) -> &'a str {
                &post.content
            }
            fn reject(self: Box<Self>) -> Box<dyn State> {
                Box::new(Draft {})
            }
        }
    }

    use Blog::Post;

    let mut post = Post::new();

    post.add_text("aaaa");

    println!("已添加字段:{}", post.content());

    post.request_review();

    println!("request_review:{}", post.content());

    post.approve();

    println!("approve:{}", post.content());

    post.approve();

    println!("approve:{}", post.content());

    pub mod Blog2 {
        pub struct Post {
            content: String,
        }

        impl Post {
            pub fn new() -> DraftPost {
                DraftPost {
                    content: String::new(),
                }
            }
            pub fn content(&self) -> &str {
                &self.content
            }
        }

        pub struct DraftPost {
            content: String,
        }
        impl DraftPost {
            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text)
            }
            pub fn request_review(self) -> PendingReviewPost {
                PendingReviewPost {
                    content: self.content,
                }
            }
        }

        pub struct PendingReviewPost {
            content: String,
        }
        impl PendingReviewPost {
            pub fn approve(self) -> PublishedPost {
                PublishedPost {
                    content: self.content,
                }
            }
        }

        pub struct PublishedPost {
            content: String,
        }
        impl PublishedPost {
            pub fn content(&self) -> &str {
                &self.content
            }
            pub fn reject(self) -> DraftPost {
                DraftPost {
                    content: self.content,
                }
            }
        }
    }

    /*

        重新赋值 post 使得这个实现不再完全遵守面向对象的状态模式：
        状态间的转换不再完全封装在 Post 实现中。
        然而，得益于类型系统和编译时类型检查，我们得到了的是无效状态是不可能的！

        在 Rust 中面向对象模式并不总是最好的解决方案，因为 Rust 拥有像所有权这样的面向对象语言所没有的功能。
    */

    // 将状态行为编码为类型

    use Blog2::Post as Post2;

    let mut post = Post2::new();

    post.add_text("I ate a saled for lunch today");

    // 不同状态下的方法调用将返回不同的实例,同时方法也与实例相对应,从编译层面上就保证了调用的安全性
    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a saled for lunch today", post.content());
}
