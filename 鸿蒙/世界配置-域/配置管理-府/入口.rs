//! 配置管理 - 府
//!
//! 密钥/模型配置注入：环境变量优先，配置文件兜底。
//! 阶段 1 仅承载配置 trait 与简单 KV 读取。
//!
//! 决策锚：260826-2230 工程-DSH § DSH 万物皆插件
//! 关联文档：DSH 架构 system-prompt + scope

use std::collections::HashMap;
use std::env;

/// 配置读取特征
pub trait 配置源: Send + Sync {
    fn 取(&self, 键: &str) -> Option<String>;
}

/// 环境变量配置源
pub struct 环境变量配置;

impl 配置源 for 环境变量配置 {
    fn 取(&self, 键: &str) -> Option<String> {
        env::var(键).ok()
    }
}

/// 内存配置源（用于测试）
#[derive(Default)]
pub struct 内存配置 {
    数据: HashMap<String, String>,
}

impl 内存配置 {
    pub fn 新建() -> Self {
        Self::default()
    }
    pub fn 置(&mut self, 键: impl Into<String>, 值: impl Into<String>) {
        self.数据.insert(键.into(), 值.into());
    }
}

impl 配置源 for 内存配置 {
    fn 取(&self, 键: &str) -> Option<String> {
        self.数据.get(键).cloned()
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 内存配置读写一致() {
        let mut c = 内存配置::新建();
        c.置("模型", "MiniMax-M3");
        assert_eq!(c.取("模型").as_deref(), Some("MiniMax-M3"));
        assert_eq!(c.取("缺失"), None);
    }
}
