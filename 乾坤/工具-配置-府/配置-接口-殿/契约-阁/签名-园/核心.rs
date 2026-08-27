use std::collections::HashMap;

/// 解析契约：所有配置解析实现必须遵循的接口签名
pub trait 解析契约 {
    /// 将文本内容解析为键值对映射
    fn 解析(&self, 内容: &str) -> Result<HashMap<String, String>, String>;
}
