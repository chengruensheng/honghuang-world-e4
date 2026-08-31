//! 运行循环 - 调度器运行全部任务 + 错误隔离
//!
//! 决策锚：v4 阶段 18 多 agent 协同
//! 关联文档：04-设计/接口契约/调度器.md §运行循环

use super::super::super::super::任务_数据_殿::任务_定义_阁::任务_项园_园::任务项;
use super::super::super::调度_核心_阁::调度_落地_园::调度器;

impl 调度器 {
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
}

#[cfg(test)]
mod 测试 {
    use super::*;

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
