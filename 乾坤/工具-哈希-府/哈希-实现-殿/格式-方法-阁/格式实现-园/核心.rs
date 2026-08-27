//! 格式实现 - 十六进制格式化

use crate::哈希_接口_殿::格式_阁::格式契约_园::格式;

pub struct 格式实现;

impl 格式 for 格式实现 {
    fn 十六进制(&self, 值: u64) -> String {
        format!("{:016x}", 值)
    }
}
