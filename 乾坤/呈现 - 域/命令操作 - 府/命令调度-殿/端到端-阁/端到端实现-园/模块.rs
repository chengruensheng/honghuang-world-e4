//! 端到端实现-园 模块桥接

#[path = "模拟_llm.rs"]
pub mod 模拟_llm;

pub use 模拟_llm::*;
