//! 任务单定义-阁 - 自举任务单结构 + 解析
//!
//! 自举任务单 = 用户描述「改哪个文件 + 做什么 + 怎么验收」的接口，
//! 对齐四要素门禁（decided_by + falsifiable + implements + 复现/验收）。
//! 决策锚：260830 第一版自举规划（阶段 1 任务单结构）。

use std::collections::HashMap;

/// 自举任务单：驱动流水线开发系统自己的最小输入单元
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct 自举任务单 {
    pub 标识: String,
    pub 目标文件: String,
    pub 需求描述: String,
    pub 验收命令: String,
    pub 可证伪命题: String,
    pub decided_by: String,
}

impl 自举任务单 {
    /// 从参数表解析；缺任一必填字段返回 Err(缺失字段名)
    pub fn 从参数解析(参数: &HashMap<String, String>) -> Result<Self, String> {
        let 必填 = [
            "标识",
            "目标文件",
            "需求描述",
            "验收命令",
            "可证伪命题",
            "decided_by",
        ];
        for 键 in 必填 {
            match 参数.get(键) {
                Some(值) if !值.trim().is_empty() => {}
                _ => return Err(format!("缺必填字段：{}", 键)),
            }
        }
        Ok(Self {
            标识: 参数["标识"].clone(),
            目标文件: 参数["目标文件"].clone(),
            需求描述: 参数["需求描述"].clone(),
            验收命令: 参数["验收命令"].clone(),
            可证伪命题: 参数["可证伪命题"].clone(),
            decided_by: 参数["decided_by"].clone(),
        })
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    fn 样例参数() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("标识".to_string(), "自举-001".to_string());
        m.insert("目标文件".to_string(), "世界/入口.rs".to_string());
        m.insert("需求描述".to_string(), "加一个版本命令".to_string());
        m.insert("验收命令".to_string(), "cargo test".to_string());
        m.insert(
            "可证伪命题".to_string(),
            "退出码0且版本字符串非空".to_string(),
        );
        m.insert("decided_by".to_string(), "界主".to_string());
        m
    }

    #[test]
    fn 全字段解析成功() {
        let 单 = 自举任务单::从参数解析(&样例参数()).expect("解析失败");
        assert_eq!(单.标识, "自举-001");
        assert_eq!(单.目标文件, "世界/入口.rs");
        assert_eq!(单.decided_by, "界主");
    }

    #[test]
    fn 缺字段报错() {
        let mut m = 样例参数();
        m.remove("验收命令");
        let r = 自举任务单::从参数解析(&m);
        assert!(r.is_err(), "缺验收命令应报错");
        assert!(r.unwrap_err().contains("验收命令"));
    }

    #[test]
    fn 空字符串视为缺字段() {
        let mut m = 样例参数();
        m.insert("标识".to_string(), "  ".to_string());
        assert!(自举任务单::从参数解析(&m).is_err());
    }
}
