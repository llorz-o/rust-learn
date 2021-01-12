use std::slice;

pub fn run() {
    /*
        裸指针

        可以在安全代码中 创建 裸指针，只是不能在不安全块之外 解引用 裸指针，
    */

    let mut num = 5;
    let r1 = &num as *const i32; // 使用 as 转为 不可变引用裸指针
    let r2 = &mut num as *mut i32; // 使用 as 转为 可变引用裸指针

    unsafe {
        // 在 unsafe 块中解引用裸指针
        // 非安全代码同时允许可变引用和不可变引用存在
        println!("r1 is: {}", *r1);
        println!("r1 is: {}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // 事实上你无法通过非安全代码实现这么一个操作
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr(); // as_mut_ptr 返回一个 *mut i32 类型的裸指针
        assert!(mid <= len);
        // (&mut v[..mid], &mut v[mid..]) // 无法同时出现两个可变引用
        unsafe {
            /*
                slice::from_raw_parts_mut 函数是不安全的因为它获取一个裸指针，并必须确信这个指针是有效的。
                裸指针上的 add 方法也是不安全的，因为其必须确信此地址偏移量也是有效的指针。
                因此必须将 slice::from_raw_parts_mut 和 add 放入 unsafe 块中以便能调用它们。

            */

            (
                slice::from_raw_parts_mut(ptr, mid), // slice::from_raw_parts_mut 函数获取一个裸指针和一个长度来创建一个 slice。
                slice::from_raw_parts_mut(ptr.add(mid), len - mid), // 在 ptr 上调用 add 方法并使用 mid 作为参数来获取一个从 mid 开始的裸指针，
            )
        }
    }

    let r2 = &mut v[..];

    let (a, b) = split_at_mut(r2, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // 创建一个指向任意内存地址的裸指针。尝试使用任意内存是未定义行为：此地址可能有数据也可能没有，
    // 编译器可能会优化掉这个内存访问，或者程序可能会出现段错误（segmentation fault）。
    let address = 0x0123456usize; // 指向未知地址的未知有效性的裸指针
    let r = address as *mut i32;

    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 1000) };

    // println!("{:?}", slice); // 试图使用臆测为有效的 slice 会导致未定义的行为。

    // extern 与其他语言编写的代码交互
    // 有助于创建和使用 外部函数接口（Foreign Function Interface， FFI）。
    // 外部函数接口是一个编程语言用以定义函数的方式，其允许不同（外部）编程语言调用这些函数。
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3)); // 外部函数的调用天然是不安全的
    }

    // no_mangle 标记该函数名在打包时不会丢失,避免暴露的函数名改变而导致无法调用
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    // 静态变量必须标注变量类型
    // 静态变量只能储存拥有 'static 生命周期的引用，这意味着 Rust 编译器可以自己计算出其生命周期而无需显式标注。
    // 访问不可变静态变量是安全的。
    static HELLO_WORLD: &'static str = "hello world"; // &str是天然具有 'static 生命周期的,可忽略 'static

    // 多个线程访问 COUNTER 则可能导致数据竞争。
    static mut COUNTER: u32 = 0;

    unsafe {
        // 访问可变静态变量都是不安全的
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }

    /*
        不安全的 trait ,当一个trait 中包含有编译器无法验证的不变量时,该trait就是不安全的;
    */

    unsafe trait Foo {}
}
