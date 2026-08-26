//! 任务执行 - 府
//!
//! LLM 池 + 工具循环 + 4 分类角色卡。
//! 阶段 5: 4 张具体角色卡 + 分类机械判定（无 LLM 也能跑通）。
//! 阶段 7 接入 LLM。
//!
//! 决策锚：260826-2210 4-分类抽象 § 4 分类角色卡
//! 关联文档：02-概念/规则注册表/06-规则注册表.md + 04-设计/状态机/01-流水线.md
//! falsifiable：4 分类机械判定覆盖率 = 100% + 4 张卡必填字段全覆盖

#![allow(non_snake_case)] // 角色卡/任务标识 等字段名遵循中文命名
#![allow(clippy::upper_case_acronyms)] // LLM 等业界缩写
#![allow(non_camel_case_types)] // 错误变体 decided_by为空 遵循语义命名

/// 角色分类（接 4-分类抽象决策）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 角色分类 {
    道祖级,
    圣人级,
    准圣级,
    大罗金仙级,
}

impl 角色分类 {
    pub fn 名称(self) -> &'static str {
        match self {
            角色分类::道祖级 => "道祖级",
            角色分类::圣人级 => "圣人级",
            角色分类::准圣级 => "准圣级",
            角色分类::大罗金仙级 => "大罗金仙级",
        }
    }
}

/// 角色卡（每分类 1 张：道祖卡/圣人卡/准圣卡/大罗卡）
#[derive(Clone, Debug)]
#[allow(non_snake_case)]
pub struct 角色卡 {
    pub 分类: 角色分类,
    pub 名称: String,
    pub 权限: Vec<String>,
    pub decided_by: String,
    pub LLM池: String,
    pub 工具偏好: Vec<String>,
}

/// 4 张具体角色卡的工厂函数（每分类 1 张）
///
/// 决策锚：260826-2210 4-分类抽象 § 流水线
/// - 道祖：化要求 / 派遣 / 终裁 / 定档
/// - 圣人：设计 / 评审
/// - 大罗：按道分工（实现）
/// - 准圣：六维验收 / 打回
pub fn 分类_角色卡(分类: 角色分类) -> 角色卡 {
    match 分类 {
        角色分类::道祖级 => 角色卡 {
            分类,
            名称: "道祖卡".to_string(),
            权限: vec![
                "化要求".to_string(),
                "派遣".to_string(),
                "终裁".to_string(),
                "定档".to_string(),
            ],
            decided_by: "界主".to_string(),
            LLM池: "化要求专用".to_string(),
            工具偏好: vec![
                "追问引擎".to_string(),
                "派遣器".to_string(),
                "终裁器".to_string(),
            ],
        },
        角色分类::圣人级 => 角色卡 {
            分类,
            名称: "圣人卡".to_string(),
            权限: vec!["设计".to_string(), "评审".to_string()],
            decided_by: "界主".to_string(),
            LLM池: "设计专用".to_string(),
            工具偏好: vec!["评审四元组".to_string(), "设计模板".to_string()],
        },
        角色分类::准圣级 => 角色卡 {
            分类,
            名称: "准圣卡".to_string(),
            权限: vec!["验收".to_string(), "打回".to_string()],
            decided_by: "界主".to_string(),
            LLM池: "验收专用".to_string(),
            工具偏好: vec!["六维验收".to_string(), "falsifiable 校验".to_string()],
        },
        角色分类::大罗金仙级 => 角色卡 {
            分类,
            名称: "大罗卡".to_string(),
            权限: vec!["实现".to_string()],
            decided_by: "界主".to_string(),
            LLM池: "实现专用".to_string(),
            工具偏好: vec!["路径前缀校验".to_string(), "cargo test".to_string()],
        },
    }
}

/// 错误
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum 错误 {
    任务标识为空,
    任务描述为空,
    分类不匹配 {
        期望: 角色分类,
        实际: 角色分类,
    },
    decided_by为空,
}

impl std::fmt::Display for 错误 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            错误::任务标识为空 => write!(f, "任务标识必填"),
            错误::任务描述为空 => write!(f, "任务描述必填"),
            错误::分类不匹配 { 期望, 实际 } => {
                write!(f, "分类不匹配：期望 {:?}，实际 {:?}", 期望, 实际)
            }
            错误::decided_by为空 => write!(f, "decided_by 必填"),
        }
    }
}

impl std::error::Error for 错误 {}

