//! 任务项 - 道祖级调度器的任务数据结构 + 状态机方法
//!
//! 决策锚：v4 阶段 18 多 agent 协同
//! 关联文档：04-设计/接口契约/任务项.md

use super::super::super::状态_枚举_阁::状态_落地_园::任务状态;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 任务项 {
    pub 标识: String,
    pub 状态: 任务状态,
    pub 错误信息: Option<String>,
}

impl 任务项 {
    pub fn 新建(标识: impl Into<String>) -> Self {
        Self {
            标识: 标识.into(),
            状态: 任务状态::待执行,
            错误信息: None,
        }
    }
    pub fn 标记_执行中(&mut self) {
        self.状态 = 任务状态::执行中;
    }
    pub fn 标记_完成(&mut self) {
        self.状态 = 任务状态::已完成;
    }
    pub fn 标记_失败(&mut self, 错误: impl Into<String>) {
        self.状态 = 任务状态::失败;
        self.错误信息 = Some(错误.into());
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 测试_任务项_新建() {
        let t = 任务项::新建("task-001");
        assert_eq!(t.标识, "task-001");
        assert_eq!(t.状态, 任务状态::待执行);
        assert_eq!(t.错误信息, None);
    }

    #[test]
    fn 测试_任务项_状态机() {
        let mut t = 任务项::新建("task-002");
        assert_eq!(t.状态, 任务状态::待执行);
        t.标记_执行中();
        assert_eq!(t.状态, 任务状态::执行中);
        t.标记_完成();
        assert_eq!(t.状态, 任务状态::已完成);
    }

    #[test]
    fn 测试_任务项_失败记录() {
        let mut t = 任务项::新建("task-003");
        t.标记_失败("测试错误");
        assert_eq!(t.状态, 任务状态::失败);
        assert_eq!(t.错误信息, Some("测试错误".to_string()));
    }
}
