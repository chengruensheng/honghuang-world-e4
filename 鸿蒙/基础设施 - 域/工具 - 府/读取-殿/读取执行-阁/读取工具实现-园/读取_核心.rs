//! 读取执行-阁 - 真实读文件工具
//!
//! 阶段 7 工具循环升级：读文件工具从 mock 升级为真实 std::fs::read_to_string。

use crate::编排_殿::{工具, 调用输入, 调用输出};

pub struct 读文件工具;

impl 读文件工具 {
    pub fn 新建() -> Self {
        Self
    }
}

impl 工具 for 读文件工具 {
    fn 名称(&self) -> &str {
        "读文件"
    }
    fn 描述(&self) -> &str {
        "真实读取文件内容（路径受白名单约束）"
    }
    fn 执行(&self, 输入: &调用输入) -> 调用输出 {
        let 路径 = 输入.参数.get("路径").cloned().unwrap_or_default();
        if 路径.is_empty() {
            return 调用输出::失败("缺参数 路径");
        }
        if let Err(e) = crate::读取_殿::校验_读取路径(路径.as_str()) {
            return 调用输出::失败(e);
        }
        match std::fs::read_to_string(&路径) {
            Ok(内容) => 调用输出::成功(内容),
            Err(e) => 调用输出::失败(format!("读取失败：{}", e)),
        }
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;
    use crate::编排_殿::{工具ID, 工具注册表, 工具调用循环, 工具调用请求};

    #[test]
    fn 真实读取返回内容() {
        let 临时 = "工具府测试_读取.tmp";
        std::fs::write(临时, "真实读取内容").expect("写临时失败");
        let mut 输入 = 调用输入::default();
        输入.参数.insert("路径".to_string(), 临时.to_string());
        let 输出 = 读文件工具::新建().执行(&输入);
        assert_eq!(输出.结果, "真实读取内容");
        std::fs::remove_file(临时).ok();
    }

    #[test]
    fn 读取治理资产被拦截() {
        let mut 输入 = 调用输入::default();
        输入.参数.insert("路径".to_string(), ".env".to_string());
        let 输出 = 读文件工具::新建().执行(&输入);
        assert!(输出.结果.contains("FAIL"), "应拦截 .env：{}", 输出.结果);
    }

    #[test]
    fn 读文件进注册表调用循环() {
        let 临时 = "工具府测试_读循环.tmp";
        std::fs::write(临时, "循环读").expect("写临时失败");
        let mut 注册表 = 工具注册表::新建();
        注册表.注册(工具ID::探查, Box::new(读文件工具::新建()));
        let mut 输入 = 调用输入::default();
        输入.参数.insert("路径".to_string(), 临时.to_string());
        let 输出 = 工具调用循环(
            &注册表,
            vec![工具调用请求 {
                id: 工具ID::探查,
                输入,
            }],
        );
        assert_eq!(输出.len(), 1);
        assert_eq!(输出[0].结果, "循环读");
        std::fs::remove_file(临时).ok();
    }
}
