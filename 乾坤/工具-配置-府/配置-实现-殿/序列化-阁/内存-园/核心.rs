use std::collections::HashMap;

/// 将内存中的 HashMap 配置序列化回 TOML 风格文本
pub fn 序列化(配置: &HashMap<String, String>) -> String {
    let mut 输出 = String::new();
    for (键, 值) in 配置 {
        输出.push_str(&format!("{} = \"{}\"\n", 键, 值));
    }
    输出
}
