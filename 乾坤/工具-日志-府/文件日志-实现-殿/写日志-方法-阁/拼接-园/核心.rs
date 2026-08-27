use crate::文件日志_接口_殿::写日志_契约_阁::日志契约_园::核心::写日志契约;

pub struct 拼接实现;

impl 写日志契约 for 拼接实现 {
    fn 格式化(&self, 时间: &str, 级别: &str, 消息: &str) -> String {
        format!("[{}] [{}] {}\n", 时间, 级别, 消息)
    }
}

pub fn 格式化(时间: &str, 级别: &str, 消息: &str) -> String {
    拼接实现.格式化(时间, 级别, 消息)
}
