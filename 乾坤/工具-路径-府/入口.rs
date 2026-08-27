//! 工具-路径-府
//!
//! 决策锚：260827-AI助手自给自足（Round 11.5）
//! 6 层结构：2 殿（接口-殿 + 实现-殿）× 2 阁 × 2 园

#![allow(non_snake_case)]

#[path = "路径-实现-殿/模块.rs"]
pub mod 路径_实现_殿;
#[path = "路径-接口-殿/模块.rs"]
pub mod 路径_接口_殿;

pub use 路径_实现_殿::拼接_方法_阁::拼接实现_园::拼接实现;
pub use 路径_实现_殿::解析_方法_阁::路径解析实现_园::解析实现;
pub use 路径_接口_殿::拼接_阁::拼接契约_园::拼接;
pub use 路径_接口_殿::路径解析_阁::解析契约_园::路径解析;

/// 拼接两个路径
pub fn 拼接(a: &str, b: &str) -> String {
    if a.ends_with('/') || a.ends_with('\\') {
        format!("{}{}", a, b)
    } else {
        format!("{}/{}", a, b)
    }
}

/// 解析路径为目录与文件名
pub fn 解析(p: &str) -> (String, String) {
    match p.rfind('/') {
        Some(i) => (p[..i].to_string(), p[i + 1..].to_string()),
        None => (String::new(), p.to_string()),
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 拼接_正常() {
        assert_eq!(拼接("a/b", "c"), "a/b/c");
    }
    #[test]
    fn 拼接_带斜杠() {
        assert_eq!(拼接("a/b/", "c"), "a/b/c");
    }
    #[test]
    fn 解析_目录与文件() {
        let (目录, 文件) = 解析("a/b/c.txt");
        assert_eq!(目录, "a/b");
        assert_eq!(文件, "c.txt");
    }
}
