//! 线性实现 - LCG 随机数

use crate::随机_接口_殿::随机_阁::随机契约_园::随机;

pub struct 线性随机;

impl 随机 for 线性随机 {
    fn 随机数(&self, 种子: u64, 范围: u64) -> u64 {
        if 范围 == 0 {
            return 0;
        }
        let 状态 = 种子
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        状态 % 范围
    }
}
