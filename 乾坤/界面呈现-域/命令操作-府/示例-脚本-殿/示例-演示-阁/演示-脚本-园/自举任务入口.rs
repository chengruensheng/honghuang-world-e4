//! 自举任务入口（example）：真实 LLM 驱动系统开发自己
//!
//! 用法：cargo run -p mingling_caozuo_fu --example 自举任务入口
//! 全过程日志含 4 角色 + 道祖终裁的 LLM 回复原文 + 确定性落盘/cargo 验证证据。

use mingling_caozuo_fu::跑流水线_自举;
use renwu_zhixing_fu::自举任务单;
use std::collections::HashMap;

fn main() {
    // 自举任务单：新增「阶段」常量（生产 CLI 归位九根后，版本标识园补阶段锚点）
    let mut 参数 = HashMap::new();
    参数.insert("标识".to_string(), "自举-阶段常量落地".to_string());
    参数.insert(
        "目标文件".to_string(),
        "乾坤/界面呈现-域/命令操作-府/命令-数据-殿/版本-查询-阁/版本-标识-园/版本_核心.rs"
            .to_string(),
    );
    参数.insert(
        "需求描述".to_string(),
        "在版本_核心.rs 中新增一个「阶段」常量（pub const 阶段: &str = \"阶段 3 自举\"），保留现有「版本」「项目」常量与测试，新增一条针对「阶段」的测试（断言等于「阶段 3 自举」），标注 decided_by/falsifiable/implements/复现命令四要素，保持中文标识符与中文注释，不得引入英文注释。".to_string(),
    );
    参数.insert("验收命令".to_string(), "cargo build".to_string());
    参数.insert(
        "可证伪命题".to_string(),
        "cargo build 退出码 0 且 版本_核心.rs 中 阶段=阶段 3 自举".to_string(),
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
