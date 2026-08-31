//! 打回闭环入口（example）：阶段 4 最小闭环——大罗产出不对 → 打回 → 带改码要求重投 → 再落盘验证 → 直到通过
//!
//! 用法：cargo run -p mingling_caozuo_fu --example 打回闭环入口
//! 任务设计：需求只写主路径规格（不含空输入哨兵），大罗首轮盲写大概率漏哨兵 →
//! 预置断言红（校验断言-园 是大罗不可见不可改的确定性锚）→ 打回 → 重投带上一轮执行证据 →
//! 次轮补哨兵 → cargo test 全绿 → 终裁通过 → 交付回填。

use mingling_caozuo_fu::跑流水线_自举;
use renwu_zhixing_fu::自举任务单;
use std::collections::HashMap;

fn main() {
    // 阶段 4 任务单：滚动校验字重铸（需求故意省略空输入哨兵规格——由预置断言兜底验收）
    let mut 参数 = HashMap::new();
    参数.insert("标识".to_string(), "自举-滚动校验字重铸".to_string());
    参数.insert(
        "目标文件".to_string(),
        "乾坤/工具-校验和-府/实现-殿/校验和-方法-阁/数据校验-园/数据校验.rs".to_string(),
    );
    参数.insert(
        "需求描述".to_string(),
        "在数据校验.rs 中完整重铸两个公开函数（文件是完整可编译 Rust 源文件，中文标识符与中文注释，无第三方依赖、无 unsafe、无英文函数名）：① pub fn 位置加权和(数据: &[u8]) -> u64：每个字节值乘以位置权重（首字节权重 1，逐字节递增 1）后求和，示例 b\"abc\" 应得 590；② pub fn 滚动校验字(数据: &[u8]) -> u32：把字节序列每 4 字节划为一组，组内按小端序（首字节为最低位）拼成 u32，所有组按位异或得到结果，示例 b\"ABCDEFGH\" 应得 0x0C040404，b\"12345\" 应得 0x34333204。两个函数必须保留且签名不变（仓库预置断言引用它们）。".to_string(),
    );
    参数.insert(
        "验收命令".to_string(),
        "cargo test -p gongju_jiaoyanhe_fu".to_string(),
    );
    参数.insert(
        "可证伪命题".to_string(),
        "cargo test -p gongju_jiaoyanhe_fu 退出码 0（预置断言含空输入哨兵 0xFFFFFFFF 全绿），且 位置加权和/滚动校验字 两函数签名不变，且范围外文件（成员_核心.rs / 版本_核心.rs）内容不变".to_string(),
    );
    参数.insert("decided_by".to_string(), "界主".to_string());

    let 单 = match 自举任务单::从参数解析(&参数) {
        Ok(单) => 单,
        Err(e) => {
            eprintln!("任务单解析失败：{}", e);
            std::process::exit(1);
        }
    };

    println!("========== 阶段 4 打回闭环流水线启动 ==========");
    let 结果 = 跑流水线_自举(&单);
    println!("========== 全过程日志 ==========");
    println!("{}", 结果.输出);
    println!("========== 退出码 {} ==========", 结果.退出码);
    std::process::exit(结果.退出码);
}
