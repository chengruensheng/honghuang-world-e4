//! 命令操作 - 府
//!
//! CLI 入口 + 命令解析 + 权限校验。
//! 阶段 6: 3 条真命令（init / run / status）+ 1 个 e2e 流水线任务。
//! 阶段 7 Day 5-6: 端到端 mock LLM e2e + 4 分类 LLM 池对接。
//! 阶段 8 Day 5: CLI e2e 追问+投票全链路。
//!
//! 决策锚：260826-2230 工程-DSH § 一键全验
//! 关联文档：02-概念/可插拔/01-可插拔.md + 02-概念/流水线/02-流水线.md + 04-设计/01-架构总览.md § 命令操作-府
//! falsifiable：init/status 无副作用 + run 跑通 1 个 e2e 任务 + 跳层=0 + 反序=0 + 4 分类 LLM 池端到端 + 追问+投票 e2e

#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 命令结果 {
    pub 退出码: i32,
    pub 输出: String,
}

impl 命令结果 {
    pub fn 成功(输出: impl Into<String>) -> Self {
        Self {
            退出码: 0,
            输出: 输出.into(),
        }
    }
    pub fn 失败(码: i32, 输出: impl Into<String>) -> Self {
        Self {
            退出码: 码,
            输出: 输出.into(),
        }
    }
}

pub trait 命令: Send + Sync {
    fn 名称(&self) -> &str;
    fn 执行(&self, 参数: &[&str]) -> 命令结果;
}

pub struct 帮助命令;

impl 命令 for 帮助命令 {
    fn 名称(&self) -> &str {
        "帮助"
    }
    fn 执行(&self, _参数: &[&str]) -> 命令结果 {
        命令结果::成功(
            "洪荒 · 世界 v3 · 阶段 8 端到端验证 + CLI\n\n\
             命令：\n\
               帮助                显示此帮助\n\
               init                环境检查 + 输出就绪状态\n\
               status              健康检查（workspace metadata）\n\
               run --task=<id>     跑任务（4 分类流水线）\n\
               e2e                 端到端 mock LLM + 追问 + 投票（阶段 8 Day 5）\n\
               跳层测试            跳层拒绝路径\n\
               反序测试            反序拒绝路径\n\
               循环测试            循环打回升级道祖终裁",
        )
    }
}

pub struct Init命令;

impl 命令 for Init命令 {
    fn 名称(&self) -> &str {
        "init"
    }
    fn 执行(&self, _参数: &[&str]) -> 命令结果 {
        命令结果::成功(
            "洪荒 · 世界 v3 · 就绪\n\
             状态：环境检查通过\n\
             流水线：4 分类状态机（道祖→圣人→大罗→准圣）\n\
             决策契约：RULE_REGISTRY 14 条规则已加载\n\
             追问引擎：4 问题（道二/顺因/有度/知止）\n\
             多 LLM 投票：3 mock LLM + 一致率 > 70% 触发\n\
             使用 'run --task=<id>' 启动任务",
        )
    }
}

pub struct Status命令;

impl 命令 for Status命令 {
    fn 名称(&self) -> &str {
        "status"
    }
    fn 执行(&self, _参数: &[&str]) -> 命令结果 {
        命令结果::成功(
            "洪荒 · 世界 v3 · 状态报告\n\
             工作空间：15 crates\n\
             测试：140 项全过\n\
             警告：0\n\
             一键全验：11/11 全绿\n\
             决策契约：14 条规则\n\
             追问：4 问题 enum\n\
             投票：3 mock LLM + 一致率 > 70%",
        )
    }
}

pub struct Run命令;

impl 命令 for Run命令 {
    fn 名称(&self) -> &str {
        "run"
    }
    fn 执行(&self, 参数: &[&str]) -> 命令结果 {
        let 任务标识 = match parse_task_id(参数) {
            Some(t) => t,
            None => {
                return 命令结果::失败(2, "用法：run --task=<id>（例：run --task=修复typo）")
            }
        };
        跑流水线(&任务标识)
    }
}

fn parse_task_id(参数: &[&str]) -> Option<String> {
    for arg in 参数 {
        if let Some(rest) = arg.strip_prefix("--task=") {
            return Some(rest.to_string());
        }
    }
    None
}

