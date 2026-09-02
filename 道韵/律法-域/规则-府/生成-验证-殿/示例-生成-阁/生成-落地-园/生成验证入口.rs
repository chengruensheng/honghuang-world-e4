//! 生成验证入口（example）：确定性验证 LLM 产出的代码
//!
//! 这是「洪荒 真任务 --落盘」的固定挂载点：
//! LLM 产出代码 → 确定性执行器覆写本目录下的 生成区.rs → cargo 编译本 example
//! （include! 静态挂载，必然编译）→ cargo test 真实执行生成区内的 #[test] 测试。
//!
//! 使用：cargo test -p guize_fu --example 生成验证入口
//! 约定：生成区.rs 由 LLM 产出覆写，只能含 #[test] 测试函数与辅助 fn
//!       （不得定义 main/生成区_主流程，避免与入口冲突）。
//! 本文件不随任务变化。

include!("生成区.rs");

fn main() {
    // 验证由生成区内的 #[test] 测试驱动（cargo test --example 执行），main 保持空转
}
