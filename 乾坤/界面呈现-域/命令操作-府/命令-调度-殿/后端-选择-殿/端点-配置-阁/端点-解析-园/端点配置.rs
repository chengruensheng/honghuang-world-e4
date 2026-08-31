//! 端点-解析-园 - 端点配置读取（LLM_BASE_URL / LLM_TIMEOUT_MS / LLM_MODEL）
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
    /// 打回重试上限（契约「可循环打回≤3 轮」的上界；生产默认 3：对齐契约上界，打回重投给真实 LLM 三次重投机会）
    pub 打回上限: usize,
}

impl 端点配置 {
    /// 默认配置（与 moxing_fu::从环境变量构造() 默认值一致）
    pub fn 默认() -> Self {
        Self {
            端点: "https://api.openai.com/v1/chat/completions".to_string(),
            超时毫秒: 30000,
            模型: "gpt-3.5-turbo".to_string(),
            打回上限: 3,
        }
    }
}

/// env var 测试串行锁
#[cfg(test)]
fn 环境锁() -> std::sync::MutexGuard<'static, ()> {
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
    let 打回上限 = env::var("LLM_打回上限")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(3);
    端点配置 {
        端点,
        超时毫秒,
        模型,
        打回上限,
    }
}

/// 从环境变量读终裁温度（道祖终裁采样温度，默认 0.3；0.1 过严、0.7 非确定，折中 0.3）
///
/// 非法值回退默认 0.3（不 fail loud：温度非安全关键，静默回退不影响正确性）
pub fn 读终裁温度() -> f32 {
    std::env::var("LLM_终裁温度")
        .ok()
        .and_then(|s| s.parse::<f32>().ok())
        .filter(|t| (0.0..=1.0).contains(t))
        .unwrap_or(0.3)
}

/// 从环境变量读终裁采样次数（道祖终裁多次采样取多数，默认 1 生产不增成本；实验设 3 对冲采样非确定）
///
/// 非法值/零/负回退 1（不 fail loud：采样次数非安全关键）。
pub fn 读终裁采样次数() -> usize {
    std::env::var("LLM_终裁采样次数")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|n| *n >= 1)
        .unwrap_or(1)
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
        打回上限: 3,
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
        let _g = 环境锁();
        std::env::remove_var("LLM_BASE_URL");
        std::env::remove_var("LLM_TIMEOUT_MS");
        std::env::remove_var("LLM_MODEL");
        let c = 读端点配置();
        assert_eq!(c, 端点配置::默认());
    }

    #[test]
    fn 测试_读端点配置_自定义env() {
        let _g = 环境锁();
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

    #[test]
    fn 测试_读终裁温度_无env走默认() {
        let _g = 环境锁();
        std::env::remove_var("LLM_终裁温度");
        assert_eq!(读终裁温度(), 0.3);
    }

    #[test]
    fn 测试_读终裁温度_自定义() {
        let _g = 环境锁();
        std::env::set_var("LLM_终裁温度", "0.1");
        assert_eq!(读终裁温度(), 0.1);
        std::env::remove_var("LLM_终裁温度");
    }

    #[test]
    fn 测试_读终裁温度_非法值回退默认() {
        let _g = 环境锁();
        // 非法：非数值 + 超范围（>1.0）均回退 0.3
        std::env::set_var("LLM_终裁温度", "abc");
        assert_eq!(读终裁温度(), 0.3);
        std::env::set_var("LLM_终裁温度", "2.5");
        assert_eq!(读终裁温度(), 0.3);
        std::env::remove_var("LLM_终裁温度");
    }
}
