//! 补丁实现-园 - 补丁升级策略（x.y.z -> x.y.z+1）

/// 补丁升级影响窗口（秒）
pub fn 窗口_秒() -> u32 {
    60
}

/// 补丁升级回滚方案
pub fn 回滚方案(到版本: &str) -> String {
    format!("git revert {}", 到版本)
}
