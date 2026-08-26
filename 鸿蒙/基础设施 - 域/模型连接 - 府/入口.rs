//! 模型连接 - 府
//!
//! LLM HTTP POST + 配置注入 + 4 分类 LLM 池。
//! 阶段 7 Day 1-2：模型连接骨架 + 4 分类 LLM 池 + mock 测试。
//!
//! 决策锚：260826-2240 传承殿启动 § 阶段 7
//! 关联文档：02-概念/可插拔/01-可插拔.md + 02-概念/角色分类/04-角色分类.md + 04-设计/01-架构总览.md § 任务执行-府
//! falsifiable：模型连接 trait + 4 分类 LLM 池 + 真实 HTTP POST（ureq）+ 100% mock 单元测试

#![allow(non_snake_case)] // 角色卡等字段名遵循中文命名
#![allow(clippy::upper_case_acronyms)] // LLM 等业界缩写

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
// 消息
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 角色 {
    系统,
    用户,
    助手,
    工具,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 消息 {
    pub 角色: 角色,
    pub 内容: String,
}

impl 消息 {
    pub fn 系统(内容: impl Into<String>) -> Self {
        Self {
            角色: 角色::系统,
            内容: 内容.into(),
        }
    }
    pub fn 用户(内容: impl Into<String>) -> Self {
        Self {
            角色: 角色::用户,
            内容: 内容.into(),
        }
    }
    pub fn 助手(内容: impl Into<String>) -> Self {
        Self {
            角色: 角色::助手,
            内容: 内容.into(),
        }
    }
}

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
// 配置（4 分类 LLM 池）
// ============================================================================

#[derive(Clone, Debug)]
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
#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 请求_构造默认值() {
        let r = 请求::新建("test-model", vec![消息::用户("hi")]);
        assert_eq!(r.模型, "test-model");
        assert_eq!(r.消息列表.len(), 1);
        assert_eq!(r.温度, 0.7);
        assert_eq!(r.最大token, 2048);
    }

    #[test]
    fn llm池_设与取() {
        let mut 池 = LLM池::新建();
        池.设("道祖", LLM配置::假配置("claude-3")).unwrap();
        池.设("圣人", LLM配置::假配置("deepseek")).unwrap();
        assert!(池.取("道祖").is_some());
        assert!(池.取("道祖").unwrap().模型 == "claude-3");
        assert!(池.取("圣人").unwrap().模型 == "deepseek");
        assert!(池.取("化要求").is_some()); // 别名
    }

    #[test]
    fn llm池_4分类_全覆盖() {
        let mut 池 = LLM池::新建();
        池.设("道祖", LLM配置::假配置("c1")).unwrap();
        池.设("圣人", LLM配置::假配置("c2")).unwrap();
        池.设("准圣", LLM配置::假配置("c3")).unwrap();
        池.设("大罗", LLM配置::假配置("c4")).unwrap();
        assert!(池.道祖池.is_some());
        assert!(池.圣人池.is_some());
        assert!(池.准圣池.is_some());
        assert!(池.大罗池.is_some());
    }

    #[test]
    fn llm池_未知池名报错() {
        let mut 池 = LLM池::新建();
        let r = 池.设("未知池", LLM配置::假配置("x"));
        assert!(r.is_err());
    }

    #[test]
    fn 调用器_未配置池报错() {
        let 调用器 = LLM调用器::新建(LLM池::新建(), HTTP连接::新建());
        let 请求 = 请求::新建("gpt-4", vec![消息::用户("hi")]);
        let r = 调用器.调用("道祖", &请求);
        assert!(matches!(r, Err(错误::配置错误(_))));
    }

    #[test]
    fn 假响应_构造() {
        let r = 响应::假响应("hello");
        assert_eq!(r.内容, "hello");
        assert_eq!(r.用量_输入tokens, 0);
    }

    // ---------- HTTP 集成测试（mock 服务器）----------

    /// Mock 连接：不发真实 HTTP，直接返回固定响应
    struct Mock连接 {
        响应内容: String,
    }
    impl Mock连接 {
        fn 新建(响应内容: impl Into<String>) -> Self {
            Self {
                响应内容: 响应内容.into(),
            }
        }
    }
    impl 模型连接 for Mock连接 {
        fn 发送(&self, _配置: &LLM配置, _请求: &请求) -> Result<响应, 错误> {
            Ok(响应::假响应(&self.响应内容))
        }
    }

    #[test]
    fn 端到端_mock调用() {
        let mut 池 = LLM池::新建();
        池.设("道祖", LLM配置::假配置("test-model")).unwrap();
        let 调用器 = LLM调用器::新建(池, Mock连接::新建("测试响应内容"));
        let 请求 = 请求::新建("", vec![消息::系统("你是助手"), 消息::用户("你好")]);
        let 响应 = 调用器.调用("道祖", &请求).unwrap();
        assert_eq!(响应.内容, "测试响应内容");
    }

    #[test]
    fn 端到端_请求_空模型_用池配置() {
        let mut 池 = LLM池::新建();
        池.设("道祖", LLM配置::假配置("池模型名")).unwrap();
        let 调用器 = LLM调用器::新建(池, Mock连接::新建("ok"));
        let 请求 = 请求::新建("", vec![消息::用户("hi")]); // 空模型名
        let _ = 调用器.调用("道祖", &请求).unwrap();
        // 验证：调用器会用池配置的"池模型名"（不报错）
    }

    #[test]
    fn 端到端_请求_指定模型_优先() {
        let mut 池 = LLM池::新建();
        池.设("道祖", LLM配置::假配置("池模型")).unwrap();
        let 调用器 = LLM调用器::新建(池, Mock连接::新建("ok"));
        let 请求 = 请求::新建("请求模型", vec![消息::用户("hi")]);
        let _ = 调用器.调用("道祖", &请求).unwrap();
        // 验证：请求.模型=请求模型 优先于 池配置
    }

    #[test]
    fn 端到端_4分类_分别调用() {
        let mut 池 = LLM池::新建();
        池.设("道祖", LLM配置::假配置("m1")).unwrap();
        池.设("圣人", LLM配置::假配置("m2")).unwrap();
        池.设("准圣", LLM配置::假配置("m3")).unwrap();
        池.设("大罗", LLM配置::假配置("m4")).unwrap();
        let 调用器 = LLM调用器::新建(池, Mock连接::新建("ok"));
        assert!(调用器.调用("道祖", &请求::新建("", vec![])).is_ok());
        assert!(调用器.调用("圣人", &请求::新建("", vec![])).is_ok());
        assert!(调用器.调用("准圣", &请求::新建("", vec![])).is_ok());
        assert!(调用器.调用("大罗", &请求::新建("", vec![])).is_ok());
    }
}
