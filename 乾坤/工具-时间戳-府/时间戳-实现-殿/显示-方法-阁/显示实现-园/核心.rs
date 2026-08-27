//! 显示实现 - 时间戳显示

use crate::时间戳_接口_殿::显示_阁::显示契约_园::显示;

pub struct 显示实现;

impl 显示 for 显示实现 {
    fn 显示(&self, 时间戳: u64) -> String {
        format!("{}s since epoch", 时间戳)
    }
}
