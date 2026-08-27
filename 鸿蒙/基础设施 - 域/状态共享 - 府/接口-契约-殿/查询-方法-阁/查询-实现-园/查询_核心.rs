//! 查询操作 - 状态共享读取与版本查询

use crate::状态_存储_殿::容器_定义_阁::容器_园::状态共享;
use crate::状态_存储_殿::数据_定义_阁::数据_园::状态值;

impl 状态共享 {
    /// 按键读取当前值
    pub fn 读(&self, 键: &str) -> Option<状态值> {
        let inner = self.inner.锁.read().expect("状态共享锁中毒");
        inner.数据.get(键).cloned()
    }

    /// 当前版本号
    pub fn 版本(&self) -> u64 {
        self.inner.锁.read().expect("状态共享锁中毒").版本
    }
}

#[cfg(test)]
mod 测试 {
    use crate::状态共享;
    #[test]
    fn 读写一致() {
        let s = 状态共享::新建();
        状态共享::写(&s, "道", "洪荒");
        let got = s.读("道").unwrap();
        assert_eq!(got.值, "洪荒");
    }
}
