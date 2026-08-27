pub struct 轮转策略 {
    pub 最大字节: u64,
    pub 备份后缀: String,
}

pub fn 新建(最大字节: u64) -> 轮转策略 {
    轮转策略 {
        最大字节,
        备份后缀: ".old".to_string(),
    }
}

pub fn 备份路径(策略: &轮转策略, 原路径: &str) -> String {
    format!("{}{}", 原路径, 策略.备份后缀)
}
