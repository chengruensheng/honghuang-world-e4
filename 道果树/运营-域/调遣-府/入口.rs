//! 调遣-府 - v4 阶段 18 多 agent 协同（道祖级调度器）

#![allow(non_snake_case)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 任务状态 {
    待执行,
    执行中,
    已完成,
    失败,
}

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

    pub fn 运行_全部<F>(&mut self, mut 执行函数: F)
    where
        F: FnMut(&任务项) -> Result<(), String>,
    {
        for i in 0..self.任务列表.len() {
            self.任务列表[i].标记_执行中();
            match 执行函数(&self.任务列表[i].clone()) {
                Ok(()) => {
                    self.任务列表[i].标记_完成();
                    self.完成数 += 1;
                }
                Err(e) => {
                    self.任务列表[i].标记_失败(e);
                    self.失败数 += 1;
                }
            }
        }
    }

    pub fn 全部完成(&self) -> bool {
        self.任务列表.iter().all(|t| t.状态 == 任务状态::已完成)
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

    #[test]
    fn 测试_调度器_添加任务() {
        let mut s = 调度器::新建();
        s.添加(任务项::新建("a"));
        s.添加(任务项::新建("b"));
        s.添加(任务项::新建("c"));
        assert_eq!(s.任务列表.len(), 3);
    }

    #[test]
    fn 测试_调度器_运行全部_全部成功() {
        let mut s = 调度器::新建();
        s.添加(任务项::新建("a"));
        s.添加(任务项::新建("b"));
        let mut counter = 0;
        s.运行_全部(|_| {
            counter += 1;
            Ok(())
        });
        assert_eq!(counter, 2);
        assert_eq!(s.完成数, 2);
        assert!(s.全部完成());
    }

    #[test]
    fn 测试_调度器_错误隔离() {
        let mut s = 调度器::新建();
        s.添加(任务项::新建("a"));
        s.添加(任务项::新建("b"));
        s.添加(任务项::新建("c"));
        let mut count = 0;
        s.运行_全部(|t| {
            count += 1;
            if t.标识 == "b" {
                Err("失败".to_string())
            } else {
                Ok(())
            }
        });
        assert_eq!(count, 3);
        assert_eq!(s.完成数, 2);
        assert_eq!(s.失败数, 1);
        assert!(!s.全部完成());
    }

    #[test]
    fn 测试_调度器_双任务并行模拟() {
        let mut s = 调度器::新建();
        for i in 0..2 {
            s.添加(任务项::新建(format!("dalun-{:03}", i)));
        }
        s.运行_全部(|t| {
            assert!(t.标识.starts_with("dalun-"));
            Ok(())
        });
        assert_eq!(s.完成数, 2);
        assert_eq!(s.失败数, 0);
        assert!(s.全部完成());
    }
}
