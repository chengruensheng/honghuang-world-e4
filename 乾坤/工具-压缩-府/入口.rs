//! 工具-压缩-府
//!
//! 决策锚：260827-AI助手自给自足（Round 11.5）
//! 6 层结构：2 殿（接口-殿 + 实现-殿）× 2 阁 × 2 园

#![allow(non_snake_case)]

#[path = "压缩-实现-殿/模块.rs"]
pub mod 压缩_实现_殿;
#[path = "压缩-接口-殿/模块.rs"]
pub mod 压缩_接口_殿;

pub use 压缩_实现_殿::压缩_方法_阁::压缩实现_园::压缩实现;
pub use 压缩_实现_殿::解压_方法_阁::解压实现_园::解压实现;
pub use 压缩_接口_殿::压缩_阁::压缩契约_园::压缩;
pub use 压缩_接口_殿::解压_阁::解压契约_园::解压;

/// RLE 压缩
pub fn 压缩(数据: &[u8]) -> Vec<u8> {
    let mut 结果 = Vec::new();
    let mut i = 0;
    while i < 数据.len() {
        let 字节 = 数据[i];
        let mut 计数 = 1;
        while i + 计数 < 数据.len() && 数据[i + 计数] == 字节 && 计数 < 255 {
            计数 += 1;
        }
        结果.push(字节);
        结果.push(计数 as u8);
        i += 计数;
    }
    结果
}

/// RLE 解压
pub fn 解压(数据: &[u8]) -> Vec<u8> {
    let mut 结果 = Vec::new();
    let mut i = 0;
    while i + 1 < 数据.len() {
        let 字节 = 数据[i];
        let 计数 = 数据[i + 1] as usize;
        for _ in 0..计数 {
            结果.push(字节);
        }
        i += 2;
    }
    结果
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 压缩_解压_往返() {
        let 数据 = b"aaaabbbccd";
        let 压缩后 = 压缩(数据);
        assert_eq!(解压(&压缩后), 数据);
    }
    #[test]
    fn 压缩_减少长度() {
        let 数据 = b"aaaaaaaaaa";
        let 压缩后 = 压缩(数据);
        assert!(压缩后.len() < 数据.len());
    }
    #[test]
    fn 压缩_空数据() {
        assert_eq!(压缩(b""), Vec::<u8>::new());
    }
}
