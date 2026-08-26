//! 校验-工具 - v4 阶段 16 架构校验 Rust 化
//!
//! 11 项架构校验（替代 验证-架构.ps1）。

#![allow(non_snake_case)]

use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 检查结果 {
    通过,
    失败(String),
    警告(String),
}

impl 检查结果 {
    pub fn 是否通过(&self) -> bool {
        matches!(self, 检查结果::通过)
    }
    pub fn 是否警告(&self) -> bool {
        matches!(self, 检查结果::警告(_))
    }
    pub fn 是否失败(&self) -> bool {
        matches!(self, 检查结果::失败(_))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 校验项 {
    pub 编号: u8,
    pub 名称: String,
    pub 结果: 检查结果,
}

fn 是排除目录(path: &Path) -> bool {
    let s = path.to_string_lossy();
    let 排除 = [
        ".git",
        ".cargo",
        "target",
        "node_modules",
        ".arts",
        ".codeartsdoer",
        ".codegraph",
        ".codebuddy",
        ".workbuddy",
        ".trae",
        ".vscode",
        ".venv",
        ".idea",
        ".DS_Store",
        "构建物-域",
        "debug",
        "doc",
        "incremental",
        "传\\承\\殿",
    ];
    for e in 排除.iter() {
        if s.contains(e) {
            return true;
        }
    }
    false
}

pub fn 检查_无_src平铺(根: &Path) -> 检查结果 {
    for entry in walkdir::WalkDir::new(根).max_depth(4).into_iter().flatten() {
        if entry.file_name().to_string_lossy() == "src" && entry.path().is_dir() {
            let p = entry.path().to_string_lossy().to_string();
            if !p.contains("传\\承\\殿") && !p.contains("node_modules") && !p.contains("构建物-域")
            {
                return 检查结果::失败(format!("发现 src/ 平铺：{}", p));
            }
        }
    }
    检查结果::通过
}

pub fn 检查_无_github(根: &Path) -> 检查结果 {
    if 根.join(".github").exists() {
        检查结果::警告(".github/ 顶层目录存在".to_string())
    } else {
        检查结果::通过
    }
}

pub fn 检查_所有目录含中文(根: &Path) -> 检查结果 {
    let 中文 = |s: &str| s.chars().any(|c| c as u32 >= 0x4E00 && c as u32 <= 0x9FFF);
    for entry in walkdir::WalkDir::new(根).max_depth(4).into_iter().flatten() {
        let p = entry.path();
        if !p.is_dir() {
            continue;
        }
        if 是排除目录(p) {
            continue;
        }
        if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
            if !中文(name) {
                return 检查结果::失败(format!("非中文目录：{}", p.display()));
            }
        }
    }
    检查结果::通过
}

pub fn 检查_crate命名风格(根: &Path) -> 检查结果 {
    for entry in walkdir::WalkDir::new(根).max_depth(4).into_iter().flatten() {
        if entry.file_name() != "Cargo.toml" {
            continue;
        }
        let content = match std::fs::read_to_string(entry.path()) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let name = match extract_crate_name(&content) {
            Some(n) => n,
            None => continue,
        };
        if name != "shijie" && !name.ends_with("_fu") {
            return 检查结果::失败(format!("crate 名称不规范：{}", name));
        }
    }
    检查结果::通过
}

fn extract_crate_name(content: &str) -> Option<String> {
    for line in content.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("name = ") {
            let rest = rest.trim();
            // 去掉首尾的 , " 等
            let name = rest.trim_matches(|c: char| c == ',' || c == '"' || c == ' ');
            return Some(name.to_string());
        }
    }
    None
}

pub fn 检查_府crate_入口rs(根: &Path) -> 检查结果 {
    for entry in walkdir::WalkDir::new(根).max_depth(5).into_iter().flatten() {
        if entry.file_name() != "Cargo.toml" {
            continue;
        }
        let content = match std::fs::read_to_string(entry.path()) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let name = match extract_crate_name(&content) {
            Some(n) => n,
            None => continue,
        };
        if !name.ends_with("_fu") {
            continue;
        }
        let dir = entry.path().parent().unwrap();
        let 入口 = dir.join("入口.rs");
        let lib = dir.join("src").join("lib.rs");
        let lib_main = dir.join("src").join("main.rs");
        if lib.exists() || lib_main.exists() {
            return 检查结果::失败(format!("{} 用了 src/", dir.display()));
        }
        if !入口.exists() {
            return 检查结果::失败(format!("{} 缺少 入口.rs", dir.display()));
        }
    }
    检查结果::通过
}

