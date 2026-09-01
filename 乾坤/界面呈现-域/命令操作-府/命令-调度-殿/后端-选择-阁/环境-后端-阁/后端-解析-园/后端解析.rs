//! 后端-解析-园 - 后端模式枚举 + LLM_BACKEND 解析
//!
//! 决策锚：260827-moxing_fu调用方集成（Round 9）
//! 关联文档：22-moxing_fu调用方集成-实施方案.md § 四、行为契约
//! falsifiable：枚举 3 态（真实/Mock/默认）+ 解析 LLM_BACKEND 环境变量

use std::sync::{Mutex, OnceLock};

/// 后端模式
///
/// - `真实`：使用 moxing_fu::从环境变量构造() 构造的真实池 + HTTP连接
/// - `Mock`：使用 mingling_caozuo_fu 的 MockLLM连接
/// - `默认`：未设置 LLM_BACKEND 或解析失败 → 真实（严禁 mock，界主硬规则）
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 后端模式 {
    真实,
    Mock,
    默认,
}

/// env var 测试串行锁（cargo test 默认并行会污染全局 env）
fn 环境锁() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

/// 解析 LLM_BACKEND 环境变量
///
/// - "real" → 后端模式::真实
/// - "mock" → 后端模式::Mock
/// - "" / 未设置 / 其他拼写 → 后端模式::默认（实际走真实，严禁 mock）
pub fn 解析后端模式() -> 后端模式 {
    use std::env;
    match env::var("LLM_BACKEND") {
        Ok(v) if v == "real" => 后端模式::真实,
        Ok(v) if v == "mock" => 后端模式::Mock,
        _ => 后端模式::默认,
    }
}

/// 显式选择后端模式（测试入口）
///
/// 与解析后端模式() 行为一致，但接受显式参数（用于单测串行化）。
pub fn 选择后端(env_value: Option<&str>) -> 后端模式 {
    match env_value {
        Some("real") => 后端模式::真实,
        Some("mock") => 后端模式::Mock,
        _ => 后端模式::默认,
    }
}

/// 测试：env var 串行锁
pub fn 测试用锁() -> std::sync::MutexGuard<'static, ()> {
    环境锁()
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 测试_解析_real() {
        let _g = 环境锁();
        std::env::set_var("LLM_BACKEND", "real");
        assert_eq!(解析后端模式(), 后端模式::真实);
        std::env::remove_var("LLM_BACKEND");
    }

    #[test]
    fn 测试_解析_mock() {
        let _g = 环境锁();
        std::env::set_var("LLM_BACKEND", "mock");
        assert_eq!(解析后端模式(), 后端模式::Mock);
        std::env::remove_var("LLM_BACKEND");
    }

    #[test]
    fn 测试_解析_空字符串_默认() {
        let _g = 环境锁();
        std::env::set_var("LLM_BACKEND", "");
        assert_eq!(解析后端模式(), 后端模式::默认);
        std::env::remove_var("LLM_BACKEND");
    }

    #[test]
    fn 测试_解析_未设置_默认() {
        let _g = 环境锁();
        std::env::remove_var("LLM_BACKEND");
        assert_eq!(解析后端模式(), 后端模式::默认);
    }

    #[test]
    fn 测试_解析_拼写错误_默认() {
        let _g = 环境锁();
        std::env::set_var("LLM_BACKEND", "Real");
        assert_eq!(解析后端模式(), 后端模式::默认);
        std::env::remove_var("LLM_BACKEND");
    }

    #[test]
    fn 测试_选择_显式real() {
        assert_eq!(选择后端(Some("real")), 后端模式::真实);
    }

    #[test]
    fn 测试_选择_显式mock() {
        assert_eq!(选择后端(Some("mock")), 后端模式::Mock);
    }

    #[test]
    fn 测试_选择_none_默认() {
        assert_eq!(选择后端(None), 后端模式::默认);
    }

    #[test]
    fn 测试_选择_空字符串_默认() {
        assert_eq!(选择后端(Some("")), 后端模式::默认);
    }

    #[test]
    fn 测试_选择_拼写错误_默认() {
        assert_eq!(选择后端(Some("REAL")), 后端模式::默认);
    }
}
