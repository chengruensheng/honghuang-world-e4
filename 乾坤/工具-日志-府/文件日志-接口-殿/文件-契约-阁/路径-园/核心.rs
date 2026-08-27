pub struct 日志路径 {
    pub 路径: String,
}

pub fn 新建(路径: &str) -> 日志路径 {
    日志路径 {
        路径: 路径.to_string(),
    }
}

pub fn 取(p: &日志路径) -> &str {
    &p.路径
}

pub fn 取字符串(p: &日志路径) -> String {
    p.路径.clone()
}
