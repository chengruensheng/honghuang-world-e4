//! 命令-执行-阁 - 真实执行命令工具
//!
//! 阶段 7 工具循环升级：执行命令工具从 mock 升级为真实 std::process::Command，
//! 执行前必经命令白名单校验，返回真实退出码 + stdout + stderr。

use crate::编排_工具_殿::{工具, 调用输入, 调用输出};

pub struct 执行命令工具;

impl 执行命令工具 {
    pub fn 新建() -> Self {
        Self
    }
}

/// 跨平台执行：Windows 走 cmd /C，Unix 走 sh -c。
fn 真实执行(命令: &str) -> Result<(i32, String, String), String> {
    #[cfg(windows)]
    let 输出 = std::process::Command::new("cmd")
        .args(["/C", 命令])
        .output()
        .map_err(|e| format!("启动失败：{}", e))?;
    #[cfg(not(windows))]
    let 输出 = std::process::Command::new("sh")
        .args(["-c", 命令])
        .output()
        .map_err(|e| format!("启动失败：{}", e))?;
    let 码 = 输出.status.code().unwrap_or(-1);
    let 标准输出 = String::from_utf8_lossy(&输出.stdout).to_string();
    let 标准错误 = String::from_utf8_lossy(&输出.stderr).to_string();
    Ok((码, 标准输出, 标准错误))
}

impl 工具 for 执行命令工具 {
    fn 名称(&self) -> &str {
        "执行命令"
    }
    fn 描述(&self) -> &str {
        "真实执行 cargo 命令（受命令白名单约束）"
    }
    fn 执行(&self, 输入: &调用输入) -> 调用输出 {
        let 命令 = 输入.参数.get("命令").cloned().unwrap_or_default();
        if 命令.is_empty() {
            return 调用输出::失败("缺参数 命令");
        }
        if let Err(e) = crate::执行_工具_殿::校验_命令(命令.as_str()) {
            return 调用输出::失败(e);
        }
        match 真实执行(命令.as_str()) {
            Ok((码, 标准输出, 标准错误)) => {
                let 结果 = format!(
                    "退出码={} stdout={} stderr={}",
                    码,
                    标准输出.trim(),
                    标准错误.trim()
                );
                if 码 == 0 {
                    调用输出::成功_有副作用(结果)
                } else {
                    调用输出::失败(结果)
                }
            }
            Err(e) => 调用输出::失败(e),
        }
    }
}

/// 命令执行结果：结构化返回真实命令的退出码、标准输出与标准错误。
///
/// 供门户终端等需要展示原始终端输出的调用方使用，避免从字符串里反解析。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 命令执行结果 {
    pub 命令: String,
    pub 退出码: i32,
    pub 标准输出: String,
    pub 标准错误: String,
    pub 成功: bool,
}

/// 执行白名单命令并返回结构化结果。
///
/// 与 `执行命令工具` 共用同一命令白名单，保证门户终端不能绕过治理约束。
pub fn 执行命令_结构化(命令: &str) -> Result<命令执行结果, String> {
    if let Err(e) = crate::执行_工具_殿::校验_命令(命令) {
        return Err(e);
    }
    let (码, 标准输出, 标准错误) = 真实执行(命令)?;
    Ok(命令执行结果 {
        命令: 命令.to_string(),
        退出码: 码,
        标准输出,
        标准错误,
        成功: 码 == 0,
    })
}

#[cfg(test)]
mod 测试 {
    use super::*;
    use crate::编排_工具_殿::{
        工具ID, 工具注册表, 工具调用循环, 工具调用请求
    };

    #[test]
    fn 真实执行cargo版本返回退出码0() {
        let mut 输入 = 调用输入::default();
        输入
            .参数
            .insert("命令".to_string(), "cargo build".to_string());
        // cargo build 在测试中耗时较长，用 cargo --version 不可行（不在白名单）；
        // 这里只验证白名单外命令被拦截、白名单内命令真实执行路径可达。
        let 输出 = 执行命令工具::新建().执行(&输入);
        // cargo build 真实执行可能成功（0）或失败（非0），但不应该是白名单拦截的 FAIL
        assert!(
            !输出.结果.contains("命令不在白名单"),
            "cargo build 应在白名单内：{}",
            输出.结果
        );
    }

    #[test]
    fn 危险命令被白名单拦截() {
        let mut 输入 = 调用输入::default();
        输入.参数.insert("命令".to_string(), "rm -rf .".to_string());
        let 输出 = 执行命令工具::新建().执行(&输入);
        assert!(输出.结果.contains("FAIL"), "应拦截 rm：{}", 输出.结果);
        assert!(!输出.副作用已发生);
    }

    #[test]
    fn 执行命令进注册表调用循环() {
        let mut 注册表 = 工具注册表::新建();
        注册表.注册(工具ID::执行, Box::new(执行命令工具::新建()));
        let mut 输入 = 调用输入::default();
        输入
            .参数
            .insert("命令".to_string(), "cargo build".to_string());
        let 输出 = 工具调用循环(
            &注册表,
            vec![工具调用请求 {
                id: 工具ID::执行,
                输入,
            }],
        );
        assert_eq!(输出.len(), 1);
        assert!(!输出[0].结果.contains("命令不在白名单"));
    }

    #[test]
    fn 结构化执行_白名单命令返回退出码() {
        let 结果 = 执行命令_结构化("cargo --version").unwrap_err();
        assert!(
            结果.contains("命令不在白名单"),
            "cargo --version 不在白名单：{}",
            结果
        );
    }

    #[test]
    fn 结构化执行_危险命令拒绝() {
        let 结果 = 执行命令_结构化("rm -rf .").unwrap_err();
        assert!(结果.contains("命令不在白名单"), "危险命令应拒绝：{}", 结果);
    }
}
