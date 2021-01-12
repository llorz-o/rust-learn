pub fn run() {
    // 宏是一种为写其他代码而写代码的方式，即所谓的 元编程（metaprogramming）
    // 元编程对于减少大量编写和维护的代码是非常有用的，它也扮演了函数扮演的角色。但宏有一些函数所没有的附加能力

    /* 使用 macro_rules! 的声明宏用于通用元编程 */

    // vec![1,2,3] 宏构建任意数量与类型的值,因为你无法使用函数实现这样的功能
    // vec! 宏定义的简化版本
    #[macro_export]
    macro_rules! vec {
        ($($x:expr),*) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x)
                )*
                temp_vec
            }
        };
    }

    /* 用于从属性生成代码的过程宏 */
    // use proc_macro::TokenStream;
    // #[some_attribute] // some_attribute 是一个使用特定宏的占位符
    // pub fn some_name(input: TokenStream) -> TokenStream {}

    /* 自定义 derive 宏 */
    pub mod hello_macro {
        pub trait HelloMacro {
            fn hello_macro();
        }
    }
    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro; // 自定义的crate位于文档下的 hello_macro_derive中

    #[derive(HelloMacro)]
    struct Pancakes;

    // impl HelloMacro for Pancakes {
    //     // 该函数用于打印出当前任意实现了 HelloMacro 的结构体的名称
    //     // 任意一个想要实现 HelloMacro 的结构都必须实现方法 hello_macro,
    //     // 我们无法提供hello_macro的默认实现,因为默认实现无法获取当前的实现类型的名称
    //     fn hello_macro() {
    //         println!("Hello, Macro! My name is Pancakes!")
    //     }
    // }

    Pancakes::hello_macro();

    // 我们需要一个在编译时生成代码的宏
    // hello_macro_derive/src/lib.rs

    /*
    类属性宏
        #[route(GET, "/")]
        fn index() {}

    #[proc_macro_attribute]
    pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}


    类函数宏
        let sql = sql!(SELECT * FROM posts WHERE id=1);

    #[proc_macro]
    pub fn sql(input: TokenStream) -> TokenStream {}

    */
}
