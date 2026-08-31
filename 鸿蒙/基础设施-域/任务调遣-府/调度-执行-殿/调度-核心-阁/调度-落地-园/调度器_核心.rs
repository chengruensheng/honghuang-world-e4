//! 调度器 - 道祖级任务调度器数据结构 + 核心方法
//!
//! 决策锚：v4 阶段 18 多 agent 协同
//! 关联文档：04-设计/接口契约/调度器.md

use super::super::super::super::任务_数据_殿::任务_定义_阁::任务_项园_园::任务项;

pub struct 调度器 {
    pub 任务列表: Vec<任务项>,
    pub 完成数: u32,
    pub 失败数: u32,
}

impl 调度器 {
    pub fn 新建() -> Self {
        Self {
            任务列表: Vec::new(),
            完成数: 0,
            失败数: 0,
        }
    }
    pub fn 添加(&mut self, 任务: 任务项) {
        self.任务列表.push(任务);
    }
    pub fn 全部完成(&self) -> bool {
        use super::super::super::super::任务_数据_殿::状态_枚举_阁::状态_落地_园::任务状态;
        self.任务列表.iter().all(|t| t.状态 == 任务状态::已完成)
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 测试_调度器_添加任务() {
        let mut s = 调度器::新建();
        s.添加(任务项::新建("a"));
        s.添加(任务项::新建("b"));
        s.添加(任务项::新建("c"));
        assert_eq!(s.任务列表.len(), 3);
        assert_eq!(s.完成数, 0);
    }

    #[test]
    fn 测试_调度器_全部完成_初始为空() {
        let s = 调度器::新建();
        // 空调度器视为全部完成（无未完成任务）
        assert!(s.全部完成());
    }
}
