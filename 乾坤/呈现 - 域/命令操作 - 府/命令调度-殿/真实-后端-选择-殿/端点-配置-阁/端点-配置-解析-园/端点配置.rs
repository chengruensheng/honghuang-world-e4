//! 端点-配置-解析-园 - 端点配置读取（LLM_BASE_URL / LLM_TIMEOUT_MS / LLM_MODEL）
//!
//! 决策锚：260827-moxing_fu调用方集成（Round 9）
//! 关联文档：22-moxing_fu调用方集成-实施方案.md § 4.3 端点配置
//! falsifiable：读 LLM_BASE_URL / LLM_TIMEOUT_MS / LLM_MODEL 环境变量，含默认值

#[cfg(test)]
use std::sync::{Mutex, OnceLock};

/// 端点配置（前端选择后用于显示与日志）
///
/// 真实连接由 moxing_fu::从环境变量构造() 内部处理；本结构仅承载端点元数据。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 端点配置 {
    pub 端点: String,
    pub 超时毫秒: u32,
    pub 模型: String,
}

impl 端点配置 {
    /// 默认配置（与 moxing_fu::从环境变量构造() 默认值一致）
    pub fn 默认() -> Self {
        Self {
            端点: "https://api.openai.com/v1/chat/completions".to_string(),
            超时毫秒: 30000,
            模型: "gpt-3.5-turbo".to_string(),
        }
    }
}

/// env var 测试串行锁
#[cfg(test)]
fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

/// 从环境变量读端点配置（仅供真实模式显示/日志用）
pub fn 读端点配置() -> 端点配置 {
    use std::env;
    let 端点 = env::var("LLM_BASE_URL")
        .unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string());
    let 超时毫秒 = env::var("LLM_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(30000);
    let 模型 = env::var("LLM_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());
    端点配置 {
        端点,
        超时毫秒,
        模型,
    }
}

/// 显式构造端点配置（测试入口）
pub fn 构造端点配置(
    env_base: Option<&str>,
    env_timeout: Option<&str>,
    env_model: Option<&str>,
) -> 端点配置 {
    let 端点 = env_base
        .filter(|s| !s.is_empty())
        .unwrap_or("https://api.openai.com/v1/chat/completions")
        .to_string();
    let 超时毫秒 = env_timeout
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(30000);
    let 模型 = env_model
        .filter(|s| !s.is_empty())
        .unwrap_or("gpt-3.5-turbo")
        .to_string();
    端点配置 {
        端点,
        超时毫秒,
        模型,
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 测试_默认端点配置() {
        let c = 端点配置::默认();
        assert_eq!(c.端点, "https://api.openai.com/v1/chat/completions");
        assert_eq!(c.超时毫秒, 30000);
        assert_eq!(c.模型, "gpt-3.5-turbo");
    }

    #[test]
    fn 测试_构造_全默认() {
        let c = 构造端点配置(None, None, None);
        assert_eq!(c, 端点配置::默认());
    }

    #[test]
    fn 测试_构造_自定义base() {
        let c = 构造端点配置(
            Some("https://api.deepseek.com/v1/chat/completions"),
            None,
            None,
        );
        assert_eq!(c.端点, "https://api.deepseek.com/v1/chat/completions");
        assert_eq!(c.超时毫秒, 30000);
        assert_eq!(c.模型, "gpt-3.5-turbo");
    }

    #[test]
    fn 测试_构造_自定义超时() {
        let c = 构造端点配置(None, Some("60000"), None);
        assert_eq!(c.超时毫秒, 60000);
    }

    #[test]
    fn 测试_构造_无效超时_走默认() {
        let c = 构造端点配置(None, Some("abc"), None);
        assert_eq!(c.超时毫秒, 30000);
    }

    #[test]
    fn 测试_构造_自定义模型() {
        let c = 构造端点配置(None, None, Some("deepseek-coder"));
        assert_eq!(c.模型, "deepseek-coder");
    }

    #[test]
    fn 测试_构造_空字符串_走默认() {
        let c = 构造端点配置(Some(""), Some(""), Some(""));
        assert_eq!(c, 端点配置::默认());
    }

    #[test]
    fn 测试_读端点配置_无env() {
        let _g = env_lock();
        std::env::remove_var("LLM_BASE_URL");
        std::env::remove_var("LLM_TIMEOUT_MS");
        std::env::remove_var("LLM_MODEL");
        let c = 读端点配置();
        assert_eq!(c, 端点配置::默认());
    }

    #[test]
    fn 测试_读端点配置_自定义env() {
        let _g = env_lock();
        std::env::set_var(
            "LLM_BASE_URL",
            "https://api.deepseek.com/v1/chat/completions",
        );
        std::env::set_var("LLM_TIMEOUT_MS", "45000");
        std::env::set_var("LLM_MODEL", "deepseek-chat");
        let c = 读端点配置();
        assert_eq!(c.端点, "https://api.deepseek.com/v1/chat/completions");
        assert_eq!(c.超时毫秒, 45000);
        assert_eq!(c.模型, "deepseek-chat");
        std::env::remove_var("LLM_BASE_URL");
        std::env::remove_var("LLM_TIMEOUT_MS");
        std::env::remove_var("LLM_MODEL");
    }
}
