/// 配置条目：单条键值对记录
pub struct 配置条目 {
    pub 键: String,
    pub 值: String,
}

/// 错误模式：解析过程中可能出现的错误形态
pub enum 错误模式 {
    格式错误(String),
    缺少键(String),
    非法值(String),
}
