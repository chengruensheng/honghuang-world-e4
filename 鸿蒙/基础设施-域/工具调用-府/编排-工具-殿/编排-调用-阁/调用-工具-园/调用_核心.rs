//! 编排-调用-阁 - 工具框架核心类型
//!
//! 工具 ID / 工具 trait / 调用输入输出 / 注册表 / 调用循环。
//! 决策锚：260826-2210 4-分类抽象 § 流水线（阶段 7 工具循环升级为真实工具）

// ============================================================================
// 工具 ID（5 类：探查/编辑/执行/版本/编排）
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
// 工具 trait + 调用请求 + 注册表 + 调用循环
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
// 编排工具：任务编排（真实返回编排计划 + 可用工具清单）
// ============================================================================

pub struct 编排工具;

impl 编排工具 {
    pub fn 新建() -> Self {
        Self
    }
}

impl 工具 for 编排工具 {
    fn 名称(&self) -> &str {
        "编排"
    }
    fn 描述(&self) -> &str {
        "任务编排：返回 5 类工具编排计划"
    }
    fn 执行(&self, 输入: &调用输入) -> 调用输出 {
        let 目标 = 输入.参数.get("目标").cloned().unwrap_or_default();
        if 目标.is_empty() {
            return 调用输出::失败("缺参数 目标");
        }
        let 计划 = format!(
            "编排计划（目标={}）：探查→编辑→执行→版本→编排，5 类工具按序协作",
            目标
        );
        调用输出::成功(计划)
    }
}
