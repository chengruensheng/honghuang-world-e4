//! 级别枚举 - 日志级别定义

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 级别 {
    跟踪,
    调试,
    信息,
    警告,
    错误,
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 级别变体数匹配() {
        let 所有 = [级别::跟踪, 级别::调试, 级别::信息, 级别::警告, 级别::错误];
        assert_eq!(所有.len(), 5);
    }
}
