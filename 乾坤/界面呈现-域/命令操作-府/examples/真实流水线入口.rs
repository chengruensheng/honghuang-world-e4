//! 真实流水线入口 - 端到端 4 分类协作（真实 LLM / mock）+ 记忆工具闭环
//!
//! 决策锚：260827-AI助手自给自足（Round 11.5）
//! 用法：
//!   LLM_API_KEY=xxx cargo run -p mingling_caozuo_fu --example 真实流水线入口 -- <任务标识>
//!   cargo run -p mingling_caozuo_fu --example 真实流水线入口 -- --demo <任务标识>   （mock LLM，免 key）
//! 验证：任务前读相关格位（持久库）→ 跑流水线 → 任务后写程序/实施记忆 → 永驻摘要 36 行

use mingling_caozuo_fu::{
    写入任务记忆, 工具永驻摘要, 读任务记忆, 跑流水线_mock_llm, 跑流水线_真实_llm, 默认记忆库路径,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let demo = args.iter().any(|s| s == "--demo");
    let 任务标识 = args
        .iter()
        .find(|s| *s != "--demo")
        .map(|s| s.as_str())
        .unwrap_or("真实流水线测试");
    println!(
        "=== 端到端 4 分类协作（{} + 记忆工具闭环）===",
        if demo { "mock LLM" } else { "真实 LLM" }
    );
    println!("任务：{}", 任务标识);
    println!("记忆库：{}", 默认记忆库路径);

    // 任务前：读取相关格位（持久库）
    println!(
        "
--- 任务前读取相关格位（持久库）---"
    );
    let 相关记忆 = 读任务记忆(默认记忆库路径, 任务标识);
    if 相关记忆.is_empty() {
        println!("  （无相关记忆命中）");
    }
    for 记忆 in &相关记忆 {
        println!("  {}", 记忆);
    }

    // 跑流水线
    println!(
        "
--- 跑流水线 ---"
    );
    let 结果 = if demo {
        跑流水线_mock_llm(任务标识)
    } else {
        跑流水线_真实_llm(任务标识)
    };
    print!("{}", 结果.输出);

    // 任务后：写入程序/实施记忆 + 永驻摘要
    println!(
        "
--- 任务后写入程序记忆 + 永驻摘要 ---"
    );
    写入任务记忆(
        默认记忆库路径,
        &format!("流水线执行完成：{}", 任务标识),
        任务标识,
    )
    .expect("任务记忆写入失败");
    let 摘要 = 工具永驻摘要(默认记忆库路径);
    let 有内容 = 摘要.iter().filter(|s| !s.ends_with("] ")).count();
    println!("  永驻摘要共 {} 行，其中 {} 行有内容：", 摘要.len(), 有内容);
    for 行 in 摘要.iter().filter(|s| !s.ends_with("] ")) {
        println!("  {}", 行);
    }

    std::process::exit(结果.退出码);
}
