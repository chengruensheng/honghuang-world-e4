//! 记忆存储-殿 - 桥接阁（存储操作阁 + 内存阁 + SQLite 阁）

#[path = "存储编排-阁/模块.rs"]
pub mod 存储编排_阁;
pub use 存储编排_阁::*;

#[path = "内存存储-阁/模块.rs"]
pub mod 内存存储_阁;
pub use 内存存储_阁::*;

// SQLite 阁保留原语义：仅在测试时编译
#[cfg(test)]
#[allow(non_snake_case)] // 中文模块名 + 大写缩写 SQLite 不遵循 snake_case
#[path = "SQLite存储-阁/模块.rs"]
pub mod SQLite存储_阁;
