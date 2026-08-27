//! 压缩实现 - RLE 压缩

use crate::压缩_接口_殿::压缩_阁::压缩契约_园::压缩;

pub struct 压缩实现;

impl 压缩 for 压缩实现 {
    fn 压缩(&self, 数据: &[u8]) -> Vec<u8> {
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
}
