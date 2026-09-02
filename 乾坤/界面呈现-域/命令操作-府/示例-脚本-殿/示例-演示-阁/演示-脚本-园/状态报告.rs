//! 状态报告 - 36 格位块数 + 账本债务 + 待补提炼快照 + 玉玺块数，一键输出四段状态
//!
//! 决策锚：100项任务 任务18 当前状态报告自动生成（持久化真实边界可观测）
//! 用法：cargo run -p mingling_caozuo_fu --example 状态报告

use mingling_caozuo_fu::{状态报告, 默认记忆库路径};

fn main() {
    match 状态报告(默认记忆库路径) {
        Ok(行) => {
            for 行 in 行 {
                println!("{}", 行);
            }
        }
        Err(e) => {
            eprintln!("状态报告失败：{}", e);
            std::process::exit(1);
        }
    }
}
