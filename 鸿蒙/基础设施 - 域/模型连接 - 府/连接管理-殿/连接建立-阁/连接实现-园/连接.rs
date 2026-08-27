//! 连接殿 - 错误 + 请求/响应 + 模型连接 trait + HTTP 实现 + LLM 调用器
//!
//! 决策锚：260826-2240 传承殿启动 § 阶段 7
//! 关联文档：02-概念/可插拔/01-可插拔.md + 04-设计/01-架构总览.md § 任务执行-府
//! falsifiable：模型连接 trait + 真实 HTTP POST（ureq）

use crate::模型池_殿::{LLM池, LLM配置};
use crate::模型消息_殿::{消息, 角色};

// ============================================================================
// 错误
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum 错误 {
    HTTP错误 { 状态码: u16, 原因: String },
    解析错误(String),
    配置错误(String),
    超时,
    鉴权失败,
}

impl std::fmt::Display for 错误 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            错误::HTTP错误 { 状态码, 原因 } => write!(f, "HTTP 错误 {}: {}", 状态码, 原因),
            错误::解析错误(msg) => write!(f, "解析错误：{}", msg),
            错误::配置错误(msg) => write!(f, "配置错误：{}", msg),
            错误::超时 => write!(f, "请求超时"),
            错误::鉴权失败 => write!(f, "鉴权失败（API key 无效或缺失）"),
        }
    }
}

impl std::error::Error for 错误 {}

// ============================================================================
// 请求 / 响应
// ============================================================================

#[derive(Clone, Debug)]
pub struct 请求 {
    pub 模型: String,
    pub 消息列表: Vec<消息>,
    pub 温度: f32,
    pub 最大token: u32,
}

impl 请求 {
    pub fn 新建(模型: impl Into<String>, 消息列表: Vec<消息>) -> Self {
        Self {
            模型: 模型.into(),
            消息列表,
            温度: 0.7,
            最大token: 2048,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 响应 {
    pub 内容: String,
    pub 用量_输入tokens: u32,
    pub 用量_输出tokens: u32,
}

impl 响应 {
    pub fn 假响应(内容: impl Into<String>) -> Self {
        Self {
            内容: 内容.into(),
            用量_输入tokens: 0,
            用量_输出tokens: 0,
        }
    }
}

// ============================================================================
// 模型连接 trait
// ============================================================================

pub trait 模型连接: Send + Sync {
    /// 发送请求，返回响应
    fn 发送(&self, 配置: &LLM配置, 请求: &请求) -> Result<响应, 错误>;
}

// ============================================================================
// HTTP 实现（ureq 同步客户端）
// ============================================================================

/// OpenAI 兼容 HTTP 实现（适配 OpenAI / DeepSeek / MiniMax 等）
pub struct HTTP连接 {
    pub user_agent: String,
}

impl HTTP连接 {
    pub fn 新建() -> Self {
        Self {
            user_agent: "chuanchengdian-moxing_fu/0.1.0".to_string(),
        }
    }
}

impl 模型连接 for HTTP连接 {
    fn 发送(&self, 配置: &LLM配置, 请求: &请求) -> Result<响应, 错误> {
        // 构造 OpenAI 兼容请求体
        let 消息列表: Vec<serde_json::Value> = 请求
            .消息列表
            .iter()
            .map(|m| {
                serde_json::json!({
                    "role": match m.角色 {
                        角色::系统 => "system",
                        角色::用户 => "user",
                        角色::助手 => "assistant",
                        角色::工具 => "tool",
                    },
                    "content": m.内容
                })
            })
            .collect();

        let 请求体 = serde_json::json!({
            "model": 请求.模型,
            "messages": 消息列表,
            "temperature": 请求.温度,
            "max_tokens": 请求.最大token,
        });

        // ureq POST
        let 代理 = ureq::AgentBuilder::new()
            .timeout_read(std::time::Duration::from_millis(配置.超时毫秒 as u64))
            .timeout_write(std::time::Duration::from_millis(配置.超时毫秒 as u64))
            .build();
        let 请求体_str = serde_json::to_string(&请求体)
            .map_err(|e| 错误::解析错误(format!("序列化失败：{}", e)))?;
        let 响应 = 代理
            .post(&配置.端点)
            .set("Authorization", &format!("Bearer {}", 配置.API密钥))
            .set("Content-Type", "application/json")
            .set("User-Agent", &self.user_agent)
            .send_string(&请求体_str)
            .map_err(|e| match e {
                ureq::Error::Status(code, resp) => 错误::HTTP错误 {
                    状态码: code,
                    原因: resp.into_string().unwrap_or_default(),
                },
                ureq::Error::Transport(t) => {
                    if matches!(t.kind(), ureq::ErrorKind::Io)
                        && t.to_string().contains("timed out")
                    {
                        错误::超时
                    } else {
                        错误::HTTP错误 {
                            状态码: 0,
                            原因: t.to_string(),
                        }
                    }
                }
            })?;

        let 响应体 = 响应
            .into_string()
            .map_err(|e| 错误::解析错误(e.to_string()))?;
        let json: serde_json::Value = serde_json::from_str(&响应体)
            .map_err(|e| 错误::解析错误(format!("JSON 解析失败：{}", e)))?;

        // 解析 OpenAI 兼容响应
        let 内容 = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| 错误::解析错误("缺 choices[0].message.content".to_string()))?
            .to_string();
        let 用量_输入 = json["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32;
        let 用量_输出 = json["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32;

        Ok(响应 {
            内容,
            用量_输入tokens: 用量_输入,
            用量_输出tokens: 用量_输出,
        })
    }
}

// ============================================================================
// 任务执行集成：4 分类 LLM 池 + 发送
// ============================================================================

/// 4 分类 LLM 调用器（按池名 → 配置 → 发送）
pub struct LLM调用器<C: 模型连接> {
    pub 池: LLM池,
    pub 连接: C,
}

impl<C: 模型连接> LLM调用器<C> {
    pub fn 新建(池: LLM池, 连接: C) -> Self {
        Self { 池, 连接 }
    }

    /// 按池名调用对应 LLM（无配置则返回错误）
    pub fn 调用(&self, 池名: &str, 请求: &请求) -> Result<响应, 错误> {
        let 配置 = self
            .池
            .取(池名)
            .ok_or_else(|| 错误::配置错误(format!("池 {} 未配置", 池名)))?;
        // 若模型名为空，使用池配置的默认模型
        let 实际请求 = if 请求.模型.is_empty() {
            let mut r = 请求.clone();
            r.模型 = 配置.模型.clone();
            r
        } else {
            请求.clone()
        };
        self.连接.发送(配置, &实际请求)
    }
}
