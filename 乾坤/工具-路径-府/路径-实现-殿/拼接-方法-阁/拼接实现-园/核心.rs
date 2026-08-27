//! 拼接实现 - 路径拼接

use crate::路径_接口_殿::拼接_阁::拼接契约_园::拼接;

pub struct 拼接实现;

impl 拼接 for 拼接实现 {
    fn 拼接(&self, a: &str, b: &str) -> String {
        if a.ends_with('/') || a.ends_with('\\') {
            format!("{}{}", a, b)
        } else {
            format!("{}/{}", a, b)
        }
    }
}
