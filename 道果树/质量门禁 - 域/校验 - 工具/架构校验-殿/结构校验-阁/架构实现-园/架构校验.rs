//! 架构校验实现 - 11 项架构校验（替代 验证-架构.ps1）
//!
//! 包含：检查结果类型 + 目录/命名/workspace/传承殿等架构校验。
//!
//! 决策锚：v4 阶段 16 架构校验 Rust 化

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

pub fn 检查_README(根: &Path) -> 检查结果 {
    if 根.join("传\\承\\殿").join("README.md").exists() || 根.join("README.md").exists() {
        检查结果::通过
    } else {
        检查结果::失败("README 不存在".to_string())
    }
}
// ============================================================================
// 命名唯一性门禁（14 号方案 · 260827-命名门禁）
// 规则 1：祖孙不同名；规则 2：同层全局唯一；规则 3：目录名无英文残留
// ============================================================================

const 层级后缀: [&str; 8] = [
    "-殿", "-阁", "-园", "-数据", "-配置", "-模板", "-脚本", "-资源",
];

fn 取层级(name: &str) -> Option<&str> {
    层级后缀.iter().copied().find(|s| name.ends_with(s))
}

fn 去层级后缀(name: &str, suffix: &str) -> String {
    name[..name.len() - suffix.len()].to_string()
}

fn 是排除目录2(p: &Path) -> bool {
    let s = p.to_string_lossy();
    [
        ".git",
        "构建物 - 域",
        "构建物-域",
        "debug",
        "doc",
        "target",
        "node_modules",
    ]
    .iter()
    .any(|e| s.contains(e))
}

/// 规则 1：同一府路径下，殿/阁/园 名（去后缀）两两不同。
pub fn 检查_祖孙不同名(根: &Path) -> 检查结果 {
    let mut 违规 = Vec::new();
    for entry in walkdir::WalkDir::new(根).max_depth(8).into_iter().flatten() {
        let p = entry.path();
        if !p.is_dir() {
            continue;
        }
        if 是排除目录2(p) {
            continue;
        }
        let name = match p.file_name().and_then(|s| s.to_str()) {
            Some(n) => n,
            None => continue,
        };
        let Some(后缀) = 取层级(name) else {
            continue;
        };
        if 后缀 != "-殿" {
            continue;
        } // 只从殿层开始检查
        let 殿名 = 去层级后缀(name, "-殿");
        // 收集该殿目录下所有阁/园名
        let mut 名集 = vec![殿名];
        let mut 子违规 = Vec::new();
        for sub in walkdir::WalkDir::new(p).max_depth(4).into_iter().flatten() {
            if !sub.path().is_dir() {
                continue;
            }
            if sub.path() == p {
                continue;
            }
            let sn = match sub.path().file_name().and_then(|s| s.to_str()) {
                Some(n) => n,
                None => continue,
            };
            let Some(ss) = 取层级(sn) else { continue };
            if ss == "-殿" {
                continue;
            } // 不跨殿（理论上殿下无殿）
            名集.push(去层级后缀(sn, ss));
        }
        // 查重复
        let mut seen = std::collections::HashMap::new();
        for n in 名集 {
            let e = seen.entry(n.clone()).or_insert(0u32);
            *e += 1;
        }
        for (n, cnt) in seen {
            if cnt > 1 {
                子违规.push(n);
            }
        }
        if !子违规.is_empty() {
            违规.push(format!("{}: 祖孙同名 {:?}", p.display(), 子违规));
        }
    }
    if 违规.is_empty() {
        检查结果::通过
    } else {
        检查结果::失败(format!("祖孙同名违规：{}", 违规.join("；")))
    }
}

/// 规则 2：全项目 `-殿`、`-阁`、`-园` 名各自唯一（跨府不重名）。
pub fn 检查_同层命名唯一(根: &Path) -> 检查结果 {
    let mut 计数: std::collections::HashMap<String, (u32, Vec<String>)> =
        std::collections::HashMap::new();
    for entry in walkdir::WalkDir::new(根).max_depth(8).into_iter().flatten() {
        let p = entry.path();
        if !p.is_dir() {
            continue;
        }
        if 是排除目录2(p) {
            continue;
        }
        let name = match p.file_name().and_then(|s| s.to_str()) {
            Some(n) => n,
            None => continue,
        };
        let Some(后缀) = 取层级(name) else {
            continue;
        };
        let 名 = 去层级后缀(name, 后缀);
        let e = 计数.entry(名.clone()).or_insert((0, Vec::new()));
        e.0 += 1;
        e.1.push(p.display().to_string());
    }
    let mut 违规 = Vec::new();
    for (名, (cnt, paths)) in 计数 {
        if cnt > 1 {
            违规.push(format!("{} 出现 {} 次：{}", 名, cnt, paths.join(" vs ")));
        }
    }
    if 违规.is_empty() {
        检查结果::通过
    } else {
        检查结果::失败(format!("同层命名重复：{}", 违规.join("；")))
    }
}

