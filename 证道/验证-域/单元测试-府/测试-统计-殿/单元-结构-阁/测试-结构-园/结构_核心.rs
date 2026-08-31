//! 统计结构 - 标准测试统计数据结构

pub struct 测试统计 {
    pub 模块: String,
    pub 通过: u32,
    pub 失败: u32,
}

impl 测试统计 {
    pub fn 新建(模块: impl Into<String>, 通过: u32, 失败: u32) -> Self {
        Self {
            模块: 模块.into(),
            通过,
            失败,
        }
    }
}
