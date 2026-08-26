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
}
