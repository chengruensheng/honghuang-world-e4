//! 命令操作 - 府
//!
//! CLI 入口 + 命令解析 + 权限校验。
//! 阶段 6: 3 条真命令（init / run / status）+ 1 个 e2e 流水线任务。
//! 阶段 7 Day 5-6: 端到端 mock LLM + 4 分类 LLM 池对接。
//! v4 阶段 17: 真实 HTTP 走 mock server（shishi_fu）。

#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

#[path = "命令调度-殿/模块.rs"]
pub mod 命令调度_殿;

#[path = "命令元数据-殿/模块.rs"]
pub mod 命令元数据_殿;

#[path = "记忆读取-殿/模块.rs"]
pub mod 记忆读取_殿;
pub use 命令元数据_殿::命令_清单_阁::命令清单_园::{
    命令清单, 命令清单_vec
};
pub use 命令元数据_殿::帮助_生成_阁::帮助文本_园::帮助文本;
pub use 命令元数据_殿::版本_查询_阁::版本标识_园::{版本, 项目};

pub use 命令调度_殿::{
    分发, 命令, 命令结果, 帮助命令, 自检命令, 跑流水线, 跑流水线_mock_llm, 跑流水线_反序,
    跑流水线_循环打回, 跑流水线_真实_llm, 跑流水线_跳层, Init命令, MockLLM连接, Run命令,
    Status命令,
};

// 记忆读取殿：任务前自动读取相关格位（25号 AI自给自足 Step4）
pub use 记忆读取_殿::{
    事件流_全部, 事件流_记录, 事件流_读取, 任务记忆闭环, 写入_按格位, 写入任务记忆, 地道精炼,
    完成度自评, 工具永驻摘要, 播种格位36, 查会话, 查全部记忆, 登记世界事实, 确认格位记忆, 终点归档,
    记会话, 读任务记忆, 读取_三档投影, 读取任务相关记忆, 读取任务相关记忆_持久, 读格位仓库,
    默认记忆库路径,
};

#[cfg(test)]
mod 测试 {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    /// env var 串行锁（mingling_caozuo_fu::模拟_llm::env_lock 是独立的，
    /// 本测试 mod 内的入口层 e2e 测试也共享同一类 env，确保可串行）
    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn 帮助命令零退出() {
        let r = 帮助命令.执行(&[]);
        assert_eq!(r.退出码, 0);
        assert!(r.输出.contains("洪荒"));
    }

    #[test]
    fn init命令零退出() {
        let r = Init命令.执行(&[]);
        assert_eq!(r.退出码, 0);
    }

    #[test]
    fn status命令零退出() {
        let r = Status命令.执行(&[]);
        assert_eq!(r.退出码, 0);
    }

    #[test]
    fn run缺任务参数() {
        let r = Run命令.执行(&[]);
        assert_eq!(r.退出码, 2);
    }

    #[test]
    fn run带错误参数格式() {
        let r = Run命令.执行(&["foo"]);
        assert_eq!(r.退出码, 2);
    }

    #[test]
    fn run修复typo完整流水线() {
        let r = Run命令.执行(&["--task=修复typo"]);
        assert_eq!(r.退出码, 0, "完整流水线应通过：{}", r.输出);
        assert!(r.输出.contains("[1/5] 道祖化要求"));
        assert!(r.输出.contains("[5/5] 道祖终裁"));
    }

    #[test]
    fn 跳层拒绝() {
        let r = 跑流水线_跳层();
        assert_eq!(r.退出码, 0);
        assert!(r.输出.contains("被拒"));
    }

    #[test]
    fn 反序拒绝() {
        let r = 跑流水线_反序();
        assert_eq!(r.退出码, 0);
    }

    #[test]
    fn 循环打回升级道祖终裁() {
        let r = 跑流水线_循环打回();
        assert_eq!(r.退出码, 0);
    }

    #[test]
    fn 分发空参数返回帮助() {
        let r = 分发(&[]);
        assert_eq!(r.退出码, 0);
    }

    #[test]
    fn 分发init() {
        let r = 分发(&["init"]);
        assert_eq!(r.退出码, 0);
    }

    #[test]
    fn 分发status() {
        let r = 分发(&["status"]);
        assert_eq!(r.退出码, 0);
    }

    #[test]
    fn 分发未知命令() {
        let r = 分发(&["foo"]);
        assert_eq!(r.退出码, 1);
    }

    #[test]
    fn 四分类到四阶段映射正确() {
        use liushuixian_qudong_fu::{分类, 分类_默认阶段};
        assert_eq!(
            分类_默认阶段(分类::道祖级),
            liushuixian_qudong_fu::阶段::道祖
        );
        assert_eq!(
            分类_默认阶段(分类::圣人级),
            liushuixian_qudong_fu::阶段::圣人
        );
        assert_eq!(
            分类_默认阶段(分类::准圣级),
            liushuixian_qudong_fu::阶段::准圣
        );
        assert_eq!(
            分类_默认阶段(分类::大罗金仙级),
            liushuixian_qudong_fu::阶段::大罗
        );
    }

    #[test]
    fn e2e_mock_llm_4分类调用() {
        let _g = env_lock();
        std::env::remove_var("LLM_BACKEND");
        let r = 跑流水线_mock_llm("e2e-test-001");
        assert_eq!(r.退出码, 0, "e2e 跑通：{}", r.输出);
        assert!(r.输出.contains("[e2e 启动]"));
        assert!(r.输出.contains("[e2e]") || r.输出.contains("[e2e 启动]"));
        assert!(r.输出.contains("[LLM 道祖]"));
        assert!(r.输出.contains("[LLM 圣人]"));
        assert!(r.输出.contains("[LLM 准圣]"));
        assert!(r.输出.contains("[LLM 大罗]"));
        assert!(r.输出.contains("[完成]"));
    }

    #[test]
    fn e2e_mock_llm_分发命令() {
        let _g = env_lock();
        std::env::remove_var("LLM_BACKEND");
        let r = 分发(&["e2e"]);
        assert_eq!(r.退出码, 0);
        assert!(r.输出.contains("[完成]"));
    }

    #[test]
    fn e2e_mock_llm_任务标识传递() {
        let _g = env_lock();
        std::env::remove_var("LLM_BACKEND");
        let r = 跑流水线_mock_llm("测试传递");
        assert!(r.输出.contains("测试传递"));
    }
}
