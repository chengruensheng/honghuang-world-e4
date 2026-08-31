//! 自检命令 - 聚合 15 项精简门禁（含第 5 项 15 项架构校验）（AI 助手自给自足入口）
//!
//! 决策锚：260827-AI助手自给自足（Round 11）
//! falsifiable：单次 cargo run --bin -- 自检 输出 15 项精简门禁通过/失败状态

use crate::命令_数据_殿::自检_聚合_阁::自检_调用_阁::自检_结果_园::{
    状态, 跑全检,
};
use crate::{命令, 命令结果};
use std::path::PathBuf;

pub struct 自检命令;

fn 找工作空间根() -> Option<PathBuf> {
    let mut 目录 = std::env::current_dir().ok()?;
    loop {
        let 清单 = 目录.join("Cargo.toml");
        if 清单.exists() {
            if let Ok(内容) = std::fs::read_to_string(&清单) {
                if 内容.contains("[workspace]") {
                    return Some(目录);
                }
            }
        }
        if !目录.pop() {
            return None;
        }
    }
}

impl 命令 for 自检命令 {
    fn 名称(&self) -> &str {
        "自检"
    }
    fn 执行(&self, _参数: &[&str]) -> 命令结果 {
        let 工作空间根 = 找工作空间根().unwrap_or_else(|| std::env::current_dir().unwrap());
        let 报告 = 跑全检(&工作空间根);
        let mut 输出 = format!("{}\n\n", 报告.摘要());
        for 项 in &报告.项 {
            let 符号 = match 项.状态 {
                状态::通过 => "✓",
                状态::警告 => "△",
                状态::失败 => "✗",
            };
            输出.push_str(&format!(
                "{} {:2}. {}\n   {}\n",
                符号, 项.编号, 项.名称, 项.详情
            ));
        }
        输出.push_str(&format!("\n工作空间根：{}", 报告.项目根));
        if 报告.通过() {
            命令结果::成功(输出)
        } else {
            命令结果::失败(1, 输出)
        }
    }
}