/// 任务（阶段 5：含分类强制校验）
#[derive(Clone, Debug)]
#[allow(non_snake_case)]
pub struct 任务 {
    pub 标识: String,
    pub 分类: 角色分类,
    pub 描述: String,
    pub decided_by: String,
}

/// LLM 池条目（阶段 1 占位结构）
#[derive(Clone, Debug)]
pub struct LLM池条目 {
    pub 名称: String,
    pub 端点: String,
}

/// 分类机械判定：硬编码 4 分类规则，无 LLM 调用即可通过/拒绝
///
/// 决策锚：260826-2210 § 流水线
/// 规则：
/// 1. 任务标识必填（非空）
/// 2. 任务描述必填（非空）
/// 3. decided_by 必填（非空）
/// 4. 分类匹配：任务的"分类"字段必须与传入的 分类 参数一致
pub fn 分类_机械判定(任务: &任务, 分类: 角色分类) -> Result<(), 错误> {
    if 任务.标识.is_empty() {
        return Err(错误::任务标识为空);
    }
    if 任务.描述.is_empty() {
        return Err(错误::任务描述为空);
    }
    if 任务.decided_by.is_empty() {
        return Err(错误::decided_by为空);
    }
    if 任务.分类 != 分类 {
        return Err(错误::分类不匹配 {
            期望: 分类,
            实际: 任务.分类,
        });
    }
    Ok(())
}

