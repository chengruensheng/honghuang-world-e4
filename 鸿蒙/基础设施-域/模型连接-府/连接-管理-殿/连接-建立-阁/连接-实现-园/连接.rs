//! 连接殿 - 错误 + 请求/响应 + 模型连接 trait + HTTP 实现 + LLM 调用器
//!
//! 决策锚：260826-2240 传承殿启动 § 阶段 7
//! 关联文档：02-概念/可插拔/01-可插拔.md + 04-设计/01-架构总览.md § 任务执行-府
//! falsifiable：模型连接 trait + 真实 HTTP POST（ureq）

use crate::模型_池库_殿::{LLM池, LLM配置};
use crate::模型_消息_殿::{消息, 角色};

// ============================================================================
// 错误
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum 错误 {
    HTTP错误 { 状态码: u16, 原因: String },
    额度耗尽,
    解析错误(String),
    配置错误(String),
    超时,
    鉴权失败,
}

impl std::fmt::Display for 错误 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            错误::HTTP错误 { 状态码, 原因 } => write!(f, "HTTP 错误 {}: {}", 状态码, 原因),
            错误::额度耗尽 => write!(f, "额度耗尽（Token Plan 用量上限，请充值或购买积分）"),
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
    /// 是否流式（stream:true，SSE 逐 delta 返回）；false 走一次性完整响应
    pub 流式: bool,
}

impl 请求 {
    pub fn 新建(模型: impl Into<String>, 消息列表: Vec<消息>) -> Self {
        Self {
            模型: 模型.into(),
            消息列表,
            温度: 0.7,
            最大token: 2048,
            流式: false,
        }
    }

    /// 设最大token（builder 风格）：执行层产出代码需更大输出窗口，默认 2048 会被截断
    pub fn 设最大token(mut self, 最大token: u32) -> Self {
        self.最大token = 最大token;
        self
    }

    /// 设温度（builder 风格）：终裁需确定性裁决（低温 0.1），执行/设计可保留默认 0.7
    pub fn 设温度(mut self, 温度: f32) -> Self {
        self.温度 = 温度;
        self
    }

