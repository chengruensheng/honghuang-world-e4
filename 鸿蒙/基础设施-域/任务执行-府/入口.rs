//! 任务执行 - 府
//!
//! LLM 池 + 工具循环 + 4 分类角色卡。
//! 阶段 1 仅承载类型表面；阶段 7 接入 LLM。
//!
//! 决策锚：260826-2210 4-分类抽象 § 4 分类角色卡
//! 关联文档：02-概念/规则注册表/06-规则注册表.md

/// 角色分类（接 4-分类抽象决策）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 角色分类 {
    道祖级,
    圣人级,
    准圣级,
    大罗金仙级,
}

/// 角色卡（每分类 1 张：道祖卡/圣人卡/准圣卡/大罗卡）
#[derive(Clone, Debug)]
pub struct 角色卡 {
    pub 分类: 角色分类,
    pub 名称: String,
    pub 权限: Vec<String>,
    pub decided_by: String,
}

/// LLM 池条目（阶段 1 占位结构）
#[derive(Clone, Debug)]
pub struct LLM池条目 {
    pub 名称: String,
    pub 端点: String,
}

/// 任务
#[derive(Clone, Debug)]
pub struct 任务 {
    pub 标识: String,
    pub 分类: 角色分类,
    pub 描述: String,
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
        let 卡 = 角色卡 {
            分类: 角色分类::道祖级,
            名称: "道祖卡".to_string(),
            权限: vec!["派遣".to_string(), "终裁".to_string()],
            decided_by: "界主".to_string(),
        };
        assert!(!卡.decided_by.is_empty());
    }
}
