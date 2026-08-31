//! 写入-执行-阁 - 真实写文件工具
//!
//! 阶段 7 工具循环升级：写文件工具从 mock 升级为真实 std::fs::write，
//! 写前必经路径白名单校验（治理铁律：LLM 产意图，确定性程序执行 + 白名单拦截越界）。

use crate::编排_工具_殿::{工具, 调用输入, 调用输出};

pub struct 写文件工具;

impl 写文件工具 {
    pub fn 新建() -> Self {
        Self
    }
}

impl 工具 for 写文件工具 {
    fn 名称(&self) -> &str {
        "写文件"
    }
    fn 描述(&self) -> &str {
        "真实写入文件（路径受白名单约束）"
    }
    fn 执行(&self, 输入: &调用输入) -> 调用输出 {
        let 路径 = 输入.参数.get("路径").cloned().unwrap_or_default();
        let 内容 = 输入.参数.get("内容").cloned().unwrap_or_default();
        if 路径.is_empty() {
            return 调用输出::失败("缺参数 路径");
        }
        // 路径白名单校验（治理铁律：越界写盘被拦截）
        if let Err(e) = crate::写入_工具_殿::校验_写入路径(路径.as_str()) {
            return 调用输出::失败(e);
        }
        // 真实写盘
        match std::fs::write(&路径, &内容) {
            Ok(_) => {
                调用输出::成功_有副作用(format!("写入 {}（{} 字节）", 路径, 内容.len()))
            }
            Err(e) => 调用输出::失败(format!("写入失败：{}", e)),
        }
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;
    use crate::编排_工具_殿::{
        工具ID, 工具注册表, 工具调用循环, 工具调用请求
    };

    #[test]
    fn 真实写入后读回一致() {
        let 临时 = "工具府测试_写入.tmp";
        let mut 输入 = 调用输入::default();
        输入.参数.insert("路径".to_string(), 临时.to_string());
        输入
            .参数
            .insert("内容".to_string(), "真实写入内容".to_string());
        let 工具 = 写文件工具::新建();
        let 输出 = 工具.执行(&输入);
        assert!(输出.副作用已发生, "写文件应有副作用");
        assert!(!输出.结果.contains("FAIL"), "写入应成功：{}", 输出.结果);
        // 读回验证
        let 读回 = std::fs::read_to_string(临时).expect("读回失败");
        assert_eq!(读回, "真实写入内容");
        std::fs::remove_file(临时).ok();
    }

    #[test]
    fn 写治理资产被白名单拦截() {
        let mut 输入 = 调用输入::default();
        输入.参数.insert("路径".to_string(), ".env".to_string());
        输入.参数.insert("内容".to_string(), "恶意覆盖".to_string());
        let 输出 = 写文件工具::新建().执行(&输入);
        assert!(输出.结果.contains("FAIL"), "应拦截 .env：{}", 输出.结果);
        assert!(!输出.副作用已发生, "拦截时不应有副作用");
    }

    #[test]
    fn 写文件进注册表调用循环() {
        let mut 注册表 = 工具注册表::新建();
        注册表.注册(工具ID::编辑, Box::new(写文件工具::新建()));
        let 临时 = "工具府测试_循环.tmp";
        let mut 输入 = 调用输入::default();
        输入.参数.insert("路径".to_string(), 临时.to_string());
        输入.参数.insert("内容".to_string(), "循环写入".to_string());
        let 输出 = 工具调用循环(
            &注册表,
            vec![工具调用请求 {
                id: 工具ID::编辑,
                输入,
            }],
        );
        assert_eq!(输出.len(), 1);
        assert!(输出[0].副作用已发生);
        std::fs::remove_file(临时).ok();
    }
}