    /// 设流式（builder 风格）：true 走 SSE 逐 delta 返回（token 级流式展示），默认 false
    pub fn 设流式(mut self, 流式: bool) -> Self {
        self.流式 = 流式;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 响应 {
    pub 内容: String,
    pub 思考链: Option<String>,
    pub 用量_输入tokens: u32,
    pub 用量_输出tokens: u32,
}

impl 响应 {
    pub fn 假响应(内容: impl Into<String>) -> Self {
        Self {
            内容: 内容.into(),
            思考链: None,
            用量_输入tokens: 0,
            用量_输出tokens: 0,
        }
    }
}

/// 剥离 <think>...</think> 思考链标签，返回（正文, 思考链）。
///
/// 架构重塑：思考链与正文在响应层分离，下游只消费正文，不解析字符串打补丁。
pub fn 剥离思考链(原始: &str) -> (String, Option<String>) {
    let 起始 = 原始.find("<think>");
    let 结束 = 原始.find("</think>");
    match (起始, 结束) {
        (Some(s), Some(e)) if s < e => {
            let 思考链 = 原始[s + "<think>".len()..e].trim().to_string();
            let mut 正文 = String::with_capacity(原始.len());
            正文.push_str(&原始[..s]);
            正文.push_str(&原始[e + "</think>".len()..]);
            (正文.trim().to_string(), Some(思考链))
        }
        _ => (原始.trim().to_string(), None),
    }
}

// ============================================================================
// 模型连接 trait
// ============================================================================

pub trait 模型连接: Send + Sync {
    /// 发送请求，返回响应
    fn 发送(&self, 配置: &LLM配置, 请求: &请求) -> Result<响应, 错误>;

    /// 流式发送：边接收增量正文边回调（token 级流式展示）。
    ///
    /// 默认实现回退到非流式 `发送`，把完整正文一次性回调——保证仅实现 `发送` 的连接
    /// （Mock / 故障注入等）零改动仍可编译。真正支持 SSE 的连接（HTTP连接）应覆写本方法。
    fn 发送_流式(
        &self,
        配置: &LLM配置,
        请求: &请求,
        回调: &mut dyn FnMut(&str),
    ) -> Result<响应, 错误> {
        let 响应 = self.发送(配置, 请求)?;
        if !响应.内容.is_empty() {
            回调(&响应.内容);
        }
        Ok(响应)
    }
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

impl HTTP连接 {
    /// 构造 OpenAI 兼容请求体字符串（stream 由参数显式控制：流式接口强制 true）
    fn 构造请求体(&self, 请求: &请求, stream: bool) -> Result<String, 错误> {
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
            "stream": stream,
        });
        serde_json::to_string(&请求体)
            .map_err(|e| 错误::解析错误(format!("序列化失败：{}", e)))
    }

    /// 发 POST 并做错误映射（连接/读/写同超时窗口；402→额度耗尽；超时→超时）
    fn 发送请求(
        &self, 配置: &LLM配置, 请求体_str: &str
    ) -> Result<ureq::Response, 错误> {
        let 代理 = ureq::AgentBuilder::new()
            .timeout_connect(std::time::Duration::from_millis(配置.超时毫秒 as u64))
            .timeout_read(std::time::Duration::from_millis(配置.超时毫秒 as u64))
            .timeout_write(std::time::Duration::from_millis(配置.超时毫秒 as u64))
            .build();
        代理
            .post(&配置.端点)
            .set("Authorization", &format!("Bearer {}", 配置.API密钥))
            .set("Content-Type", "application/json")
            .set("User-Agent", &self.user_agent)
            .send_string(请求体_str)
            .map_err(|e| match e {
                ureq::Error::Status(402, _resp) => 错误::额度耗尽,
                ureq::Error::Status(code, resp) => 错误::HTTP错误 {
                    状态码: code,
                    原因: resp.into_string().unwrap_or_default(),
                },
                ureq::Error::Transport(t) => {
                    // 连接超时识别：ureq「timed out」+ Windows 连接超时 os error 10060/10061（真实 MiniMax 网络抖动常见）
                    let 文本 = t.to_string();
                    if matches!(t.kind(), ureq::ErrorKind::Io)
                        && (文本.contains("timed out")
                            || 文本.contains("10060")
                            || 文本.contains("10061"))
                    {
                        错误::超时
                    } else {
                        错误::HTTP错误 {
                            状态码: 0,
                            原因: t.to_string(),
                        }
                    }
                }
            })
    }
}

impl 模型连接 for HTTP连接 {
    fn 发送(&self, 配置: &LLM配置, 请求: &请求) -> Result<响应, 错误> {
        let 请求体_str = self.构造请求体(请求, 请求.流式)?;
        let 响应 = self.发送请求(配置, &请求体_str)?;

        let 响应体 = 响应
            .into_string()
            .map_err(|e| 错误::解析错误(e.to_string()))?;
        let json: serde_json::Value = serde_json::from_str(&响应体)
            .map_err(|e| 错误::解析错误(format!("JSON 解析失败：{}", e)))?;

        // 解析 OpenAI 兼容响应：思考链（reasoning_content / <think> 标签）与正文分离
        let 思考链_字段 = json["choices"][0]["message"]["reasoning_content"]
            .as_str()
            .map(|s| s.to_string());
        let 内容_原始 = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| 错误::解析错误("缺 choices[0].message.content".to_string()))?
            .to_string();
        let (内容, 思考链_标签) = 剥离思考链(&内容_原始);
        let 思考链 = 思考链_字段.or(思考链_标签);
        let 用量_输入 = json["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32;
        let 用量_输出 = json["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32;

        Ok(响应 {
            内容,
            思考链,
            用量_输入tokens: 用量_输入,
            用量_输出tokens: 用量_输出,
        })
    }

    /// 流式发送（SSE）：stream:true + 逐行解析 `data:`，边收增量正文边回调，
    /// 最终把累积的完整正文/思考链打包为 `响应` 返回（下游流水线逻辑不变）。
    fn 发送_流式(
        &self,
        配置: &LLM配置,
        请求: &请求,
        回调: &mut dyn FnMut(&str),
    ) -> Result<响应, 错误> {
        // 流式接口强制 stream:true（忽略请求.流式 字段，语义由方法决定）
        let 请求体_str = self.构造请求体(请求, true)?;
        let 响应对象 = self.发送请求(配置, &请求体_str)?;

        let reader = 响应对象.into_reader();
        let mut buf = std::io::BufReader::new(reader);
        let (正文, 思考链, 用量_输入, 用量_输出) = 解析_流式响应(&mut buf, 回调)?;

        Ok(响应 {
            内容: 正文,
            思考链,
            用量_输入tokens: 用量_输入,
            用量_输出tokens: 用量_输出,
        })
    }
}

/// 解析 OpenAI 兼容 SSE 流式响应：逐行读 `data:` 负载，提取增量正文（delta.content）
/// 与思考链（delta.reasoning_content），边读边回调增量正文；末尾 usage 用于统计。
///
/// 兼容两种 chunk 形态：标准流式 `choices[0].delta.content` 与个别实现的 `message.content`。
fn 解析_流式响应<R: std::io::BufRead>(
    reader: &mut R,
    回调: &mut dyn FnMut(&str),
) -> Result<(String, Option<String>, u32, u32), 错误> {
    let mut 正文 = String::new();
    let mut 思考链 = String::new();
    let mut 用量_输入: u32 = 0;
    let mut 用量_输出: u32 = 0;
    let mut 行 = String::new();
    loop {
        行.clear();
        let n = reader
            .read_line(&mut 行)
            .map_err(|e| 错误::解析错误(format!("读取流式响应失败：{}", e)))?;
        if n == 0 {
            break; // EOF
        }
        let 行_trim = 行.trim();
        if 行_trim.is_empty() {
            continue;
        }
        // 只处理 data: 前缀行；忽略 event:/id:/空行等 SSE 控制行
        let Some(负载) = 行_trim.strip_prefix("data:") else {
            continue;
        };
        let 负载 = 负载.trim();
        if 负载 == "[DONE]" {
            break;
        }
        if 负载.is_empty() {
            continue;
        }
        let Ok(json) = serde_json::from_str::<serde_json::Value>(负载) else {
            continue; // 非法 JSON 行静默跳过（容错，不中断流）
        };
        // 增量正文：优先 delta.content，兼容 message.content
        let 增量 = json["choices"][0]["delta"]["content"]
            .as_str()
            .or_else(|| json["choices"][0]["message"]["content"].as_str());
        if let Some(片段) = 增量 {
            if !片段.is_empty() {
                正文.push_str(片段);
                回调(片段);
            }
        }
        // 思考链增量（MiniMax 等思考模型的 reasoning_content）
        if let Some(推理) = json["choices"][0]["delta"]["reasoning_content"].as_str() {
            思考链.push_str(推理);
        }
        // usage 统计（通常出现在最后一块）
        if let Some(v) = json["usage"]["prompt_tokens"].as_u64() {
            用量_输入 = v as u32;
        }
        if let Some(v) = json["usage"]["completion_tokens"].as_u64() {
            用量_输出 = v as u32;
        }
    }
    let 思考链_结果 = if 思考链.is_empty() {
        None
    } else {
        Some(思考链)
    };
    Ok((正文, 思考链_结果, 用量_输入, 用量_输出))
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

    /// 流式调用：边接收增量正文边回调（token 级流式展示），最终仍返回完整响应
    pub fn 调用_流式(
        &self,
        池名: &str,
        请求: &请求,
        回调: &mut dyn FnMut(&str),
    ) -> Result<响应, 错误> {
        let 配置 = self
            .池
            .取(池名)
            .ok_or_else(|| 错误::配置错误(format!("池 {} 未配置", 池名)))?;
        let 实际请求 = if 请求.模型.is_empty() {
            let mut r = 请求.clone();
            r.模型 = 配置.模型.clone();
            r
        } else {
            请求.clone()
        };
        self.连接.发送_流式(配置, &实际请求, 回调)
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 剥离思考链_含think标签() {
        let (正文, 思考链) = 剥离思考链("前置<think>推理过程</think>后置");
        assert_eq!(正文, "前置后置");
        assert_eq!(思考链.as_deref(), Some("推理过程"));
    }

    #[test]
    fn 剥离思考链_无标签原样返回() {
        let (正文, 思考链) = 剥离思考链("纯正文");
        assert_eq!(正文, "纯正文");
        assert_eq!(思考链, None);
    }

    #[test]
    fn 剥离思考链_只有think() {
        let (正文, 思考链) = 剥离思考链("<think>仅思考</think>");
        assert_eq!(正文, "");
        assert_eq!(思考链.as_deref(), Some("仅思考"));
    }

    // ---------- SSE 流式解析 ----------

    /// 构造一段标准 OpenAI 兼容 SSE 流，交给 解析_流式响应，断言正文累积 + 逐段回调 + usage
    #[test]
    fn 解析流式_标准delta累积与回调() {
        let 流 = "data: {\"choices\":[{\"delta\":{\"content\":\"你\"}}]}\n\
                  \n\
                  data: {\"choices\":[{\"delta\":{\"content\":\"好\"}}]}\n\
                  \n\
                  data: {\"choices\":[{\"delta\":{},\"finish_reason\":\"stop\"}],\"usage\":{\"prompt_tokens\":11,\"completion_tokens\":2}}\n\
                  \n\
                  data: [DONE]\n";
        let mut 回调片段 = Vec::new();
        let mut 回调 = |片段: &str| 回调片段.push(片段.to_string());
        let (正文, 思考链, 入, 出) = 解析_流式响应(&mut 流.as_bytes(), &mut 回调).unwrap();
        assert_eq!(正文, "你好");
        assert_eq!(思考链, None);
        assert_eq!(回调片段, vec!["你".to_string(), "好".to_string()]);
        assert_eq!(入, 11);
        assert_eq!(出, 2);
    }

    /// 思考链模型（MiniMax）delta 带 reasoning_content，应与正文分离
    #[test]
    fn 解析流式_思考链分离() {
        let 流 = "data: {\"choices\":[{\"delta\":{\"reasoning_content\":\"推理中\"}}]}\n\
                  \ndata: {\"choices\":[{\"delta\":{\"content\":\"答案\"}}]}\n\
                  \ndata: [DONE]\n";
        let mut 回调 = |_: &str| {};
        let (正文, 思考链, _, _) = 解析_流式响应(&mut 流.as_bytes(), &mut 回调).unwrap();
        assert_eq!(正文, "答案");
        assert_eq!(思考链.as_deref(), Some("推理中"));
    }

    /// 兼容个别实现的 message.content（非 delta）与非法 JSON 行容错
    #[test]
    fn 解析流式_兼容message与非法行容错() {
        let 流 = "data: {\"choices\":[{\"message\":{\"content\":\"A\"}}]}\n\
                  \ndata: 这不是合法JSON\n\
                  \ndata: {\"choices\":[{\"delta\":{\"content\":\"B\"}}]}\n\
                  \ndata: [DONE]\n";
        let mut 回调 = |_: &str| {};
        let (正文, _, _, _) = 解析_流式响应(&mut 流.as_bytes(), &mut 回调).unwrap();
        assert_eq!(正文, "AB");
    }

    /// 空流（无任何 data 行）→ 正文为空，不报错
    #[test]
    fn 解析流式_空流返回空正文() {
        let mut 回调 = |_: &str| {};
        let (正文, 思考链, 入, 出) = 解析_流式响应(&mut "".as_bytes(), &mut 回调).unwrap();
        assert_eq!(正文, "");
        assert_eq!(思考链, None);
        assert_eq!(入, 0);
        assert_eq!(出, 0);
    }
}
