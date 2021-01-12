pub fn run() {
    /*
        有时我们希望库用户在特定情况下能够扩展有效的类型集合。
        为了展示如何实现这一点，这里将创建一个图形用户接口（Graphical User Interface， GUI）工具的例子，
        它通过遍历列表并调用每一个项目的 draw 方法来将其绘制到屏幕上 —— 此乃一个 GUI 工具的常见技术。
        我们将要创建一个叫做 gui 的库 crate，它含一个 GUI 库的结构。这个 GUI 库包含一些可供开发者使用的类型，
        比如 Button 或 TextField。在此之上，gui 的用户希望创建自定义的可以绘制于屏幕上的类型：
        比如，一个程序员可能会增加 Image，另一个可能会增加 SelectBox。

        这个例子中并不会实现一个功能完善的 GUI 库，不过会展示其中各个部分是如何结合在一起的。编写库的时候，
        我们不可能知晓并定义所有其他程序员希望创建的类型。我们所知晓的是 gui 需要记录一系列不同类型的值，
        并需要能够对其中每一个值调用 draw 方法。这里无需知道调用 draw 方法时具体会发生什么，只要该值会有那个方法可供我们调用。

        在拥有继承的语言中，可以定义一个名为 Component 的类，该类上有一个 draw 方法。
        其他的类比如 Button、Image 和 SelectBox 会从 Component 派生并因此继承 draw 方法。
        它们各自都可以覆盖 draw 方法来定义自己的行为，但是框架会把所有这些类型当作是 Component 的实例，
        并在其上调用 draw。不过 Rust 并没有继承，我们得另寻出路。
    */

    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        // 这与定义使用了带有 trait bound 的泛型类型参数的结构体不同。泛型类型参数一次只能替代一个具体类型，而 trait 对象则允许在运行时替代多种具体类型。
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    // run 方法使用泛型和 trait bound
    // 限制了 Screen 实例必须拥有一个全是 Button 类型或者全是 TextField 类型的组件列表。
    // 如果只需要同质（相同类型）集合，则倾向于使用泛型和 trait bound，因为其定义会在编译时采用具体类型进行单态化。
    //
    // pub struct Screen<T: Draw> {
    //     pub components: Vec<T>,
    // }

    // impl<T> Screen<T>
    //     where T: Draw {
    //     pub fn run(&self) {
    //         for component in self.components.iter() {
    //             component.draw();
    //         }
    //     }
    // }

    //     trait Run {}
    //     struct Human {}
    //     impl Run for Human {}
    //     struct Cat {}
    //     impl Run for Cat {}

    //     fn demo<T>(v: Vec<Box<T>>)
    //     where
    //         T: Run,
    //     {
    //     }

    //     let v = vec![];
    //     v.push(Box::new(Human {}));
    //     // v.push(Box::new(Cat {}));
    //     demo(v);

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // 绘制按钮的代码
            println!("Button context {}", self.label)
        }
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            //
            println!("SelectBox context {}", self.options[0])
        }
    }

    fn main() {
        let screen = Screen {
            // 由于components 的类型约束为 dynamic trait ,所以必须实现了Draw trait才有可能
            // 当使用 trait 对象时，Rust 必须使用动态分发。编译器无法知晓所有可能用于 trait 对象代码的类型，所以它也不知道应该调用哪个类型的哪个方法实现。
            // 为此，Rust 在运行时使用 trait 对象中的指针来知晓需要调用哪个方法。动态分发也阻止编译器有选择的内联方法代码，这会相应的禁用一些优化。
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };

        screen.run();

        /*
           只有 对象安全（object safe）的 trait 才可以组成 trait 对象。

           如果一个 trait 中所有的方法有如下属性时，则该 trait 是对象安全的:

            Trait本身是没有Self: Sized约束
            所有方法都是Object Safe（对象安全）的
                函数有Self: Sized约束
                Or--
                函数不能有泛型参数
                第一个参数必须为Self类型或者可以解引用为Self的类型（目前包含self&self&mut selfself:: Box<Self>）
                其他参数或者返回值均不能使用Self类型

            总结为两条:
            返回值类型不为 Self
            方法没有任何泛型类型参数


           Self 关键字是我们要实现 trait 或方法的类型的别名。
           对象安全对于 trait 对象是必须的，因为一旦有了 trait 对象，就不再知晓实现该 trait 的具体类型是什么了。
           如果 trait 方法返回具体的 Self 类型，但是 trait 对象忘记了其真正的类型，那么方法不可能使用已经忘却的原始具体类型。
           同理对于泛型类型参数来说，当使用 trait 时其会放入具体的类型参数：此具体类型变成了实现该 trait 的类型的一部分。
           当使用 trait 对象时其具体类型被抹去了，故无从得知放入泛型参数类型的类型是什么。

           !! Trait object的本质是指针，它可以指向不同的类型，指向的具体类型不同，调用的方法也不同
        */
    }
}
