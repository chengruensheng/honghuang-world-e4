//! 内存容器 - 内存配置数据结构 + 新建/置

use std::collections::HashMap;

/// 内存配置源（用于测试）
#[derive(Default)]
pub struct 内存配置 {
    pub 数据: HashMap<String, String>,
}

impl 内存配置 {
    pub fn 新建() -> Self {
        Self::default()
    }
    pub fn 置(&mut self, 键: impl Into<String>, 值: impl Into<String>) {
        self.数据.insert(键.into(), 值.into());
    }
}
