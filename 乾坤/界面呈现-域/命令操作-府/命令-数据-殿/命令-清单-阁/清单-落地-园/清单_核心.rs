//! 命令清单 - 可用命令列表

pub const 命令清单: &[&str] = &["init", "status", "run", "e2e", "help"];

pub fn 命令清单_vec() -> Vec<&'static str> {
    命令清单.to_vec()
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 清单至少4项() {
        assert!(命令清单.len() >= 4);
    }
}
