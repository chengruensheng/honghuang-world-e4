/// 两数求和：返回两个整数的和，负数与零均正确
pub fn 两数求和(甲: i64, 乙: i64) -> i64 {
    甲 + 乙
}

#[cfg(test)]
mod 测试 {
    use super::两数求和;
    #[test]
    fn 测试_两数求和() {
        assert_eq!(两数求和(2, 3), 5);
        assert_eq!(两数求和(0, 0), 0);
        assert_eq!(两数求和(-2, -3), -5);
        assert_eq!(两数求和(-5, 10), 5);
        assert_eq!(两数求和(i64::MIN, 0), i64::MIN);
        assert_eq!(两数求和(i64::MAX, 0), i64::MAX);
    }
}
