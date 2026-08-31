//! 通过率计算 - 测试统计的通过率方法

use crate::测试_统计_殿::单元_结构_阁::测试_结构_园::测试统计;

impl 测试统计 {
    pub fn 通过率(&self) -> f64 {
        let 总 = self.通过 + self.失败;
        if 总 == 0 {
            0.0
        } else {
            self.通过 as f64 / 总 as f64
        }
    }
}

#[cfg(test)]
mod 测试 {
    use crate::测试统计;
    #[test]
    fn 通过率计算() {
        let s = 测试统计::新建("示例", 8, 2);
        assert!((s.通过率() - 0.8).abs() < 1e-9);
    }
    #[test]
    fn 零分母返回零() {
        let s = 测试统计::新建("空", 0, 0);
        assert_eq!(s.通过率(), 0.0);
    }
}