/// 规则 3：目录名（去层级后缀）不得含 ASCII 字母；白名单：SQLite、P0-P3。
pub fn 检查_目录名无英文(根: &Path) -> 检查结果 {
    let 允许前缀: [&str; 5] = ["SQLite", "P0", "P1", "P2", "P3"];
    let mut 违规 = Vec::new();
    for entry in walkdir::WalkDir::new(根).max_depth(8).into_iter().flatten() {
        let p = entry.path();
        if !p.is_dir() {
            continue;
        }
        if 是排除目录2(p) {
            continue;
        }
        let name = match p.file_name().and_then(|s| s.to_str()) {
            Some(n) => n,
            None => continue,
        };
        let 名 = match 取层级(name) {
            Some(s) => 去层级后缀(name, s),
            None => name.to_string(),
        };
        let 含英文 = 名.chars().any(|ch| ch.is_ascii_alphabetic());
        if !含英文 {
            continue;
        }
        let 白名单 = 允许前缀.iter().any(|pre| 名.starts_with(pre));
        if !白名单 {
            违规.push(format!("{}（目录 {}）", 名, p.display()));
        }
    }
    if 违规.is_empty() {
        检查结果::通过
    } else {
        检查结果::失败(format!("英文目录名：{}", 违规.join("；")))
    }
}

// ============================================================================
// 防退化门禁（Round 7+ 补录）
// ============================================================================

const 府级目录列表: [&str; 21] = [
    "鸿蒙/基础设施 - 域/插件上下文 - 府",
    "鸿蒙/基础设施 - 域/跨维事件总线 - 府",
    "鸿蒙/基础设施 - 域/记忆承载 - 府",
    "鸿蒙/基础设施 - 域/流水线驱动 - 府",
    "鸿蒙/基础设施 - 域/任务执行 - 府",
    "鸿蒙/基础设施 - 域/模型连接 - 府",
    "鸿蒙/基础设施 - 域/追问引擎 - 府",
    "鸿蒙/基础设施 - 域/状态共享 - 府",
    "鸿蒙/基础设施 - 域/观测探针 - 府",
    "鸿蒙/基础设施 - 域/日志记录 - 府",
    "鸿蒙/世界配置 - 域/配置管理 - 府",
    "乾坤/呈现 - 域/命令操作 - 府",
    "证道/鸿蒙 - 域/单元测试 - 府",
    "道韵/规则 - 域/规则 - 府",
    "道果树/质量门禁 - 域/监控 - 府",
    "道果树/质量门禁 - 域/校验 - 工具",
    "道果树/质量门禁 - 域/评估 - 府",
    "道果树/运营 - 域/实时 - 府",
    "道果树/运营 - 域/调遣 - 府",
    "道果树/运营 - 域/升级 - 府",
    "世界",
];

pub fn 检查_府级crate至少2殿(根: &Path) -> 检查结果 {
    let mut 违规 = Vec::new();
    for fu in 府级目录列表.iter() {
        let fu_path = 根.join(fu);
        if !fu_path.exists() {
            continue;
        }
        let entries = match std::fs::read_dir(&fu_path) {
            Ok(e) => e,
            Err(_) => continue,
        };
        let 殿数 = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .filter(|e| e.file_name().to_string_lossy().ends_with("-殿"))
            .count();
        if 殿数 < 2 {
            违规.push(format!("{} (殿数={})", fu, 殿数));
        }
    }
    if 违规.is_empty() {
        检查结果::通过
    } else {
        检查结果::失败(format!("府级 crate <2 殿：{}", 违规.join("; ")))
    }
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
    fn 测试_检查结果方法() {
        assert!(检查结果::通过.是否通过());
        assert!(!检查结果::通过.是否失败());
        assert!(检查结果::失败("x".to_string()).是否失败());
        assert!(检查结果::警告("x".to_string()).是否警告());
    }
}
