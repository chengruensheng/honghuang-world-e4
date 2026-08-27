//! 真实流水线入口 - 端到端 4 分类协作（真实 LLM）
//!
//! 决策锚：260827-AI助手自给自足（Round 11.5）
//! 用法：LLM_API_KEY=xxx cargo run -p mingling_caozuo_fu --example 真实流水线入口 -- <任务标识>
//! 验证：真实 LLM 跑完整流水线（道祖→圣人→大罗→准圣→道祖终裁）

use mingling_caozuo_fu::跑流水线_真实_llm;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let 任务标识 = args.first().map(|s| s.as_str()).unwrap_or("真实流水线测试");
    println!("=== 端到端 4 分类协作（真实 LLM）===");
    println!("任务：{}", 任务标识);
    let 结果 = 跑流水线_真实_llm(任务标识);
    print!("{}", 结果.输出);
    std::process::exit(结果.退出码);
}
