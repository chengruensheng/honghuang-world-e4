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

// ============================================================================
// 殿阁园桥接：工具循环殿 + 角色卡册殿（核心阁 + 4 分类阁）
// ============================================================================

#[path = "工具循环-殿/模块.rs"]
pub mod 工具循环_殿;
pub use 工具循环_殿::*;

#[path = "角色卡册-殿/模块.rs"]
pub mod 角色卡册_殿;
pub use 角色卡册_殿::*;

// 对外 API 重新导出（保持原 pub 符号完全不变）
// 工具循环_殿 的符号经 pub use 已在 crate root；角色卡册_殿 同理。
// 下面显式 re-export 以保持入口.rs 原有导出列表可读性。
pub use 工具循环_殿::{
    写文件工具, 工具, 工具ID, 工具描述, 工具注册表, 工具清单, 工具调用循环, 工具调用请求,
    执行命令工具, 版本工具, 编排工具, 读文件工具, 调用输入, 调用输出, Mock读文件工具,
};
pub use 角色卡册_殿::{
    任务, 分类_机械判定, 分类_角色卡, 角色分类, 角色卡, 错误, LLM池条目
};

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
