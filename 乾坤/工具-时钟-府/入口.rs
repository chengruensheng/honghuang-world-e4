//! 工具-时钟-府 - 计时/秒表
//!
//! 决策锚：260827-AI助手自给自足（Round 11.5）
//! 6 层结构：2 殿（接口-殿 + 实现-殿）× 2 阁 × 2 园

#![allow(non_snake_case)]

#[path = "时钟-实现-殿/模块.rs"]
pub mod 时钟_实现_殿;
#[path = "时钟-接口-殿/模块.rs"]
pub mod 时钟_接口_殿;

pub use 时钟_实现_殿::秒表_阁::秒表_实现_园::秒表;
pub use 时钟_实现_殿::转换_阁::转换_实现_园::转换;
pub use 时钟_接口_殿::单位_阁::单位_契约_园::单位;
pub use 时钟_接口_殿::计时_阁::计时_契约_园::计时;

/// 计时一个闭包，返回耗时
pub fn 计时<F>(f: F) -> std::time::Duration
where
    F: FnOnce(),
{
    let 开始 = std::time::Instant::now();
    f();
    开始.elapsed()
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 计时_返回耗时() {
        let d = 计时(|| std::thread::sleep(std::time::Duration::from_millis(10)));
        assert!(d.as_millis() >= 10);
    }
    #[test]
    fn 秒表_计时() {
        let 表 = 秒表::新建();
        let d = 表.计时(|| std::thread::sleep(std::time::Duration::from_millis(5)));
        assert!(d.as_millis() >= 5);
    }
    #[test]
    fn 转换_毫秒() {
        let d = std::time::Duration::from_millis(1500);
        assert_eq!(转换(d, 单位::毫秒), 1500.0);
    }
}
