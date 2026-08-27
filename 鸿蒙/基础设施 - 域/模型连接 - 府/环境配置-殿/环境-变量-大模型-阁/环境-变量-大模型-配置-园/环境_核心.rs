//! 环境变量 LLM 配置 - 从系统环境变量读取真实云端 LLM 配置

use crate::模型池_殿::{LLM池, LLM配置};

/// 从环境变量构造 4 分类 LLM 池
/// 环境变量约定：
/// - LLM_API_KEY (必需，Bearer token)
/// - LLM_BASE_URL (默认 https://api.openai.com/v1/chat/completions)
/// - LLM_MODEL_DAOZU / LLM_MODEL_SHENGREN / LLM_MODEL_ZHUNSHENG / LLM_MODEL_DALUO (4 分类，未设置则用 LLM_MODEL)
/// - LLM_MODEL (默认 gpt-3.5-turbo)
/// - LLM_TIMEOUT_MS (默认 30000)
///
/// 返回 None 表示 LLM_API_KEY 未设置（调用方应降级到 mock）
pub fn 从环境变量构造() -> Option<LLM池> {
    use std::env;
    let 密钥 = env::var("LLM_API_KEY").ok().filter(|s| !s.is_empty())?;
    let 端点 = env::var("LLM_BASE_URL")
        .unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string());
    let 默认模型 = env::var("LLM_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());
    let 超时 = env::var("LLM_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(30000);

    let 取分类模型 = |分类: &str| -> String {
        env::var(format!("LLM_MODEL_{}", 分类.to_uppercase())).unwrap_or_else(|_| 默认模型.clone())
    };

    let 配置 = |模型: String| LLM配置 {
        端点: 端点.clone(),
        API密钥: 密钥.clone(),
        模型,
        超时毫秒: 超时,
    };

    let mut 池 = LLM池::新建();
    let _ = 池.设("道祖", 配置(取分类模型("DAOZU")));
    let _ = 池.设("圣人", 配置(取分类模型("SHENGREN")));
    let _ = 池.设("准圣", 配置(取分类模型("ZHUNSHENG")));
    let _ = 池.设("大罗", 配置(取分类模型("DALUO")));
    Some(池)
}

#[cfg(test)]
mod 测试 {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    /// env var 测试串行锁（cargo test 默认并行会污染全局 env）
    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn 测试_无环境变量返回None() {
        let _g = env_lock();
        std::env::remove_var("LLM_API_KEY");
        std::env::remove_var("LLM_BASE_URL");
        std::env::remove_var("LLM_MODEL");
        std::env::remove_var("LLM_MODEL_DAOZU");
        std::env::remove_var("LLM_MODEL_SHENGREN");
        std::env::remove_var("LLM_MODEL_ZHUNSHENG");
        std::env::remove_var("LLM_MODEL_DALUO");
        std::env::remove_var("LLM_TIMEOUT_MS");
        assert!(从环境变量构造().is_none());
    }

    #[test]
    fn 测试_空字符串API_KEY返回None() {
        let _g = env_lock();
        std::env::set_var("LLM_API_KEY", "");
        assert!(从环境变量构造().is_none());
        std::env::remove_var("LLM_API_KEY");
    }

    #[test]
    fn 测试_有效API_KEY返回Some池() {
        let _g = env_lock();
        std::env::set_var("LLM_API_KEY", "sk-test-123");
        std::env::set_var("LLM_BASE_URL", "https://api.test.com/v1/chat/completions");
        std::env::set_var("LLM_MODEL", "gpt-4");
        std::env::remove_var("LLM_MODEL_DAOZU");
        std::env::remove_var("LLM_MODEL_SHENGREN");
        std::env::remove_var("LLM_MODEL_ZHUNSHENG");
        std::env::remove_var("LLM_MODEL_DALUO");
        std::env::remove_var("LLM_TIMEOUT_MS");
        let 池 = 从环境变量构造().expect("有 API_KEY 应返回 Some");
        assert!(池.道祖池.is_some());
        assert_eq!(池.道祖池.as_ref().unwrap().模型, "gpt-4");
        assert_eq!(
            池.道祖池.as_ref().unwrap().端点,
            "https://api.test.com/v1/chat/completions"
        );
        assert_eq!(池.道祖池.as_ref().unwrap().API密钥, "sk-test-123");
        std::env::remove_var("LLM_API_KEY");
        std::env::remove_var("LLM_BASE_URL");
        std::env::remove_var("LLM_MODEL");
    }

    #[test]
    fn 测试_4分类各自模型名覆盖() {
        let _g = env_lock();
        std::env::set_var("LLM_API_KEY", "sk-test");
        std::env::set_var("LLM_MODEL", "default-model");
        std::env::set_var("LLM_MODEL_DAOZU", "claude-3-opus");
        std::env::set_var("LLM_MODEL_DALUO", "deepseek-coder");
        let 池 = 从环境变量构造().unwrap();
        assert_eq!(池.道祖池.as_ref().unwrap().模型, "claude-3-opus");
        assert_eq!(池.圣人池.as_ref().unwrap().模型, "default-model");
        assert_eq!(池.准圣池.as_ref().unwrap().模型, "default-model");
        assert_eq!(池.大罗池.as_ref().unwrap().模型, "deepseek-coder");
        std::env::remove_var("LLM_API_KEY");
        std::env::remove_var("LLM_MODEL");
        std::env::remove_var("LLM_MODEL_DAOZU");
        std::env::remove_var("LLM_MODEL_DALUO");
    }

    #[test]
    fn 测试_超时默认值() {
        let _g = env_lock();
        std::env::set_var("LLM_API_KEY", "sk-test");
        std::env::remove_var("LLM_TIMEOUT_MS");
        let 池 = 从环境变量构造().unwrap();
        assert_eq!(池.道祖池.as_ref().unwrap().超时毫秒, 30000);
        std::env::remove_var("LLM_API_KEY");
    }

    #[test]
    fn 测试_超时自定义() {
        let _g = env_lock();
        std::env::set_var("LLM_API_KEY", "sk-test");
        std::env::set_var("LLM_TIMEOUT_MS", "60000");
        let 池 = 从环境变量构造().unwrap();
        assert_eq!(池.道祖池.as_ref().unwrap().超时毫秒, 60000);
        std::env::remove_var("LLM_API_KEY");
        std::env::remove_var("LLM_TIMEOUT_MS");
    }
}
