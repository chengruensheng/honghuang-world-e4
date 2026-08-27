//! 匹配实现 - 通配符匹配

use crate::正则_接口_殿::匹配_阁::匹配契约_园::匹配;

pub struct 匹配实现;

impl 匹配 for 匹配实现 {
    fn 匹配(&self, 模式: &str, 文本: &str) -> bool {
        let 模式: Vec<char> = 模式.chars().collect();
        let 文本: Vec<char> = 文本.chars().collect();
        let (mut i, mut j) = (0, 0);
        let (mut 星, mut 匹配) = (usize::MAX, 0);
        while j < 文本.len() {
            if i < 模式.len() && (模式[i] == 文本[j] || 模式[i] == '?') {
                i += 1;
                j += 1;
            } else if i < 模式.len() && 模式[i] == '*' {
                星 = i;
                匹配 = j;
                i += 1;
            } else if 星 != usize::MAX {
                i = 星 + 1;
                匹配 += 1;
                j = 匹配;
            } else {
                return false;
            }
        }
        while i < 模式.len() && 模式[i] == '*' {
            i += 1;
        }
        i == 模式.len()
    }
}
