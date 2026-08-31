//! 准圣阁 - 准圣级角色卡
//!
//! 决策锚：260826-2210 4-分类抽象 § 流水线
//! 准圣：六维验收 / 打回

use crate::角色_卡册_殿::{角色分类, 角色卡};

/// 准圣卡构造（验收 / 打回）
pub fn 准圣卡() -> 角色卡 {
    角色卡 {
        分类: 角色分类::准圣级,
        名称: "准圣卡".to_string(),
        权限: vec!["验收".to_string(), "打回".to_string()],
        decided_by: "界主".to_string(),
        LLM池: "验收专用".to_string(),
        工具偏好: vec!["六维验收".to_string(), "falsifiable 校验".to_string()],
    }
}
