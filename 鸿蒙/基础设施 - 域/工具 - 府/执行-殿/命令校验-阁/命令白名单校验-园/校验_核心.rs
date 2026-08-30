//! 命令校验-阁 - 命令白名单校验
//!
//! 治理铁律：执行命令工具只允许 cargo 构建/测试类 + 一键全验，
//! 拒绝任意 shell（rm/del/网络命令），并拒绝命令分隔符注入。

/// 校验命令安全：仅允许 cargo build/test/fmt/clippy/check + 一键全验，拒绝分隔符注入。
pub fn 校验_命令(命令: &str) -> Result<(), String> {
    let 命令 = 命令.trim();
    if 命令.is_empty() {
        return Err("命令为空".to_string());
    }
    // 命令白名单：只允许构建/测试/格式化/静态检查/一键全验
    let 允许前缀 = [
        "cargo build",
        "cargo test",
        "cargo fmt",
        "cargo clippy",
        "cargo check",
        "一键全验",
    ];
    let 命中 = 允许前缀.iter().any(|前缀| 命令.starts_with(前缀));
    if !命中 {
        return Err(format!("命令不在白名单：{}", 命令));
    }
    // 分隔符注入检查：拒绝追加恶意命令
    if 命令.contains("&&")
        || 命令.contains("||")
        || 命令.contains(';')
        || 命令.contains('|')
        || 命令.contains('&')
    {
        return Err("命令含分隔符，拒绝（防注入）".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 白名单命令放行() {
        assert!(校验_命令("cargo build").is_ok());
        assert!(校验_命令("cargo test --workspace --lib").is_ok());
        assert!(校验_命令("cargo fmt --check").is_ok());
        assert!(校验_命令("cargo clippy -- -D warnings").is_ok());
        assert!(校验_命令("一键全验").is_ok());
    }

    #[test]
    fn 危险命令拒绝() {
        assert!(校验_命令("rm -rf .").is_err());
        assert!(校验_命令("del /f /s").is_err());
        assert!(校验_命令("curl http://x").is_err());
        assert!(校验_命令("").is_err());
    }

    #[test]
    fn 注入命令拒绝() {
        assert!(校验_命令("cargo build && rm -rf .").is_err());
        assert!(校验_命令("cargo test; echo hacked").is_err());
    }
}
