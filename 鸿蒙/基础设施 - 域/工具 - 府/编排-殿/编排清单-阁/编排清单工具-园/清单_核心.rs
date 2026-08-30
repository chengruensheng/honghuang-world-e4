//! 编排清单-阁 - 工具清单 manifest
//!
//! 5 类工具的参数 schema 与描述（供 LLM 选择工具时参考）。

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 工具描述 {
    pub 名称: String,
    pub 描述: String,
    pub 参数schema: String,
}

#[derive(Clone, Debug)]
pub struct 工具清单 {
    pub 工具: Vec<工具描述>,
}

impl 工具清单 {
    pub fn manifest() -> Self {
        Self {
            工具: vec![
                工具描述 {
                    名称: "探查".to_string(),
                    描述: "读取文件内容（无副作用）".to_string(),
                    参数schema: r#"{"type":"object","properties":{"路径":{"type":"string"}},"required":["路径"]}"#.to_string(),
                },
                工具描述 {
                    名称: "编辑".to_string(),
                    描述: "写入文件（有副作用，路径受白名单约束）".to_string(),
                    参数schema: r#"{"type":"object","properties":{"路径":{"type":"string"},"内容":{"type":"string"}},"required":["路径","内容"]}"#.to_string(),
                },
                工具描述 {
                    名称: "执行".to_string(),
                    描述: "执行 cargo 命令（受命令白名单约束）".to_string(),
                    参数schema: r#"{"type":"object","properties":{"命令":{"type":"string"}},"required":["命令"]}"#.to_string(),
                },
                工具描述 {
                    名称: "版本".to_string(),
                    描述: "git 只读操作（status/log/diff）".to_string(),
                    参数schema: r#"{"type":"object","properties":{"子命令":{"type":"string"}},"required":["子命令"]}"#.to_string(),
                },
                工具描述 {
                    名称: "编排".to_string(),
                    描述: "任务编排（plan/orchestrate）".to_string(),
                    参数schema: r#"{"type":"object","properties":{"目标":{"type":"string"}},"required":["目标"]}"#.to_string(),
                },
            ],
        }
    }
}
