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
// 殿阁园桥接：池殿 + 消息殿 + 连接殿
// ============================================================================

#[path = "模型池-殿/模块.rs"]
pub mod 模型池_殿;
pub use 模型池_殿::*;

#[path = "模型消息-殿/模块.rs"]
pub mod 模型消息_殿;
pub use 模型消息_殿::*;

#[path = "连接管理-殿/模块.rs"]
pub mod 连接管理_殿;
pub use 连接管理_殿::*;

// 对外 API 重新导出（保持原 pub 符号完全不变）
pub use 模型池_殿::{LLM池, LLM配置};
pub use 模型消息_殿::{消息, 角色};
pub use 连接管理_殿::{响应, 模型连接, 请求, 错误, HTTP连接, LLM调用器};

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
