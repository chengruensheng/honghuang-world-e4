//! 工具-正则-府
//!
//! 决策锚：260827-AI助手自给自足（Round 11.5）
//! 6 层结构：2 殿（接口-殿 + 实现-殿）× 2 阁 × 2 园

#![allow(non_snake_case)]

#[path = "正则-实现-殿/模块.rs"]
pub mod 正则_实现_殿;
#[path = "正则-接口-殿/模块.rs"]
pub mod 正则_接口_殿;

pub use 正则_实现_殿::匹配_方法_阁::匹配实现_园::匹配实现;
pub use 正则_实现_殿::提取_方法_阁::提取实现_园::提取实现;
pub use 正则_接口_殿::匹配_阁::匹配契约_园::匹配;
pub use 正则_接口_殿::提取_阁::提取契约_园::提取;

/// 通配符匹配（* 匹配任意，? 匹配单字符）
pub fn 匹配(模式: &str, 文本: &str) -> bool {
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

/// 提取第一个匹配（简单实现：返回匹配到的子串）
pub fn 提取(模式: &str, 文本: &str) -> Option<String> {
    if 匹配(模式, 文本) {
        Some(文本.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 匹配_星号() {
        assert!(匹配("a*c", "abc"));
        assert!(!匹配("a*c", "abd"));
    }
    #[test]
    fn 匹配_问号() {
        assert!(匹配("a?c", "abc"));
        assert!(!匹配("a?c", "ab"));
    }
    #[test]
    fn 提取_匹配返回() {
        assert_eq!(提取("a*c", "abc"), Some("abc".to_string()));
        assert_eq!(提取("a*c", "abd"), None);
    }
}
