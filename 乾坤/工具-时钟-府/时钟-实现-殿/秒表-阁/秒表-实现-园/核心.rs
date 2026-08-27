//! 秒表实现 - 计时特征实现

use crate::时钟_接口_殿::计时_阁::计时_契约_园::计时;

pub struct 秒表;

impl 秒表 {
    pub fn 新建() -> Self {
        Self
    }
}

impl 计时 for 秒表 {
    fn 计时<F>(&self, f: F) -> std::time::Duration
    where
        F: FnOnce(),
    {
        let 开始 = std::time::Instant::now();
        f();
        开始.elapsed()
    }
}
