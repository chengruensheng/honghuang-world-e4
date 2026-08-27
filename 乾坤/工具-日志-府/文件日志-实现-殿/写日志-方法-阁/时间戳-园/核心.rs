use std::time::{SystemTime, UNIX_EPOCH};

pub fn 现在() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn 格式(秒: u64) -> String {
    let 总时 = 秒 / 3600;
    let 时 = 总时 % 24;
    let 分 = 秒 / 60 % 60;
    let 钞 = 秒 % 60;
    format!("{:02}:{:02}:{:02}", 时, 分, 钞)
}
