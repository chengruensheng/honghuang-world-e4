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

#[path = "任务材料-殿/模块.rs"]
pub mod 任务材料_殿;

#[path = "自举执行-殿/模块.rs"]
pub mod 自举执行_殿;

pub use 命令元数据_殿::命令_清单_阁::命令清单_园::{
    命令清单, 命令清单_vec
};
pub use 命令元数据_殿::帮助_生成_阁::帮助文本_园::帮助文本;
pub use 命令元数据_殿::版本_查询_阁::版本标识_园::{版本, 项目};

pub use 命令调度_殿::{
    分发, 命令, 命令结果, 帮助命令, 自举命令, 自检命令, 读任务列表文件, 读任务单文件, 跑单自举,
    跑批量自举, 跑流水线, 跑流水线_mock_llm, 跑流水线_反序, 跑流水线_循环打回, 跑流水线_真实_llm,
    跑流水线_自举, 跑流水线_跳层, 退出码, Init命令, MockLLM连接, Run命令, Status命令,
};

// 记忆读取殿：任务前自动读取相关格位（25号 AI自给自足 Step4）
pub use 记忆读取_殿::{
    世界快照, 事件流_全部, 事件流_记录, 事件流_读取, 任务收尾, 任务记忆闭环, 写入_按格位,
    写入任务记忆, 地道精炼, 完成度自评, 工具永驻摘要, 播种格位36, 查会话, 查全部记忆, 格位统计,
    状态报告, 登记世界事实, 确认格位记忆, 终点归档, 记会话, 读任务记忆, 读取_三档投影,
    读取任务相关记忆, 读取任务相关记忆_持久, 读格位仓库, 默认记忆库路径,
};

// 任务材料殿：真实任务材料结构化建模 + 模板生成 + 写入格位（架构重塑，消除 example 重复）
pub use 任务材料_殿::{
    任务材料, 写入任务材料, 材料模板_开发任务, 材料模板_验证任务
};

// 自举执行殿：确定性执行器（代码提取 + 落盘 + cargo 验证）
pub use 自举执行_殿::{提取代码块, 提取目标文件, 自举执行, 自举执行结果};

#[cfg(test)]
mod 测试 {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    /// env var 串行锁（mingling_caozuo_fu::模拟_llm::环境锁 是独立的，
    /// 本测试 mod 内的入口层 e2e 测试也共享同一类 env，确保可串行）
    fn 环境锁() -> std::sync::MutexGuard<'static, ()> {
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
        let _g = 环境锁();
        std::env::set_var("LLM_BACKEND", "mock");
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
        let _g = 环境锁();
        std::env::set_var("LLM_BACKEND", "mock");
        let r = 分发(&["e2e"]);
        assert_eq!(r.退出码, 0);
        assert!(r.输出.contains("[完成]"));
    }

    #[test]
    fn e2e_mock_llm_任务标识传递() {
        let _g = 环境锁();
        std::env::set_var("LLM_BACKEND", "mock");
        let r = 跑流水线_mock_llm("测试传递");
        assert!(r.输出.contains("测试传递"));
    }

    /// 记忆子命令缺参数边界：触库前即失败（退出码 1），杜绝 panic 与默认库副作用
    #[test]
    fn 记忆子命令缺参数返回失败() {
        let 矩阵: &[&[&str]] = &[
            &["记忆", "精炼"],
            &["记忆", "会话"],
            &["记忆", "会话", "记"],
            &["记忆", "会话", "记", "任务X"],
            &["记忆", "会话", "查"],
            &["记忆", "会话", "归档", "任务X"],
            &["记忆", "事件"],
            &["记忆", "事件", "记录"],
            &["记忆", "完成度", "4"],
            &["记忆", "收尾", "任务X"],
            &["记忆", "世界快照"],
            &["记忆", "世界快照", "337"],
            &["记忆", "未知子命令"],
        ];
        for 参数 in 矩阵 {
            let r = 分发(参数);
            assert_eq!(r.退出码, 1, "缺参数应失败码 1：{:?} → {}", 参数, r.输出);
        }
    }

    #[test]
    fn 帮助与兜底含全部子命令() {
        let 帮助 = 分发(&["帮助"]);
        for 子 in [
            "三档",
            "查",
            "播种",
            "精炼",
            "会话",
            "事件",
            "完成度",
            "统计",
            "世界快照",
            "收尾",
        ] {
            assert!(帮助.输出.contains(子), "帮助应含 {}：{}", 子, 帮助.输出);
        }
    }

    /// 会话 记：分发器成功路径（临时库）+ 查回闭环
    #[test]
    fn 会话记分发闭环() {
        let 库 = std::env::temp_dir().join("分发会话记-闭环.sq3");
        let 库 = 库.to_str().unwrap();
        let r = 分发(&["记忆", "会话", "记", "分发闭环任务", "首行；次行；三行", 库]);
        assert_eq!(r.退出码, 0, "会话记应成功：{}", r.输出);
        assert!(r.输出.contains("记录 3 行"), "应记 3 行：{}", r.输出);
        let q = 分发(&["记忆", "会话", "查", "分发闭环任务", 库]);
        assert_eq!(q.退出码, 0);
        assert!(q.输出.contains("[3]"), "应含第三行：{}", q.输出);
    }
}