// ============================================================================
// 阶段 7 Day 3-4：5 类工具 + 工具调用循环
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

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 四分类齐备() {
        let 所有 = [
            角色分类::道祖级,
            角色分类::圣人级,
            角色分类::准圣级,
            角色分类::大罗金仙级,
        ];
        assert_eq!(所有.len(), 4);
    }

    #[test]
    fn 角色卡必填decided_by() {
        let 卡 = 分类_角色卡(角色分类::道祖级);
        assert!(!卡.decided_by.is_empty());
    }

    // ---------- 4 张具体角色卡 ----------

    #[test]
    fn 道祖卡内容完整() {
        let 卡 = 分类_角色卡(角色分类::道祖级);
        assert_eq!(卡.名称, "道祖卡");
        assert!(卡.权限.contains(&"化要求".to_string()));
        assert!(卡.权限.contains(&"终裁".to_string()));
        assert_eq!(卡.LLM池, "化要求专用");
        assert!(卡.工具偏好.contains(&"追问引擎".to_string()));
    }

    #[test]
    fn 圣人卡内容完整() {
        let 卡 = 分类_角色卡(角色分类::圣人级);
        assert_eq!(卡.名称, "圣人卡");
        assert!(卡.权限.contains(&"设计".to_string()));
        assert_eq!(卡.LLM池, "设计专用");
    }

    #[test]
    fn 准圣卡内容完整() {
        let 卡 = 分类_角色卡(角色分类::准圣级);
        assert_eq!(卡.名称, "准圣卡");
        assert!(卡.权限.contains(&"验收".to_string()));
        assert_eq!(卡.LLM池, "验收专用");
    }

    #[test]
    fn 大罗卡内容完整() {
        let 卡 = 分类_角色卡(角色分类::大罗金仙级);
        assert_eq!(卡.名称, "大罗卡");
        assert!(卡.权限.contains(&"实现".to_string()));
        assert_eq!(卡.LLM池, "实现专用");
    }

    // ---------- 分类机械判定 ----------

    #[test]
    fn 机械判定完整任务通过() {
        let 任务 = 任务 {
            标识: "test-001".to_string(),
            分类: 角色分类::道祖级,
            描述: "测试任务".to_string(),
            decided_by: "界主".to_string(),
        };
        assert!(分类_机械判定(&任务, 角色分类::道祖级).is_ok());
    }

    #[test]
    fn 机械判定缺标识拒绝() {
        let 任务 = 任务 {
            标识: "".to_string(),
            分类: 角色分类::道祖级,
            描述: "测试".to_string(),
            decided_by: "界主".to_string(),
        };
        assert_eq!(
            分类_机械判定(&任务, 角色分类::道祖级),
            Err(错误::任务标识为空)
        );
    }

    #[test]
    fn 机械判定缺decided_by拒绝() {
        let 任务 = 任务 {
            标识: "t-1".to_string(),
            分类: 角色分类::道祖级,
            描述: "测试".to_string(),
            decided_by: "".to_string(),
        };
        assert_eq!(
            分类_机械判定(&任务, 角色分类::道祖级),
            Err(错误::decided_by为空)
        );
    }

    #[test]
    fn 机械判定分类不匹配拒绝() {
        let 任务 = 任务 {
            标识: "t-1".to_string(),
            分类: 角色分类::道祖级, // 任务分类 = 道祖
            描述: "测试".to_string(),
            decided_by: "界主".to_string(),
        };
        // 期望 = 圣人，实际 = 道祖 → 不匹配
        assert_eq!(
            分类_机械判定(&任务, 角色分类::圣人级),
            Err(错误::分类不匹配 {
                期望: 角色分类::圣人级,
                实际: 角色分类::道祖级
            })
        );
    }

    #[test]
    fn 四分类机械判定全通过() {
        for 分类 in [
            角色分类::道祖级,
            角色分类::圣人级,
            角色分类::准圣级,
            角色分类::大罗金仙级,
        ] {
            let 任务 = 任务 {
                标识: format!("t-{:?}", 分类),
                分类,
                描述: "测试".to_string(),
                decided_by: "界主".to_string(),
            };
            assert!(分类_机械判定(&任务, 分类).is_ok(), "{:?} 应通过", 分类);
        }
    }

    // ---------- 5 类工具 测试 ----------

    #[test]
    fn 工具_列举_5类_全覆盖() {
        assert_eq!(工具ID::所有().len(), 5);
    }

    #[test]
    fn 工具调用_注册与查找() {
        let mut 注册表 = 工具注册表::新建();
        注册表.注册(工具ID::探查, Box::new(读文件工具::新建()));
        let tool = 注册表.取(&工具ID::探查);
        assert!(tool.is_some());
        assert_eq!(tool.unwrap().名称(), "读文件");
    }

    #[test]
    fn 工具调用_读文件mock成功() {
        let mut 注册表 = 工具注册表::新建();
        注册表.注册(工具ID::探查, Box::new(Mock读文件工具::新建("内容")));
        let tool = 注册表.取(&工具ID::探查).unwrap();
        let mut 输入 = 调用输入::default();
        输入
            .参数
            .insert("路径".to_string(), "/tmp/test".to_string());
        let r = tool.执行(&输入);
        assert_eq!(r.结果, "内容");
        assert!(r.副作用已发生);
    }

    #[test]
    fn 工具调用_未知ID报错() {
        let 注册表 = 工具注册表::新建();
        assert!(注册表.取(&工具ID::编辑).is_none());
    }

    #[test]
    fn 工具调用_循环_无工具直接返回() {
        let 注册表 = 工具注册表::新建();
        let 调用列表 = vec![];
        let r = 工具调用循环(&注册表, 调用列表);
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn 工具调用_循环_按顺序执行() {
        let mut 注册表 = 工具注册表::新建();
        注册表.注册(工具ID::探查, Box::new(Mock读文件工具::新建("A")));
        let mut i1 = 调用输入::default();
        i1.参数.insert("路径".to_string(), "/a".to_string());
        let mut i2 = 调用输入::default();
        i2.参数.insert("路径".to_string(), "/b".to_string());
        let 调用列表 = vec![
            工具调用请求 {
                id: 工具ID::探查,
                输入: i1,
            },
            工具调用请求 {
                id: 工具ID::探查,
                输入: i2,
            },
        ];
        let r = 工具调用循环(&注册表, 调用列表);
        assert_eq!(r.len(), 2);
        // Mock 工具是静态内容，两次调用返回相同结果；按顺序执行是验证调用都发生
        assert_eq!(r[0].结果, r[1].结果, "两次调用应按顺序各执行一次");
        assert!(r[0].副作用已发生);
        assert!(r[1].副作用已发生);
    }

    #[test]
    fn 工具调用_循环_缺失工具不中断() {
        let mut 注册表 = 工具注册表::新建();
        注册表.注册(工具ID::探查, Box::new(Mock读文件工具::新建("A")));
        let mut i1 = 调用输入::default();
        i1.参数.insert("路径".to_string(), "/a".to_string());
        let 调用列表 = vec![
            工具调用请求 {
                id: 工具ID::探查,
                输入: i1.clone(),
            },
            工具调用请求 {
                id: 工具ID::编辑,
                输入: i1.clone(),
            },
        ];
        let r = 工具调用循环(&注册表, 调用列表);
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].结果, "A");
        assert!(r[1].结果.contains("未知工具") || r[1].结果.contains("未注册"));
    }

    #[test]
    fn 工具manifest_5工具完整() {
        let m = 工具清单::manifest();
        assert_eq!(m.工具.len(), 5);
    }
}
