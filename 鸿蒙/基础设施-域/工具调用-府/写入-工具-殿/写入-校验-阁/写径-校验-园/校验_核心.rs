//! 写入-校验-阁 - 写路径白名单校验
//!
//! 治理铁律 1「确定程序是治理操作唯一执行者」落地：LLM 只产写入意图，
//! 本确定性函数校验路径安全后才放行真实写盘，拒绝逃逸与治理资产覆盖。

/// 校验写入路径安全：拒绝 .. 逃逸 + 治理资产黑名单。
pub fn 校验_写入路径(路径: &str) -> Result<(), String> {
    if 路径.is_empty() {
        return Err("路径为空".to_string());
    }
    // 逃逸检查：拒绝任何 .. 组件（防目录穿越）
    if 路径.split(['/', '\\']).any(|段| 段 == "..") {
        return Err("路径含 .. 逃逸，拒绝写入".to_string());
    }
    // 治理资产黑名单：保护 .env / 记忆库 / 构建物 / 版本库 / 锁文件
    let 黑名单 = [
        ".env",
        "洪荒记忆库",
        "构建物",
        ".git",
        "Cargo.lock",
        "target",
    ];
    for 名 in 黑名单 {
        if 路径.contains(名) {
            return Err(format!("路径命中治理资产黑名单：{}", 名));
        }
    }
    Ok(())
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 正常路径放行() {
        assert!(校验_写入路径("鸿蒙/工具调用-府/新文件.rs").is_ok());
    }

    #[test]
    fn 空路径拒绝() {
        assert!(校验_写入路径("").is_err());
    }

    #[test]
    fn 逃逸路径拒绝() {
        assert!(校验_写入路径("../逃逸.rs").is_err());
        assert!(校验_写入路径("鸿蒙/../../逃逸.rs").is_err());
    }

    #[test]
    fn 治理资产拒绝() {
        assert!(校验_写入路径(".env").is_err());
        assert!(校验_写入路径("洪荒记忆库.sq3").is_err());
        assert!(校验_写入路径("道果树/构建物-域/x.rs").is_err());
        assert!(校验_写入路径(".git/config").is_err());
    }
}
