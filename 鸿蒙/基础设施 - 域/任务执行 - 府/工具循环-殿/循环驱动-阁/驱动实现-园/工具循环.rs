//! 工具循环殿 - 5 类工具 + 工具调用循环
//!
//! 阶段 7 Day 3-4：5 类工具 + 工具调用循环
//! 决策锚：260826-2210 4-分类抽象 § 流水线

// ============================================================================
// 工具 ID
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 工具ID {
    探查,
    编辑,
    执行,
    版本,
    编排,
}

impl 工具ID {
    pub fn 名称(self) -> &'static str {
        match self {
            工具ID::探查 => "探查",
            工具ID::编辑 => "编辑",
            工具ID::执行 => "执行",
            工具ID::版本 => "版本",
            工具ID::编排 => "编排",
        }
    }
    pub fn 所有() -> [工具ID; 5] {
        [
            工具ID::探查,
            工具ID::编辑,
            工具ID::执行,
            工具ID::版本,
            工具ID::编排,
        ]
    }
}

// ============================================================================
// 调用输入 / 调用输出
// ============================================================================

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct 调用输入 {
    pub 参数: std::collections::HashMap<String, String>,
}

impl 调用输入 {
    pub fn 设(mut self, 键: &str, 值: &str) -> Self {
        self.参数.insert(键.to_string(), 值.to_string());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 调用输出 {
    pub 结果: String,
    pub 副作用已发生: bool,
}

impl 调用输出 {
    pub fn 成功(结果: impl Into<String>) -> Self {
        Self {
            结果: 结果.into(),
            副作用已发生: false,
        }
    }
    pub fn 成功_有副作用(结果: impl Into<String>) -> Self {
        Self {
            结果: 结果.into(),
            副作用已发生: true,
        }
    }
    pub fn 失败(msg: impl Into<String>) -> Self {
        Self {
            结果: format!("FAIL: {}", msg.into()),
            副作用已发生: false,
        }
    }
}

// ============================================================================
// 工具 trait + 清单 + 注册表
// ============================================================================

pub trait 工具: Send + Sync {
    fn 名称(&self) -> &str;
    fn 描述(&self) -> &str;
    fn 执行(&self, 输入: &调用输入) -> 调用输出;
}

#[derive(Clone, Debug)]
pub struct 工具调用请求 {
    pub id: 工具ID,
    pub 输入: 调用输入,
}

#[derive(Clone, Debug)]
pub struct 工具清单 {
    pub 工具: Vec<工具描述>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 工具描述 {
    pub 名称: String,
    pub 描述: String,
    pub 参数schema: String,
}

impl 工具清单 {
    pub fn manifest() -> Self {
        Self {
            工具: vec![
                工具描述 {
                    名称: "探查".to_string(),
                    描述: "读取文件/列出目录（无副作用）".to_string(),
                    参数schema: r#"{"type":"object","properties":{"路径":{"type":"string"}},"required":["路径"]}"#.to_string(),
                },
                工具描述 {
                    名称: "编辑".to_string(),
                    描述: "写入或修改文件（有副作用）".to_string(),
                    参数schema: r#"{"type":"object","properties":{"路径":{"type":"string"},"内容":{"type":"string"}},"required":["路径","内容"]}"#.to_string(),
                },
                工具描述 {
                    名称: "执行".to_string(),
                    描述: "执行 shell 命令或测试".to_string(),
                    参数schema: r#"{"type":"object","properties":{"命令":{"type":"string"}},"required":["命令"]}"#.to_string(),
                },
                工具描述 {
                    名称: "版本".to_string(),
                    描述: "git 操作（status/log/diff）".to_string(),
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

#[derive(Default)]
pub struct 工具注册表 {
    注册: std::collections::HashMap<工具ID, Box<dyn 工具>>,
}

impl 工具注册表 {
    pub fn 新建() -> Self {
        Self::default()
    }
    pub fn 注册(&mut self, id: 工具ID, 工具: Box<dyn 工具>) {
        self.注册.insert(id, 工具);
    }
    pub fn 取(&self, id: &工具ID) -> Option<&dyn 工具> {
        self.注册.get(id).map(|b| b.as_ref())
    }
    pub fn 数量(&self) -> usize {
        self.注册.len()
    }
}

/// 工具调用循环：按顺序执行调用列表，未知工具不中断
pub fn 工具调用循环(
    注册表: &工具注册表, 调用列表: Vec<工具调用请求>
) -> Vec<调用输出> {
    let mut 输出列表 = Vec::new();
    for 请求 in 调用列表 {
        let 输出 = match 注册表.取(&请求.id) {
            Some(tool) => tool.执行(&请求.输入),
            None => 调用输出::失败(format!("未知工具/未注册：{:?}", 请求.id)),
        };
        输出列表.push(输出);
    }
    输出列表
}

// ============================================================================
// 5 类具体工具（mock 实现）
// ============================================================================

pub struct 读文件工具 {
    mock内容: String,
}
impl 读文件工具 {
    pub fn 新建() -> Self {
        Self {
            mock内容: "<文件内容>".to_string(),
        }
    }
    pub fn 新建_mock(内容: impl Into<String>) -> Self {
        Self {
            mock内容: 内容.into(),
        }
    }
}
impl 工具 for 读文件工具 {
    fn 名称(&self) -> &str {
        "读文件"
    }
    fn 描述(&self) -> &str {
        "读取文件内容（mock）"
    }
    fn 执行(&self, 输入: &调用输入) -> 调用输出 {
        let 路径 = 输入.参数.get("路径").cloned().unwrap_or_default();
        if 路径.is_empty() {
            return 调用输出::失败("缺参数 路径");
        }
        调用输出::成功(format!("[mock read {}]: {}", 路径, self.mock内容))
    }
}

pub struct Mock读文件工具 {
    内容: String,
}
impl Mock读文件工具 {
    pub fn 新建(内容: impl Into<String>) -> Self {
        Self {
            内容: 内容.into()
        }
    }
}
impl 工具 for Mock读文件工具 {
    fn 名称(&self) -> &str {
        "mock读文件"
    }
    fn 描述(&self) -> &str {
        "测试 mock"
    }
    fn 执行(&self, _输入: &调用输入) -> 调用输出 {
        调用输出::成功_有副作用(&self.内容)
    }
}

pub struct 写文件工具;
impl 工具 for 写文件工具 {
    fn 名称(&self) -> &str {
        "写文件"
    }
    fn 描述(&self) -> &str {
        "写入文件（mock）"
    }
    fn 执行(&self, 输入: &调用输入) -> 调用输出 {
        let 路径 = 输入.参数.get("路径").cloned().unwrap_or_default();
        let 内容 = 输入.参数.get("内容").cloned().unwrap_or_default();
        if 路径.is_empty() || 内容.is_empty() {
            return 调用输出::失败("缺参数 路径/内容");
        }
        调用输出::成功_有副作用(format!("[mock write {}] {} bytes", 路径, 内容.len()))
    }
}

pub struct 执行命令工具;
impl 工具 for 执行命令工具 {
    fn 名称(&self) -> &str {
        "执行命令"
    }
    fn 描述(&self) -> &str {
        "执行 shell 命令（mock）"
    }
    fn 执行(&self, 输入: &调用输入) -> 调用输出 {
        let 命令 = 输入.参数.get("命令").cloned().unwrap_or_default();
        if 命令.is_empty() {
            return 调用输出::失败("缺参数 命令");
        }
        调用输出::成功_有副作用(format!("[mock exec] {}", 命令))
    }
}

pub struct 版本工具;
impl 工具 for 版本工具 {
    fn 名称(&self) -> &str {
        "版本"
    }
    fn 描述(&self) -> &str {
        "git 操作（mock）"
    }
    fn 执行(&self, 输入: &调用输入) -> 调用输出 {
        let 子命令 = 输入.参数.get("子命令").cloned().unwrap_or_default();
        if 子命令.is_empty() {
            return 调用输出::失败("缺参数 子命令");
        }
        调用输出::成功(format!("[mock git {}]", 子命令))
    }
}

pub struct 编排工具;
impl 工具 for 编排工具 {
    fn 名称(&self) -> &str {
        "编排"
    }
    fn 描述(&self) -> &str {
        "任务编排（mock）"
    }
    fn 执行(&self, 输入: &调用输入) -> 调用输出 {
        let 目标 = 输入.参数.get("目标").cloned().unwrap_or_default();
        if 目标.is_empty() {
            return 调用输出::失败("缺参数 目标");
        }
        调用输出::成功(format!("[mock plan] {}", 目标))
    }
}
