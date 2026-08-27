//! 转换实现 - 时间单位转换

use crate::时钟_接口_殿::单位_阁::单位_契约_园::单位;

pub fn 转换(d: std::time::Duration, 单位: 单位) -> f64 {
    match 单位 {
        单位::纳秒 => d.as_nanos() as f64,
        单位::毫秒 => d.as_millis() as f64,
        单位::秒 => d.as_secs_f64(),
    }
}