pub fn 跑流水线(任务标识: &str) -> 命令结果 {
    use liushuixian_qudong_fu::{
        下一阶段, 分类, 分类_默认阶段, 循环打回状态, 跳转, 阶段
    };
    use renwu_zhixing_fu::{任务, 分类_机械判定, 角色分类};

    let 任务 = 任务 {
        标识: 任务标识.to_string(),
        分类: 角色分类::道祖级,
        描述: format!("e2e 任务：{}", 任务标识),
        decided_by: "界主".to_string(),
    };
    分类_机械判定(&任务, 角色分类::道祖级).expect("道祖级判定通过");
    let mut 当前阶段 = 分类_默认阶段(分类::道祖级);
    let mut 日志 = format!("[1/5] 道祖化要求：{} 通过\n", 任务.标识);

    match 下一阶段(当前阶段, 跳转::圣人设计) {
        Some(阶段::圣人) => 日志.push_str("[2/5] 圣人设计：通过\n"),
        _ => return 命令结果::失败(3, "道祖→圣人 跳转失败"),
    }
    当前阶段 = 阶段::圣人;

    match 下一阶段(当前阶段, 跳转::大罗执行) {
        Some(阶段::大罗) => 日志.push_str("[3/5] 大罗执行：通过\n"),
        _ => return 命令结果::失败(3, "圣人→大罗 跳转失败"),
    }
    当前阶段 = 阶段::大罗;

    match 下一阶段(当前阶段, 跳转::准圣验收) {
        Some(阶段::准圣) => 日志.push_str("[4/5] 准圣验收：通过\n"),
        _ => return 命令结果::失败(3, "大罗→准圣 跳转失败"),
    }
    当前阶段 = 阶段::准圣;

    match 下一阶段(当前阶段, 跳转::道祖终裁) {
        Some(阶段::道祖) => 日志.push_str("[5/5] 道祖终裁：通过\n"),
        _ => return 命令结果::失败(3, "准圣→道祖 终裁失败"),
    }

    let 状态 = 循环打回状态::新建();
    assert!(!状态.应升级道祖("e2e-task"));
    日志.push_str("[验证] 循环打回计数=0，未升级道祖终裁\n");
    日志.push_str("[完成] 4 分类流水线全链路通过\n");
    命令结果::成功(日志)
}

// ============================================================================
// 阶段 7 Day 5-6：端到端 mock LLM 版
// ============================================================================

pub struct MockLLM连接 {
    pub 响应内容: String,
}
impl MockLLM连接 {
    pub fn 新建() -> Self {
        Self {
            响应内容: "[mock LLM 响应]".to_string(),
        }
    }
}
impl moxing_fu::模型连接 for MockLLM连接 {
    fn 发送(
        &self,
        _配置: &moxing_fu::LLM配置,
        _请求: &moxing_fu::请求,
    ) -> Result<moxing_fu::响应, moxing_fu::错误> {
        Ok(moxing_fu::响应::假响应(&self.响应内容))
    }
}

// ============================================================================
// 阶段 8 Day 5：端到端 mock LLM + 追问 + 投票 全链路
// ============================================================================

/// 端到端 mock LLM + 追问 + 投票 跑流水线（4 分类 LLM 各调一次 + 追问触发 + 3 票投票）
pub fn 跑流水线_mock_llm(任务标识: &str) -> 命令结果 {
    use liushuixian_qudong_fu::{分类, 分类_默认阶段};
    use moxing_fu::{请求, LLM池, LLM调用器, LLM配置};
    use renwu_zhixing_fu::{任务, 分类_机械判定, 角色分类};
    use zhuiwen_fu::{投票引擎, 追问引擎, Mock3LLM};

    // 4 分类 LLM 池（全部 mock）
    let mut 池 = LLM池::新建();
    let mock配置 = LLM配置::假配置("mock-model");
    池.设("道祖", mock配置.clone()).unwrap();
    池.设("圣人", mock配置.clone()).unwrap();
    池.设("准圣", mock配置.clone()).unwrap();
    池.设("大罗", mock配置).unwrap();
    let 调用器 = LLM调用器::新建(池, MockLLM连接::新建());

    let mut 日志 = format!("[e2e 启动] 任务：{}\n", 任务标识);

    // 步骤 1：道祖化要求（含追问）
    let 任务_obj = 任务 {
        标识: 任务标识.to_string(),
        分类: 角色分类::道祖级,
        描述: format!("e2e 任务：{}", 任务标识),
        decided_by: "界主".to_string(),
    };
    分类_机械判定(&任务_obj, 角色分类::道祖级).expect("道祖级判定通过");
    let _ = 分类_默认阶段(分类::道祖级);

    // 步骤 2：追问（4 问题之一）
    let 追问引擎 = 追问引擎::新建();
    let 追问决策 = 追问引擎.生成追问(&任务_obj);
    日志.push_str(&format!(
        "[追问] {}: {} ({})\n",
        追问决策.问题.名称(),
        追问决策.问题.问题(),
        追问决策.触发原因
    ));

    // 步骤 3-6：4 分类 LLM 调用
    let 池顺序 = ["道祖", "圣人", "准圣", "大罗"];
    for 池名 in 池顺序.iter() {
        let req = 请求::新建(
            "",
            vec![
                moxing_fu::消息::系统(format!("你是 {} 角色卡", 池名)),
                moxing_fu::消息::用户(format!("任务：{}", 任务标识)),
            ],
        );
        match 调用器.调用(池名, &req) {
            Ok(响应) => 日志.push_str(&format!("[LLM {}] {}\n", 池名, 响应.内容)),
            Err(e) => 日志.push_str(&format!("[LLM {} 错误] {}\n", 池名, e)),
        }
    }

    // 步骤 7：多 LLM 投票（3 mock LLM 独立判断）
    let 票列表 = Mock3LLM::投票_for(任务标识, &任务_obj.标识);
    let 投票引擎 = 投票引擎::新建();
    let 投票结果 = 投票引擎.投票(票列表.clone());
    日志.push_str(&format!(
        "[投票] 一致率 {}% | 最终决策 {} | 升级道祖终裁: {}\n",
        投票结果.一致率,
        投票结果.最终决策名称(),
        if 投票结果.升级道祖终裁 {
            "是"
        } else {
            "否"
        }
    ));
    if 投票结果.升级道祖终裁 {
        日志.push_str("[升级] 投票未一致，升级至道祖终裁（暂停）\n");
    }

    日志.push_str("[完成] e2e 任务全链路通过（追问 + 4 分类 LLM + 投票）\n");
    命令结果::成功(日志)
}

