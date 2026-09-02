//! 脚本实现 - 门禁脚本校验 + 一键全验 + 跑全部聚合
//!
//! 聚合架构校验殿 + 脚本殿的所有检查项，返回 15 项校验结果。
//!
//! 决策锚：v4 阶段 16 架构校验 Rust 化

use std::path::Path;

use crate::架构_校验_殿::检查_府级crate至少2殿;
use crate::架构_校验_殿::{
    校验项, 检查_README, 检查_crate命名风格, 检查_workspace_members, 检查_传承殿8大类,
    检查_同层命名唯一, 检查_实施方案文档, 检查_府crate_入口rs, 检查_所有目录含中文, 检查_无_github,
    检查_无_src平铺, 检查_目录名无英文, 检查_祖孙不同名, 检查结果,
};

pub fn 检查_门禁脚本(根: &Path) -> 检查结果 {
    let guard_dir = 根.join("证道").join("质量门禁-域").join("门禁-府");
    if !guard_dir.exists() {
        return 检查结果::失败("门禁-府 不存在".to_string());
    }
    let count = walkdir::WalkDir::new(&guard_dir)
        .into_iter()
        .flatten()
        .filter(|e| {
            let n = e.file_name().to_string_lossy();
            n.starts_with("验证-") && n.ends_with(".ps1")
        })
        .count();
    if count < 5 {
        检查结果::失败(format!("脚本 = {} < 5", count))
    } else {
        检查结果::通过
    }
}

pub fn 检查_一键全验(根: &Path) -> 检查结果 {
    if 根.join("一键全验.sh").exists() {
        检查结果::通过
    } else {
        检查结果::失败("一键全验.sh 不存在".to_string())
    }
}

pub fn 跑全部(根: &Path) -> Vec<校验项> {
    vec![
        校验项 {
            编号: 1,
            名称: "无 src/ 平铺目录".into(),
            结果: 检查_无_src平铺(根),
        },
        校验项 {
            编号: 2,
            名称: "无 .github/ 顶层目录".into(),
            结果: 检查_无_github(根),
        },
        校验项 {
            编号: 3,
            名称: "所有目录含中文".into(),
            结果: 检查_所有目录含中文(根),
        },
        校验项 {
            编号: 4,
            名称: "crate 名称 *_fu 风格".into(),
            结果: 检查_crate命名风格(根),
        },
        校验项 {
            编号: 5,
            名称: "所有府 crate 用 入口.rs".into(),
            结果: 检查_府crate_入口rs(根),
        },
        校验项 {
            编号: 6,
            名称: "workspace members >= 15".into(),
            结果: 检查_workspace_members(根),
        },
        校验项 {
            编号: 7,
            名称: "传承殿 6 大类目录完整".into(),
            结果: 检查_传承殿8大类(根),
        },
        校验项 {
            编号: 8,
            名称: "决策契约文档 >= 7".into(),
            结果: 检查_实施方案文档(根),
        },
        校验项 {
            编号: 9,
            名称: ">= 5 项门禁脚本".into(),
            结果: 检查_门禁脚本(根),
        },
        校验项 {
            编号: 10,
            名称: "一键全验.sh 存在".into(),
            结果: 检查_一键全验(根),
        },
        校验项 {
            编号: 11,
            名称: "README 存在".into(),
            结果: 检查_README(根),
        },
        校验项 {
            编号: 12,
            名称: "殿/阁/园 祖孙不同名".into(),
            结果: 检查_祖孙不同名(根),
        },
        校验项 {
            编号: 13,
            名称: "同层命名全局唯一".into(),
            结果: 检查_同层命名唯一(根),
        },
        校验项 {
            编号: 14,
            名称: "目录名无英文残留".into(),
            结果: 检查_目录名无英文(根),
        },
        校验项 {
            编号: 15,
            名称: "所有府级 crate ≥2 殿（防退化）".into(),
            结果: 检查_府级crate至少2殿(根),
        },
    ]
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 测试_跑全部_返回15项() {
        let items =
            跑全部(&std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")));
        assert_eq!(items.len(), 15);
    }
}
