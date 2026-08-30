//! 读取校验-阁 - 读路径白名单校验
//!
//! 治理铁律：读取同样受白名单约束，不泄露治理资产（.env 密钥等）。

/// 校验读取路径安全：拒绝 .. 逃逸 + 治理资产黑名单。
pub fn 校验_读取路径(路径: &str) -> Result<(), String> {
    if 路径.is_empty() {
        return Err("路径为空".to_string());
    }
    if 路径.split(['/', '\\']).any(|段| 段 == "..") {
        return Err("路径含 .. 逃逸，拒绝读取".to_string());
    }
    let 黑名单 = [".env", ".git", "构建物", "target"];
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
        assert!(校验_读取路径("乾坤/入口.rs").is_ok());
    }

    #[test]
    fn 密钥资产拒绝读取() {
        assert!(校验_读取路径(".env").is_err());
        assert!(校验_读取路径(".git/config").is_err());
    }

    #[test]
    fn 逃逸拒绝() {
        assert!(校验_读取路径("../.env").is_err());
    }
}
