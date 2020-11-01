mod module; // 这将告诉 Rust 在另一个与模块同名的文件中加载模块的内容

pub use crate::module::hosting;

// 使用文件结构拆分模块树
pub fn test() {
    hosting::add();
}
