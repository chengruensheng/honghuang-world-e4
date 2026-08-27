//! 内存读取 - 内存配置读取

use crate::内存_配置_实现_殿::内存_结构_阁::内存容器_园::内存配置;
use crate::配置源_接口_殿::配置_契约_阁::接口契约_园::配置源;

impl 配置源 for 内存配置 {
    fn 取(&self, 键: &str) -> Option<String> {
        self.数据.get(键).cloned()
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;
    use crate::内存配置;
    #[test]
    fn 内存配置读写一致() {
        let mut c = 内存配置::新建();
        c.置("模型", "MiniMax-M3");
        assert_eq!(c.取("模型").as_deref(), Some("MiniMax-M3"));
        assert_eq!(c.取("缺失"), None);
    }
}
