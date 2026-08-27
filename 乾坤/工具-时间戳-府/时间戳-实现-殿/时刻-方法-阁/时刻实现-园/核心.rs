//! 时刻实现 - 时间戳实现

use crate::时间戳_接口_殿::时刻_阁::时刻契约_园::时刻;

pub struct 时刻实现;

impl 时刻 for 时刻实现 {
    fn 当前(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}
