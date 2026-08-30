//! 自举任务入口（example）：真实 LLM 驱动系统开发自己
//!
//! 用法：cargo run -p mingling_caozuo_fu --example 自举任务入口
//! 全过程日志含 4 角色 + 道祖终裁的 LLM 回复原文 + 确定性落盘/cargo 验证证据。

use mingling_caozuo_fu::跑流水线_自举;
use renwu_zhixing_fu::自举任务单;
use std::collections::HashMap;

fn main() {
    // 自举任务单：修复版本漂移（世界 crate 版本号 v0.1.0「阶段 1 地基」→ v0.2.0「阶段 3 自举」）
    let mut 参数 = HashMap::new();
    参数.insert("标识".to_string(), "自举-版本漂移修复".to_string());
    参数.insert(
        "目标文件".to_string(),
        "世界/工作空间-状态-殿/工作空间版本-阁/工作空间版本标识-园/版本_核心.rs".to_string(),
    );
    参数.insert(
        "需求描述".to_string(),
        "把版本号常量从 v0.1.0 更新为 v0.2.0，阶段常量从「阶段 1 地基」更新为「阶段 3 自举」，保留现有测试与中文标识符，不得引入英文注释。".to_string(),
    );
    参数.insert("验收命令".to_string(), "cargo build".to_string());
    参数.insert(
        "可证伪命题".to_string(),
        "cargo build 退出码 0 且 版本_核心.rs 中版本=v0.2.0".to_string(),
    );
    参数.insert("decided_by".to_string(), "界主".to_string());

    let 单 = match 自举任务单::从参数解析(&参数) {
        Ok(单) => 单,
        Err(e) => {
            eprintln!("任务单解析失败：{}", e);
            std::process::exit(1);
        }
    };

    println!("========== 自举流水线启动 ==========");
    let 结果 = 跑流水线_自举(&单);
    println!("========== 全过程日志 ==========");
    println!("{}", 结果.输出);
    println!("========== 退出码 {} ==========", 结果.退出码);
    std::process::exit(结果.退出码);
}
