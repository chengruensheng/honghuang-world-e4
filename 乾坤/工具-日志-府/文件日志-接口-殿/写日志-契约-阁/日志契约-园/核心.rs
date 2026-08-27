pub trait 写日志契约 {
    fn 格式化(&self, 时间: &str, 级别: &str, 消息: &str) -> String;
}
