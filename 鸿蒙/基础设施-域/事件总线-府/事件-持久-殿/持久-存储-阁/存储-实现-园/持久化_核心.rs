//! 持久化殿 - JSONL append-only 序列化/反序列化（无 serde 依赖，手写 JSON 行）
//!
//! 决策锚：260826-2230 工程-DSH § Waterfall 事件 + frozen outcome
//! 关联文档：02-概念/事件流/04-事件流.md + 02-概念/不可逆结果/07-不可逆结果.md

// 跨殿引用：事件类型与错误定义在事件类型殿（六层返工后改用 crate:: 路径）
use crate::事件_类型_殿::{事件, 事件类型, 分发模式, 错误};

// ============================================================================
// 序列化（无 serde 依赖，手写 JSON 行）
// ============================================================================

pub fn 序列化为行(事件: &事件) -> String {
    let 类型名 = match 事件.类型 {
        事件类型::会话 => "会话",
        事件类型::智能体 => "智能体",
        事件类型::能力 => "能力",
    };
    let 模式名 = match 事件.模式 {
        分发模式::瀑布 => "瀑布",
        分发模式::串行 => "串行",
    };
    format!(
        r#"{{"id":{},"类型":"{}","模式":"{}","prev_hash":"{:x}","名称":"{}","时间戳_毫秒":{},"决定者":"{}","hash":"{:x}","immutable":{}}}"#,
        事件.id,
        类型名,
        模式名,
        事件.prev_hash,
        事件.名称,
        事件.时间戳_毫秒,
        事件.决定者,
        事件.hash,
        事件.immutable
    )
}

pub fn 反序列化行(行: &str) -> Result<事件, 错误> {
    let mut id = 0u64;
    let mut prev_hash = 0u64;
    let mut hash = 0u64;
    let mut 名称 = String::new();
    let mut 时间戳_毫秒 = 0u64;
    let mut 决定者 = String::new();
    let mut immutable = false;
    let mut 类型 = 事件类型::会话;
    let mut 模式 = 分发模式::瀑布;

    // 简单键值对解析（适用于本模块写出的固定格式）
    let 内部 = 行.trim().trim_start_matches('{').trim_end_matches('}');
    for 部分 in 内部.split(',') {
        let 部分 = 部分.trim();
        if 部分.is_empty() {
            continue;
        }
        let kv: Vec<&str> = 部分.splitn(2, ':').collect();
        if kv.len() != 2 {
            continue;
        }
        let k = kv[0].trim().trim_matches('"');
        let v = kv[1].trim().trim_matches(',');
        match k {
            "id" => id = v.parse().unwrap_or(0),
            "类型" => {
                类型 = match v.trim_matches('"') {
                    "会话" => 事件类型::会话,
                    "智能体" => 事件类型::智能体,
                    "能力" => 事件类型::能力,
                    _ => return Err(错误::IO错误("未知类型".to_string())),
                }
            }
            "模式" => {
                模式 = match v.trim_matches('"') {
                    "瀑布" => 分发模式::瀑布,
                    "串行" => 分发模式::串行,
                    _ => return Err(错误::IO错误("未知模式".to_string())),
                }
            }
            "prev_hash" => prev_hash = u64::from_str_radix(v.trim_matches('"'), 16).unwrap_or(0),
            "名称" => 名称 = v.trim_matches('"').to_string(),
            "时间戳_毫秒" => 时间戳_毫秒 = v.parse().unwrap_or(0),
            "决定者" => 决定者 = v.trim_matches('"').to_string(),
            "hash" => hash = u64::from_str_radix(v.trim_matches('"'), 16).unwrap_or(0),
            "immutable" => immutable = v == "true",
            _ => {}
        }
    }

    Ok(事件 {
        id,
        类型,
        模式,
        prev_hash,
        名称,
        时间戳_毫秒,
        决定者,
        hash,
        immutable,
    })
}
