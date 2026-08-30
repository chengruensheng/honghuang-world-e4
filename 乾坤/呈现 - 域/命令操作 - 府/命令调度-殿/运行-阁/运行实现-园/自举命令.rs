//! 自举命令-园 - 生产 CLI 自举入口：从任务单文件读取 → 真实 LLM 流水线自举
//!
//! 阶段 4 最小闭环：把自举从 example 升级为生产命令，
//! 单任务「自举 --file=<单.json>」/ 批量「自举 --list=<列表.json>」，
//! 批量结构化汇总成功/失败/评分/债务，失败任务正确返回非零退出码。
//! 决策锚：260830 阶段 4 批量自举+打回改码闭环（生产 CLI 化）。

use super::super::super::{命令, 命令结果, 跑流水线_自举, 退出码};
use renwu_zhixing_fu::自举任务单;
use std::collections::HashMap;

pub struct 自举命令;

impl 命令 for 自举命令 {
    fn 名称(&self) -> &str {
        "自举"
    }
    fn 执行(&self, 参数: &[&str]) -> 命令结果 {
        if let Some(路径) = 取值(参数, "--list=") {
            return 跑批量自举(&路径);
        }
        if let Some(路径) = 取值(参数, "--file=") {
            return 跑单自举(&路径);
        }
        命令结果::失败(
            退出码::参数错误,
            "用法：自举 --file=<任务单.json> 或 自举 --list=<任务列表.json>",
        )
    }
}

fn 取值<'a>(参数: &'a [&'a str], 前缀: &str) -> Option<String> {
    参数
        .iter()
        .find_map(|a| a.strip_prefix(前缀).map(|s| s.to_string()))
}

/// 读任务单 JSON（中文键六字段：标识/目标文件/需求描述/验收命令/可证伪命题/decided_by）
pub fn 读任务单文件(路径: &str) -> Result<自举任务单, String> {
    let 内容 = std::fs::read_to_string(路径).map_err(|e| format!("读任务单失败：{}", e))?;
    let 参数: HashMap<String, String> =
        serde_json::from_str(&内容).map_err(|e| format!("任务单 JSON 解析失败：{}", e))?;
    自举任务单::从参数解析(&参数)
}

/// 读任务列表 JSON（任务单数组）
pub fn 读任务列表文件(路径: &str) -> Result<Vec<自举任务单>, String> {
    let 内容 = std::fs::read_to_string(路径).map_err(|e| format!("读任务列表失败：{}", e))?;
    let 列表: Vec<HashMap<String, String>> =
        serde_json::from_str(&内容).map_err(|e| format!("任务列表 JSON 解析失败：{}", e))?;
    列表.iter().map(自举任务单::从参数解析).collect()
}

/// 跑单个自举任务（真实 LLM 完整证据链，退出码直接透传）
pub fn 跑单自举(路径: &str) -> 命令结果 {
    match 读任务单文件(路径) {
        Ok(单) => 跑流水线_自举(&单),
        Err(e) => 命令结果::失败(退出码::材料错误, e),
    }
}

/// 跑批量自举：串行跑多个任务（各自独立证据链），结构化汇总
pub fn 跑批量自举(路径: &str) -> 命令结果 {
    let 列表 = match 读任务列表文件(路径) {
        Ok(l) if !l.is_empty() => l,
        Ok(_) => return 命令结果::失败(退出码::材料错误, "任务列表为空".to_string()),
        Err(e) => return 命令结果::失败(退出码::材料错误, e),
    };
    let 总数 = 列表.len();
    let mut 成功数 = 0usize;
    let mut 失败数 = 0usize;
    let mut 汇总 = format!("[批量自举] 任务总数={}\n", 总数);
    for (序, 单) in 列表.iter().enumerate() {
        汇总.push_str(&format!(
            "\n===== [{}/{}] {} =====\n",
            序 + 1,
            总数,
            单.标识
        ));
        let 结果 = 跑流水线_自举(单);
        汇总.push_str(&结果.输出);
        if 结果.退出码 == 0 {
            成功数 += 1;
        } else {
            失败数 += 1;
            汇总.push_str(&format!(
                "[批量自举] 任务「{}」失败（退出码 {}）\n",
                单.标识, 结果.退出码
            ));
        }
    }
    汇总.push_str(&format!(
        "\n[批量自举·汇总] 成功 {} / 失败 {} / 总计 {}\n",
        成功数, 失败数, 总数
    ));
    if 失败数 == 0 {
        命令结果::成功(汇总)
    } else {
        // 有任务未通过终裁：状态机违规（流水线闭环未收敛），不得返回成功
        命令结果::失败(退出码::状态机违规, 汇总)
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 缺参数返回用法() {
        let r = 自举命令.执行(&[]);
        assert_eq!(r.退出码, 1);
        assert!(r.输出.contains("--file="));
    }

    #[test]
    fn 任务单文件不存在报材料错误() {
        let r = 跑单自举("不存在的任务单.json");
        assert_eq!(r.退出码, 2);
        assert!(r.输出.contains("读任务单失败"));
    }

    #[test]
    fn 任务单缺字段报错() {
        let 临时 = std::env::temp_dir().join("自举命令_缺字段.json");
        std::fs::write(&临时, r#"{"标识":"自举-x"}"#).unwrap();
        let r = 跑单自举(临时.to_str().unwrap());
        assert_eq!(r.退出码, 2);
        assert!(r.输出.contains("缺必填字段"));
        std::fs::remove_file(&临时).ok();
    }

    #[test]
    fn 任务列表空数组报材料错误() {
        let 临时 = std::env::temp_dir().join("自举命令_空列表.json");
        std::fs::write(&临时, "[]").unwrap();
        let r = 跑批量自举(临时.to_str().unwrap());
        assert_eq!(r.退出码, 2);
        assert!(r.输出.contains("任务列表为空"));
        std::fs::remove_file(&临时).ok();
    }
}