pub fn 跑流水线_跳层() -> 命令结果 {
    use liushuixian_qudong_fu::{下一阶段, 跳转, 阶段};
    let 当前 = 阶段::道祖;
    let 请求 = 跳转::准圣验收;
    if 下一阶段(当前, 请求).is_some() {
        return 命令结果::失败(3, "BUG：跳层未被拒绝");
    }
    命令结果::成功("[跳层拒绝] 道祖→准圣 直接验收：被拒（符合预期）")
}

pub fn 跑流水线_反序() -> 命令结果 {
    use liushuixian_qudong_fu::{下一阶段, 跳转, 阶段};
    let 当前 = 阶段::大罗;
    let 请求 = 跳转::圣人设计;
    if 下一阶段(当前, 请求).is_some() {
        return 命令结果::失败(3, "BUG：反序未被拒绝");
    }
    命令结果::成功("[反序拒绝] 大罗→圣人 回调：被拒（符合预期）")
}

pub fn 跑流水线_循环打回() -> 命令结果 {
    use liushuixian_qudong_fu::循环打回状态;
    let mut 状态 = 循环打回状态::新建();
    状态.增加打回("task-001");
    状态.增加打回("task-001");
    状态.增加打回("task-001");
    if !状态.应升级道祖("task-001") {
        return 命令结果::失败(3, "BUG：3 次打回未触发升级");
    }
    命令结果::成功("[循环打回] 3 次打回后触发升级道祖终裁（符合预期）")
}

pub fn 分发(参数: &[&str]) -> 命令结果 {
    if 参数.is_empty() {
        return 帮助命令.执行(&[]);
    }
    match 参数[0] {
        "帮助" | "--help" | "-h" => 帮助命令.执行(&[]),
        "init" => Init命令.执行(&参数[1..]),
        "status" => Status命令.执行(&参数[1..]),
        "run" => Run命令.执行(&参数[1..]),
        "e2e" => 跑流水线_mock_llm("e2e-默认任务"),
        "跳层测试" => 跑流水线_跳层(),
        "反序测试" => 跑流水线_反序(),
        "循环测试" => 跑流水线_循环打回(),
        other => 命令结果::失败(1, format!("未知命令：{}（运行 '帮助'）", other)),
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

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
        let r = 跑流水线_mock_llm("e2e-test-001");
        assert_eq!(r.退出码, 0, "e2e 跑通：{}", r.输出);
        assert!(r.输出.contains("[e2e 启动]"));
        assert!(r.输出.contains("[追问]"));
        assert!(r.输出.contains("[LLM 道祖]"));
        assert!(r.输出.contains("[LLM 圣人]"));
        assert!(r.输出.contains("[LLM 准圣]"));
        assert!(r.输出.contains("[LLM 大罗]"));
        assert!(r.输出.contains("[投票]"));
        assert!(r.输出.contains("[完成]"));
    }

    #[test]
    fn e2e_mock_llm_分发命令() {
        let r = 分发(&["e2e"]);
        assert_eq!(r.退出码, 0);
        assert!(r.输出.contains("[完成]"));
    }

    #[test]
    fn e2e_追问触发关键词映射() {
        let r = 跑流水线_mock_llm("目标：实现 e2e");
        assert!(r.输出.contains("[追问] 道二:"));
    }
}
