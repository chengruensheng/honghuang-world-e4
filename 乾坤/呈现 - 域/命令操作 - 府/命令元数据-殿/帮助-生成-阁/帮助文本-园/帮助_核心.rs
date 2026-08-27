//! 帮助文本 - CLI 帮助信息生成

pub fn 帮助文本() -> Vec<&'static str> {
    vec![
        "洪荒 · 世界 v3 CLI",
        "用法：",
        "  洪荒 init                    初始化工作空间",
        "  洪荒 status                  查看状态",
        "  洪荒 run --task=<任务标识>    运行任务流水线",
        "  洪荒 e2e                     端到端 mock LLM 流水线",
        "  洪荒 --help                  显示本帮助",
    ]
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 帮助含核心命令() {
        let h = 帮助文本();
        assert!(h.iter().any(|l| l.contains("init")));
        assert!(h.iter().any(|l| l.contains("status")));
        assert!(h.iter().any(|l| l.contains("run")));
    }
}
