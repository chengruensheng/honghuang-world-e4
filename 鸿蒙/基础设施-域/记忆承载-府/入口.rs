//! 记忆承载 - 府
//!
//! 范畴维度 36 格位（6 范畴 × 6 格位）+ 时间维度 3 档投影（经/权/行）
//! + 来源维度 3 源记录（代码/LLM/人类）——三维正交写入/检索。
//!
//! 决策锚：260826-2240 传承殿启动 § 记忆模型
//! 关联文档：02-概念/记忆/03-记忆.md

/// 6 范畴（哲学层）——系统关心什么
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 范畴 {
    目标,
    规则,
    自我,
    程序,
    世界,
    经历,
}

/// 6 格位（范畴内部的位置）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 格位 {
    现状,
    期望,
    障碍,
    路径,
    评估,
    演化,
}

/// 3 档时间投影（儒家经权之辨）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 时间档 {
    /// 经典不变
    经档,
    /// 因时制宜
    权档,
    /// 当下行动
    行档,
}

/// 3 源记录（决策者来源）
///
/// 注：保留 "LLM" 全大写为业界缩写（接 AGENTS.md § 4.2）
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 来源 {
    代码,
    LLM,
    人类,
}

/// 记忆条目
#[derive(Clone, Debug)]
pub struct 记忆条目 {
    pub 范畴: 范畴,
    pub 格位: 格位,
    pub 时间档: 时间档,
    pub 来源: 来源,
    pub 内容: String,
}

/// 36 格位总览：6 × 6 = 36
pub const 格位总数: usize = 36;

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 三维枚举满足约束() {
        assert_eq!(6 * 6, 格位总数);
    }

    #[test]
    fn 记忆条目可构造() {
        let 条目 = 记忆条目 {
            范畴: 范畴::目标,
            格位: 格位::现状,
            时间档: 时间档::行档,
            来源: 来源::LLM,
            内容: "洪荒 · 世界 阶段 1 启动".to_string(),
        };
        assert_eq!(条目.范畴, 范畴::目标);
    }
}
