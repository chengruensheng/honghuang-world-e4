//! 代码提取-阁 - 从大罗产出确定性提取「目标文件路径 + 代码块」
//!
//! 治理铁律：LLM 只产意图（文本），确定性程序提取并执行，
//! 提取规则纯机械（找标记行 + 代码围栏），不依赖 LLM 自报路径的可信度。
//! 决策锚：260830 第一版自举规划（阶段 2 流水线接工具循环）。

/// 提取代码块：收集所有 \u{60}\u{60}\u{60} 围栏，优先 \u{60}\u{60}\u{60}rust 围栏，
/// 找不到则回退到最长的无语言标记围栏（代码通常最长）；绝不取诊断命令/JSON/脚本围栏。
///
/// 稳定性教训（260830 批量自举回归）：大罗产出常含多个围栏（诊断命令、JSON、脚本、真代码），
/// 「取第一对围栏」会把诊断命令写进 .rs 导致落盘污染；必须按语言标记筛选。
pub fn 提取代码块(产出: &str) -> String {
    const 围栏: &str = "\u{60}\u{60}\u{60}";
    // (语言标记, 内容)
    let mut 块列表: Vec<(String, String)> = Vec::new();
    let mut 在块内 = false;
    let mut 当前语言 = String::new();
    let mut 当前内容 = String::new();
    for 行 in 产出.lines() {
        let 行去空格 = 行.trim_start();
        if 行去空格.starts_with(围栏) {
            if 在块内 {
                // 结束围栏 → 保存块
                块列表.push((当前语言.clone(), 当前内容.clone()));
                当前内容.clear();
                当前语言.clear();
                在块内 = false;
            } else {
                // 开始围栏 → 读语言标记（如 rust / json / powershell）
                在块内 = true;
                当前语言 = 行去空格
                    .strip_prefix(围栏)
                    .unwrap_or_default()
                    .trim()
                    .to_string();
            }
            continue;
        }
        if 在块内 {
            当前内容.push_str(行);
            当前内容.push('\n');
        }
    }
    // 未闭合围栏不保存（半成品丢弃）
    // 优先 rust 围栏（大罗意图明确标为 Rust 代码）
    for (语言, 内容) in &块列表 {
        if 语言.starts_with("rust") {
            return 内容.clone();
        }
    }
    // 回退：最长的无语言标记围栏（真代码通常是最长的那块）
    let mut 候选: Option<&String> = None;
    for (语言, 内容) in &块列表 {
        if 语言.is_empty() && 候选.map_or(true, |已有| 内容.len() > 已有.len()) {
            候选 = Some(内容);
        }
    }
    候选.cloned().unwrap_or_default()
}

/// 提取目标文件路径：找「目标文件」「文件路径」「文件：」标记行后的路径
pub fn 提取目标文件(产出: &str) -> Option<String> {
    for 行 in 产出.lines() {
        for 标记 in [
            "目标文件：",
            "目标文件:",
            "文件路径：",
            "文件路径:",
            "文件：",
            "文件:",
        ] {
            if let Some(位置) = 行.find(标记) {
                let 路径 = 行[位置 + 标记.len()..].trim();
                if !路径.is_empty() {
                    return Some(路径.to_string());
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 提取代码块去语言标记() {
        let 产出 = "目标文件：世界/入口.rs\n代码：\n\u{60}\u{60}\u{60}rust\npub fn 版本() {}\n\u{60}\u{60}\u{60}";
        let 块 = 提取代码块(产出);
        assert!(块.contains("pub fn 版本()"), "代码块应含代码：{}", 块);
        assert!(!块.contains("rust"), "语言标记应被去掉：{}", 块);
    }

    #[test]
    fn 多围栏时优先rust而非诊断命令() {
        // 稳定性回归：大罗产出含诊断命令围栏 + JSON 围栏 + rust 代码围栏，
        // 必须提取 rust 围栏，绝不把诊断命令/JSON 写进 .rs
        let 产出 = "先看结构：\n\u{60}\u{60}\u{60}\n{\"command\":\"cat 就绪_核心.rs\"}\n\u{60}\u{60}\u{60}\n然后写代码：\n\u{60}\u{60}\u{60}rust\npub fn 就绪() -> bool { true }\n\u{60}\u{60}\u{60}";
        let 块 = 提取代码块(产出);
        assert!(
            块.contains("pub fn 就绪"),
            "应提取 rust 围栏而非诊断命令：{}",
            块
        );
        assert!(!块.contains("command"), "不得提取 JSON 诊断围栏：{}", 块);
    }

    #[test]
    fn 无rust围栏回退最长无标记块() {
        let 产出 = "短：\n\u{60}\u{60}\u{60}\nfn 短() {}\n\u{60}\u{60}\u{60}\n长：\n\u{60}\u{60}\u{60}\npub fn 长函数() { /* 更多内容 */ }\n\u{60}\u{60}\u{60}";
        let 块 = 提取代码块(产出);
        assert!(块.contains("长函数"), "应回退到最长无标记围栏：{}", 块);
    }

    #[test]
    fn 提取目标文件路径() {
        let 产出 = "目标文件：世界/入口.rs\n代码：\u{60}\u{60}\u{60}\nfn x(){}\n\u{60}\u{60}\u{60}";
        assert_eq!(提取目标文件(产出).as_deref(), Some("世界/入口.rs"));
    }

    #[test]
    fn 无标记返回空() {
        assert_eq!(提取目标文件("没有标记"), None);
        assert!(提取代码块("没有代码块").is_empty());
    }
}
