//! 自检命令入口示例
//!
//! 决策锚：260827-AI助手自给自足（Round 11）
//! 用法：cargo run -p mingling_caozuo_fu --example 自检入口 -- [--help|init|status|run --task=|自检]
//! 备注：本示例跑分发函数（所有命令），重点跑「自检」输出 13 项门禁状态

use mingling_caozuo_fu::{分发};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let 结果 = 分发(&arg_refs);
    print!("{}", 结果.输出);
    std::process::exit(结果.退出码);
}
