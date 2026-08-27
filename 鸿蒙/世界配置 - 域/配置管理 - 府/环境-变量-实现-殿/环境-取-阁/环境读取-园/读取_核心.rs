//! 环境读取 - 环境变量配置读取

use crate::环境_变量_实现_殿::环境_结构_阁::环境容器_园::环境变量配置;
use crate::配置源_接口_殿::配置_契约_阁::接口契约_园::配置源;
use std::env;

impl 配置源 for 环境变量配置 {
    fn 取(&self, 键: &str) -> Option<String> {
        env::var(键).ok()
    }
}