pub fn 检查_workspace_members(根: &Path) -> 检查结果 {
    let toml = match std::fs::read_to_string(根.join("Cargo.toml")) {
        Ok(c) => c,
        Err(_) => return 检查结果::失败("读 Cargo.toml 失败".to_string()),
    };
    let count = extract_members(&toml).len();
    if count < 15 {
        检查结果::失败(format!("workspace members = {} < 15", count))
    } else {
        检查结果::通过
    }
}

fn extract_members(content: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut in_members = false;
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("members") && t.contains("[") {
            in_members = true;
            continue;
        }
        if in_members {
            if t == "]" {
                in_members = false;
                continue;
            }
            if t.starts_with("#") {
                continue;
            }
            if let Some(start) = t.find('"') {
                if let Some(end) = t.rfind('"') {
                    if end > start {
                        result.push(t[start + 1..end].to_string());
                    }
                }
            }
        }
    }
    result
}

pub fn 检查_传承殿8大类(根: &Path) -> 检查结果 {
    let 传承殿 = 根.join("传\\承\\殿");
    if !传承殿.exists() {
        return 检查结果::失败("传承殿 不存在".to_string());
    }
    let 必需 = [
        "00-宪法",
        "01-哲学",
        "02-概念",
        "03-决策",
        "04-设计",
        "05-质量",
        "06-治理",
        "08-参考",
    ];
    let mut 缺失 = Vec::new();
    for d in 必需 {
        if !传承殿.join(d).exists() {
            缺失.push(d.to_string());
        }
    }
    if 缺失.is_empty() {
        检查结果::通过
    } else {
        检查结果::失败(format!("缺失：{}", 缺失.join(", ")))
    }
}

pub fn 检查_实施方案文档(根: &Path) -> 检查结果 {
    let plan_dir = 根.join("传\\承\\殿").join("10-地基");
    if !plan_dir.exists() {
        return 检查结果::失败("10-地基 不存在".to_string());
    }
    let count = walkdir::WalkDir::new(&plan_dir)
        .into_iter()
        .flatten()
        .filter(|e| {
            let n = e.file_name().to_string_lossy();
            n.contains("阶段") && n.contains("实施方案") && n.ends_with(".md")
        })
        .count();
    if count < 7 {
        检查结果::失败(format!("文档 = {} < 7", count))
    } else {
        检查结果::通过
    }
}

pub fn 检查_门禁脚本(根: &Path) -> 检查结果 {
    let guard_dir = 根.join("道果树").join("质量门禁-域").join("门禁-府");
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

pub fn 检查_README(根: &Path) -> 检查结果 {
    if 根.join("传\\承\\殿").join("README.md").exists() || 根.join("README.md").exists() {
        检查结果::通过
    } else {
        检查结果::失败("README 不存在".to_string())
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
            名称: "传承殿 8 大类目录完整".into(),
            结果: 检查_传承殿8大类(根),
        },
        校验项 {
            编号: 8,
            名称: "实施方案文档 >= 7".into(),
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
    ]
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 测试_排除目录() {
        assert!(是排除目录(Path::new("/tmp/target/debug/build")));
        assert!(是排除目录(Path::new("/tmp/道果树/构建物-域/debug")));
        assert!(!是排除目录(Path::new("/tmp/道果树/质量门禁-域")));
    }

    #[test]
    fn 测试_中文检测() {
        assert!("传"
            .chars()
            .any(|c| c as u32 >= 0x4E00 && c as u32 <= 0x9FFF));
        assert!(!"abc"
            .chars()
            .any(|c| c as u32 >= 0x4E00 && c as u32 <= 0x9FFF));
    }

    #[test]
    fn 测试_提取crate_name() {
        let c = "name = \"test_fu\"";
        assert_eq!(extract_crate_name(c), Some("test_fu".to_string()));
    }

    #[test]
    fn 测试_跑全部_返回11项() {
        let items =
            跑全部(&std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")));
        assert_eq!(items.len(), 11);
    }

    #[test]
    fn 测试_检查结果方法() {
        assert!(检查结果::通过.是否通过());
        assert!(!检查结果::通过.是否失败());
        assert!(检查结果::失败("x".to_string()).是否失败());
        assert!(检查结果::警告("x".to_string()).是否警告());
    }
}
