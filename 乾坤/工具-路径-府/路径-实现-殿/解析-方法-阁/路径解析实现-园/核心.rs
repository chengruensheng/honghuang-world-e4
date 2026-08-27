//! 解析实现 - 路径解析

use crate::路径_接口_殿::路径解析_阁::解析契约_园::路径解析;

pub struct 解析实现;

impl 路径解析 for 解析实现 {
    fn 解析(&self, p: &str) -> (String, String) {
        match p.rfind('/') {
            Some(i) => (p[..i].to_string(), p[i + 1..].to_string()),
            None => (String::new(), p.to_string()),
        }
    }
}
