//! 池殿 - 4 分类 LLM 池 + LLM 配置
//!
//! 决策锚：260826-2240 传承殿启动 § 阶段 7
//! 关联文档：02-概念/角色分类/04-角色分类.md § 4 分类 LLM 池配置

use crate::连接_管理_殿::错误;

// ============================================================================
// 配置（4 分类 LLM 池）
// ============================================================================

#[derive(Clone, Debug)]
#[allow(non_snake_case)]
pub struct LLM配置 {
    pub 端点: String,
    pub 模型: String,
    pub API密钥: String,
    pub 超时毫秒: u32,
}

impl LLM配置 {
    pub fn 假配置(模型: impl Into<String>) -> Self {
        Self {
            端点: "http://127.0.0.1:0/mock".to_string(),
            模型: 模型.into(),
            API密钥: "mock-key".to_string(),
            超时毫秒: 5000,
        }
    }
}

/// 4 分类 LLM 池（按角色卡.LLM池 字段选择）
///
/// 决策锚：02-概念/角色分类/04-角色分类.md § 4 分类 LLM 池配置
#[derive(Clone, Debug, Default)]
pub struct LLM池 {
    pub 道祖池: Option<LLM配置>,
    pub 圣人池: Option<LLM配置>,
    pub 准圣池: Option<LLM配置>,
    pub 大罗池: Option<LLM配置>,
}

impl LLM池 {
    pub fn 新建() -> Self {
        Self::default()
    }

    pub fn 设(&mut self, 池名: &str, 配置: LLM配置) -> Result<(), 错误> {
        match 池名 {
            "道祖" | "化要求" => self.道祖池 = Some(配置),
            "圣人" | "设计" => self.圣人池 = Some(配置),
            "准圣" | "验收" => self.准圣池 = Some(配置),
            "大罗" | "实现" => self.大罗池 = Some(配置),
            _ => return Err(错误::配置错误(format!("未知 LLM 池：{}", 池名))),
        }
        Ok(())
    }

    pub fn 取(&self, 池名: &str) -> Option<&LLM配置> {
        match 池名 {
            "道祖" | "化要求" => self.道祖池.as_ref(),
            "圣人" | "设计" => self.圣人池.as_ref(),
            "准圣" | "验收" => self.准圣池.as_ref(),
            "大罗" | "实现" => self.大罗池.as_ref(),
            _ => None,
        }
    }
}
