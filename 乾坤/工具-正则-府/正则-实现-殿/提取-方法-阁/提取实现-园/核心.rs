//! 提取实现 - 提取匹配

use crate::正则_实现_殿::匹配_方法_阁::匹配实现_园::匹配实现;
use crate::正则_接口_殿::匹配_阁::匹配契约_园::匹配;
use crate::正则_接口_殿::提取_阁::提取契约_园::提取;

pub struct 提取实现;

impl 提取 for 提取实现 {
    fn 提取(&self, 模式: &str, 文本: &str) -> Option<String> {
        if 匹配实现.匹配(模式, 文本) {
            Some(文本.to_string())
        } else {
            None
        }
    }
}
