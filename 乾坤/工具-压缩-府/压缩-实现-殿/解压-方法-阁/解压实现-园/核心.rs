//! 解压实现 - RLE 解压

use crate::压缩_接口_殿::解压_阁::解压契约_园::解压;

pub struct 解压实现;

impl 解压 for 解压实现 {
    fn 解压(&self, 数据: &[u8]) -> Vec<u8> {
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
}
