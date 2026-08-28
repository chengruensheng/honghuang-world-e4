//! 自检结果-园 - 聚合所有门禁检查的结构化结果
//!
//! 决策锚：260827-AI助手自给自足（Round 11）
//! 关联文档：14-命名唯一性门禁 + 16-统一六层风格 + 23-CI集成13项门禁
//! falsifiable：聚合 13 项门禁 + 单次输出通过/失败状态

use std::path::Path;
use std::process::Command;

#[derive(Clone, Debug)]
pub struct 自检项 {
    pub 编号: u8,
    pub 名称: String,
    pub 状态: 状态,
    pub 详情: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 状态 {
    通过,
    警告,
    失败,
}

#[derive(Clone, Debug)]
pub struct 自检报告 {
    pub 项目根: String,
    pub 总计: u32,
    pub 通过: u32,
    pub 失败: u32,
    pub 警告: u32,
    pub 项: Vec<自检项>,
}

impl 自检报告 {
    pub fn 通过(&self) -> bool {
        self.失败 == 0
    }
    pub fn 摘要(&self) -> String {
        format!(
            "[自检] 总={} 通过={} 失败={} 警告={} {}",
            self.总计,
            self.通过,
            self.失败,
            self.警告,
            if self.通过() {
                "✓ 全部通过"
            } else {
                "✗ 有失败"
            }
        )
    }
}

/// 单项检查：执行 shell 命令并捕获退出码
fn 检查命令(编号: u8, 名称: &str, cmd: &str, args: &[&str], cwd: &Path) -> 自检项 {
    let 输出 = Command::new(cmd).args(args).current_dir(cwd).output();
    match 输出 {
        Ok(o) => {
            let 退出码 = o.status.code().unwrap_or(-1);
            let stdout = String::from_utf8_lossy(&o.stdout);
            let 详情: String = stdout
                .lines()
                .rev()
                .take(3)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("\n");
            let 详情 = 详情.chars().take(200).collect::<String>();
            let 状态 = if 退出码 == 0 {
                状态::通过
            } else {
                状态::失败
            };
            自检项 {
                编号,
                名称: 名称.to_string(),
                状态,
                详情,
            }
        }
        Err(e) => 自检项 {
            编号,
            名称: 名称.to_string(),
            状态: 状态::失败,
            详情: format!("无法执行：{}", e),
        },
    }
}

/// 13 项一键全验门禁（精简版，单次 cargo test 复用编译）
pub fn 跑全检(项目根: &Path) -> 自检报告 {
    let 项: Vec<自检项> = vec![
        检查命令(
            1,
            "格式 (cargo fmt --check)",
            "cargo",
            &["fmt", "--all", "--", "--check"],
            项目根,
        ),
        检查命令(
            2,
            "编译 (cargo check)",
            "cargo",
            &["check", "--workspace"],
            项目根,
        ),
        检查命令(
            3,
            "单元测试 (cargo test)",
            "cargo",
            &["test", "--workspace", "--lib", "--", "--test-threads=1"],
            项目根,
        ),
        检查命令(
            4,
            "静态分析 (cargo clippy)",
            "cargo",
            &[
                "clippy",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
            项目根,
        ),
        检查命令(
            5,
            "架构校验 15项 (jianyan_gongju)",
            "cargo",
            &["test", "-p", "jianyan_gongju", "--lib"],
            项目根,
        ),
        检查命令(
            6,
            "决策契约 9文件",
            "powershell.exe",
            &[
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-File",
                "E:/洪荒 - 世界/道果树/质量门禁 - 域/门禁 - 府/验证-决策契约.ps1",
            ],
            项目根,
        ),
        检查命令(
            7,
            "命名唯一性",
            "powershell.exe",
            &[
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-File",
                "E:/洪荒 - 世界/道果树/质量门禁 - 域/门禁 - 府/验证-命名唯一性.ps1",
            ],
            项目根,
        ),
        检查命令(
            8,
            "无空目录",
            "powershell.exe",
            &[
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-File",
                "E:/洪荒 - 世界/道果树/质量门禁 - 域/门禁 - 府/验证-无空目录.ps1",
            ],
            项目根,
        ),
        检查命令(
            9,
            "无临时目录残留",
            "powershell.exe",
            &[
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-File",
                "E:/洪荒 - 世界/道果树/质量门禁 - 域/门禁 - 府/验证-临时目录.ps1",
            ],
            项目根,
        ),
        检查命令(
            10,
            "无 src/ 平铺",
            "powershell.exe",
            &[
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-File",
                "E:/洪荒 - 世界/道果树/质量门禁 - 域/门禁 - 府/验证-无src目录.ps1",
            ],
            项目根,
        ),
        检查命令(
            11,
            "依赖审查 (cargo deny)",
            "cargo",
            &["deny", "check", "bans", "licenses", "sources"],
            项目根,
        ),
        检查命令(
            12,
            "安全审计 (cargo audit)",
            "cargo",
            &["audit", "--no-fetch"],
            项目根,
        ),
        检查命令(
            13,
            "文档 (cargo doc)",
            "cargo",
            &["doc", "--no-deps"],
            项目根,
        ),
        // 注意：不能经 cargo run 重建自检入口.exe（自检进程正占用该 exe，Windows 无法覆盖）——
        // 直接调用已编译示例，避免重建
        检查命令(
            14,
            "任务收尾三件套 (临时库)",
            "E:/洪荒 - 世界/道果树/构建物 - 域/debug/examples/自检入口.exe",
            &[
                "记忆",
                "收尾",
                "自检任务",
                "自检三件套通过",
                "5",
                "C:/Users/17628/AppData/Local/Temp/自检收尾临时库.sq3",
            ],
            项目根,
        ),
        // 一键全验双版本引用完整性：防止并行线新增门禁后未同步聚合脚本而悄然丢失
        检查命令(
            15,
            "一键全验 15 项引用完整 (.sh/.ps1)",
            "E:/洪荒 - 世界/道果树/质量门禁 - 域/门禁 - 府/验证-一键全验完整性.ps1",
            &[],
            项目根,
        ),
    ];
    let 通过 = 项.iter().filter(|x| x.状态 == 状态::通过).count() as u32;
    let 失败 = 项.iter().filter(|x| x.状态 == 状态::失败).count() as u32;
    let 警告 = 项.iter().filter(|x| x.状态 == 状态::警告).count() as u32;
    自检报告 {
        项目根: 项目根.display().to_string(),
        总计: 项.len() as u32,
        通过,
        失败,
        警告,
        项,
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 测试_通过判定() {
        let r = 自检报告 {
            项目根: "x".into(),
            总计: 10,
            通过: 10,
            失败: 0,
            警告: 0,
            项: vec![],
        };
        assert!(r.通过());
    }
    #[test]
    fn 测试_失败判定() {
        let r = 自检报告 {
            项目根: "x".into(),
            总计: 10,
            通过: 9,
            失败: 1,
            警告: 0,
            项: vec![],
        };
        assert!(!r.通过());
    }
    #[test]
    fn 测试_摘要格式() {
        let r = 自检报告 {
            项目根: "x".into(),
            总计: 5,
            通过: 5,
            失败: 0,
            警告: 0,
            项: vec![],
        };
        let s = r.摘要();
        assert!(s.contains("总=5"));
        assert!(s.contains("✓"));
    }
}
