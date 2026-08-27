//! 计时契约 - 计时特征

/// 计时特征
pub trait 计时 {
    fn 计时<F>(&self, f: F) -> std::time::Duration
    where
        F: FnOnce();
}
