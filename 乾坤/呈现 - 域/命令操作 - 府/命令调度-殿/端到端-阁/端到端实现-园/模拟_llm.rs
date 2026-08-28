//! 端到端实现-园 - 后端选择端到端（v4 阶段 17 + Round 9）
//!
//! 决策锚：260827-moxing_fu调用方集成（Round 9）
//! 关联文档：22-moxing_fu调用方集成-实施方案.md § 七、回归保证
//! falsifiable：
//!   - 跑流水线_mock_llm 默认走 MockLLM连接（向后兼容）
//!   - 跑流水线_真实_llm 走 moxing_fu::HTTP连接（无 key 时降级 mock）

use super::super::super::命令结果;
use super::super::super::真实_后端_选择_殿::{
    后端模式, 解析后端模式, 读端点配置
};

/// 端到端后端选择（v4 阶段 17 + Round 9）
///
/// 根据 LLM_BACKEND 环境变量选择后端：
/// - "real" + 有 LLM_API_KEY → moxing_fu::HTTP连接
/// - 其他 → MockLLM连接（默认 + 降级）
pub fn 跑流水线_mock_llm(任务标识: &str) -> 命令结果 {
    run_pipeline_with_backend(任务标识, 解析后端模式())
}

/// 端到端后端选择（指定 backend）
pub fn 跑流水线_真实_llm(任务标识: &str) -> 命令结果 {
    run_pipeline_with_backend(任务标识, 后端模式::真实)
}

/// 抽象连接枚举（绕开 LLM调用器<C> 单态化限制）
enum 连接抽象 {
    Mock(MockLLM连接),
    HTTP(moxing_fu::HTTP连接),
}

impl moxing_fu::模型连接 for 连接抽象 {
    fn 发送(
        &self,
        配置: &moxing_fu::LLM配置,
        请求: &moxing_fu::请求,
    ) -> Result<moxing_fu::响应, moxing_fu::错误> {
        match self {
            连接抽象::Mock(c) => c.发送(配置, 请求),
            连接抽象::HTTP(c) => c.发送(配置, 请求),
        }
    }
}

fn run_pipeline_with_backend(任务标识: &str, 模式: 后端模式) -> 命令结果 {
    use moxing_fu::{请求, LLM调用器};
    use renwu_zhixing_fu::{任务, 分类_机械判定, 角色分类};

    let mut 日志 = format!("[e2e 启动] 任务：{} 后端={:?}\n", 任务标识, 模式);

    // Round 9：根据后端模式选择池 + 连接（用枚举 + 模型连接 trait 兼容）
    let 调用器: LLM调用器<连接抽象> = match 模式 {
        后端模式::真实 => {
            // 真实模式：尝试 moxing_fu::从环境变量构造()；失败 → 降级 mock
            match moxing_fu::从环境变量构造() {
                Some(池) => {
                    let 配置 = 读端点配置();
                    日志.push_str(&format!(
                        "[真实模式] 端点={} 超时={}ms 模型={}\n",
                        配置.端点, 配置.超时毫秒, 配置.模型
                    ));
                    LLM调用器::新建(池, 连接抽象::HTTP(moxing_fu::HTTP连接::新建()))
                }
                None => {
                    日志.push_str("[降级] 真实模式无可用 API key → fallback mock\n");
                    let 池 = build_mock_pool();
                    LLM调用器::新建(池, 连接抽象::Mock(MockLLM连接::新建()))
                }
            }
        }
        后端模式::Mock | 后端模式::默认 => {
            let 池 = build_mock_pool();
            LLM调用器::新建(池, 连接抽象::Mock(MockLLM连接::新建()))
        }
    };

    let 任务_obj = 任务 {
        标识: 任务标识.to_string(),
        分类: 角色分类::道祖级,
        描述: format!("e2e 任务：{}", 任务标识),
        decided_by: "界主".to_string(),
    };
    let _ = 分类_机械判定(&任务_obj, 角色分类::道祖级);

    // 模型可见⟺已记录：流水线组装 LLM 请求前，读持久库相关记忆 + 反向断言（可审计）
    let 记忆 = match 组装记忆上下文(任务标识) {
        Ok(记) => 记,
        Err(错) => return 命令结果::失败(4, 错),
    };
    let 记忆文本 = 记忆.join("\n");

    let 池顺序 = ["道祖", "圣人", "准圣", "大罗"];
    let mut llm失败数 = 0;
    for 池名 in 池顺序.iter() {
        let 消息列表 = 组装消息列表(池名, 任务标识, &记忆文本);
        // 正向断言（拦截点：调用器.调用 之前）：读到的每条记忆必须出现在消息列表
        if let Err(错) = 断言注入到位(&记忆, &消息列表) {
            return 命令结果::失败(4, 错);
        }
        let req = 请求::新建("", 消息列表);
        match 调用器.调用(池名, &req) {
            Ok(响应) => 日志.push_str(&format!("[LLM {}] {}\n", 池名, 响应.内容)),
            Err(e) => {
                llm失败数 += 1;
                日志.push_str(&format!("[LLM {} 错误] {}\n", 池名, e));
            }
        }
    }

    日志.push_str(&format!(
        "[完成] e2e 任务全链路（追问 + 4 分类 LLM）LLM 失败数={}\n",
        llm失败数
    ));
    if llm失败数 > 0 {
        // 真实 LLM 故障 fail loud：任一角色调用失败，流水线不得假装成功（frozen outcome）
        命令结果::失败(4, 日志)
    } else {
        命令结果::成功(日志)
    }
}

/// 组装记忆上下文：读持久库相关记忆 + 反向断言（模型可见⟺已记录）
///
/// 机制：任务进来 → 读任务记忆（持久 SQLite 库）→ 反向断言每条注入记忆可被持久库重建
/// → 返回记忆条目列表（调用方拼接注入消息列表）。
/// falsifiable：临时库写已知条目后，组装记忆上下文_按路径 返回含该条目文本；
///             反向断言失败路径（库中不存在的内容）返回 Err。
fn 组装记忆上下文(任务标识: &str) -> Result<Vec<String>, String> {
    组装记忆上下文_按路径(crate::默认记忆库路径, 任务标识)
}

/// 按路径组装记忆上下文（可测试变体，避免污染默认库）
fn 组装记忆上下文_按路径(
    记忆库路径: &str,
    任务标识: &str,
) -> Result<Vec<String>, String> {
    let 记忆 = crate::读任务记忆(记忆库路径, 任务标识);
    let 全部 = crate::查全部记忆(记忆库路径);
    断言可重建(&记忆, &全部)?;
    Ok(记忆)
}

/// 组装单个池的消息列表：记忆上下文（系统，头部）→ 角色卡（系统）→ 任务（用户）
fn 组装消息列表(
    池名: &str, 任务标识: &str, 记忆文本: &str
) -> Vec<moxing_fu::消息> {
    let mut 列表 = vec![moxing_fu::消息::系统(format!("你是 {} 角色卡", 池名))];
    if !记忆文本.is_empty() {
        列表.insert(
            0,
            moxing_fu::消息::系统(format!("相关记忆：\n{}", 记忆文本)),
        );
    }
    列表.push(moxing_fu::消息::用户(format!("任务：{}", 任务标识)));
    列表
}

/// 正向可审计断言：读到的每条记忆必须出现在消息列表（注入确实发生，可见⟺已记录）
fn 断言注入到位(记忆: &[String], 消息列表: &[moxing_fu::消息]) -> Result<(), String> {
    for 条 in 记忆 {
        let 内容 = 条.split_once("] ").map(|(_, 内)| 内).unwrap_or(条.as_str());
        if !消息列表.iter().any(|m| m.内容.contains(内容)) {
            return Err(format!("正向断言失败：读到的记忆未注入消息列表：{}", 条));
        }
    }
    Ok(())
}

/// 反向可审计断言：凡注入 LLM 的记忆必可由持久库重建（可见⟺已记录）
///
/// 读任务记忆 返回 "[范畴] 内容"，查全部记忆 返回 "[范畴·阶段] 内容"；
/// 按 "内容" 部分做子串匹配（不同范畴/阶段的同内容条目视为可重建）。
fn 断言可重建(注入: &[String], 全部: &[String]) -> Result<(), String> {
    for 条 in 注入 {
        let 内容 = 条.split_once("] ").map(|(_, 内)| 内).unwrap_or(条.as_str());
        if !全部.iter().any(|全| 全.contains(内容)) {
            return Err(format!("注入记忆不可审计（持久库不可重建）：{}", 条));
        }
    }
    Ok(())
}

/// 构造 Mock 4 分类 LLM 池
fn build_mock_pool() -> moxing_fu::LLM池 {
    use moxing_fu::{LLM池, LLM配置};
    let mut 池 = LLM池::新建();
    let mock配置 = LLM配置::假配置("mock-model");
    池.设("道祖", mock配置.clone()).unwrap();
    池.设("圣人", mock配置.clone()).unwrap();
    池.设("准圣", mock配置.clone()).unwrap();
    池.设("大罗", mock配置).unwrap();
    池
}

pub struct MockLLM连接 {
    pub 响应内容: String,
    /// 最近一次收到的请求（审计捕获：模型可见⟺已记录 正向断言的可测试支撑）
    pub 最近请求: std::sync::Mutex<Option<moxing_fu::请求>>,
}
impl MockLLM连接 {
    pub fn 新建() -> Self {
        Self {
            响应内容: "[mock LLM 响应]".to_string(),
            最近请求: std::sync::Mutex::new(None),
        }
    }
}
impl moxing_fu::模型连接 for MockLLM连接 {
    fn 发送(
        &self,
        _配置: &moxing_fu::LLM配置,
        请求: &moxing_fu::请求,
    ) -> Result<moxing_fu::响应, moxing_fu::错误> {
        // 审计捕获：请求存入字段（最近请求）+ thread_local 槽（供测试断言消息列表含注入记忆）
        if let Ok(mut 槽) = self.最近请求.lock() {
            *槽 = Some(请求.clone());
        }
        记录最近请求(请求);
        Ok(moxing_fu::响应::假响应(&self.响应内容))
    }
}

// 审计捕获槽：MockLLM连接::发送 把最近请求写入 thread_local，
// 供单元测试断言「模型可见⟺已记录」（注入记忆必出现在发往 LLM 的真实请求消息列表）。
thread_local! {
    static 最近请求槽: std::cell::RefCell<Vec<moxing_fu::请求>> = const { std::cell::RefCell::new(Vec::new()) };
}

fn 记录最近请求(请求: &moxing_fu::请求) {
    最近请求槽.with(|槽| 槽.borrow_mut().push(请求.clone()));
}

#[cfg(test)]
mod 测试 {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|e| e.into_inner())
    }

    /// 清理所有 LLM 相关 env（避免测试间污染）
    fn 清空_env() {
        std::env::remove_var("LLM_BACKEND");
        std::env::remove_var("LLM_API_KEY");
        std::env::remove_var("LLM_BASE_URL");
        std::env::remove_var("LLM_MODEL");
        std::env::remove_var("LLM_TIMEOUT_MS");
        std::env::remove_var("LLM_MODEL_DAOZU");
        std::env::remove_var("LLM_MODEL_SHENGREN");
        std::env::remove_var("LLM_MODEL_ZHUNSHENG");
        std::env::remove_var("LLM_MODEL_DALUO");
    }

    #[test]
    fn 断言可重建_正向通过() {
        let 注入 = vec!["[程序] 36 格位闭环 API".to_string()];
        let 全部 = vec!["[程序·实施] 36 格位闭环 API".to_string()];
        assert!(断言可重建(&注入, &全部).is_ok());
    }

    #[test]
    fn 断言可重建_反向失败() {
        let 注入 = vec!["[程序] 库中不存在的记忆".to_string()];
        let 全部 = vec!["[程序·实施] 36 格位闭环 API".to_string()];
        match 断言可重建(&注入, &全部) {
            Err(错) => assert!(错.contains("不可审计"), "错误信息应含「不可审计」：{}", 错),
            Ok(_) => panic!("库中不存在的内容必须判定不可审计"),
        }
    }

    #[test]
    fn 组装记忆上下文_已知条目可重建() {
        let 路径 = std::env::temp_dir().join(format!("洪荒记忆测试_{}.sq3", std::process::id()));
        let 路径_str = 路径.to_str().unwrap();
        // 空库自动种子落盘（含 程序/实施 "36 格位闭环 API"）
        let _ = crate::读取任务相关记忆_持久(路径_str, "实现 Cargo 测试");
        let 记忆 =
            组装记忆上下文_按路径(路径_str, "实现 Cargo 测试").expect("注入记忆必须可被持久库重建");
        assert!(
            记忆.iter().any(|m| m.contains("程序")),
            "应命中程序范畴记忆：{:?}",
            记忆
        );
        let _ = std::fs::remove_file(&路径);
    }

    #[test]
    fn 组装消息列表_记忆注入头部() {
        let 记忆文本 = "[程序] 36 格位闭环 API\n[规则] 命名门禁规则";
        let 列表 = 组装消息列表("道祖", "实现 Cargo 测试", 记忆文本);
        // 首条为记忆系统消息
        assert!(matches!(列表[0].角色, moxing_fu::角色::系统));
        assert!(
            列表[0].内容.contains("36 格位闭环 API"),
            "首条应含记忆：{}",
            列表[0].内容
        );
        assert!(列表[0].内容.contains("命名门禁规则"));
        // 末条为用户任务
        let 用户 = 列表
            .iter()
            .find(|m| matches!(m.角色, moxing_fu::角色::用户))
            .expect("应有用户消息");
        assert!(用户.内容.contains("实现 Cargo 测试"));
    }

    #[test]
    fn 断言注入到位_正向通过() {
        let 记忆 = vec!["[程序] 36 格位闭环 API".to_string()];
        let 列表 = 组装消息列表("道祖", "任务", "[程序] 36 格位闭环 API");
        assert!(断言注入到位(&记忆, &列表).is_ok());
    }

    #[test]
    fn 断言注入到位_缺失记忆失败() {
        let 记忆 = vec!["[程序] 36 格位闭环 API".to_string()];
        // 消息列表不含该记忆（仅角色卡）→ 正向断言应失败
        let 列表 = vec![moxing_fu::消息::系统("你是 道祖 角色卡".to_string())];
        match 断言注入到位(&记忆, &列表) {
            Err(错) => assert!(错.contains("未注入"), "错误信息应含「未注入」：{}", 错),
            Ok(_) => panic!("缺失记忆必须判定正向断言失败"),
        }
    }

    #[test]
    fn 测试_流水线_请求消息列表_含注入记忆() {
        let _g = env_lock();
        清空_env();
        最近请求槽.with(|槽| 槽.borrow_mut().clear());
        let r = 跑流水线_mock_llm("记忆注入审计测试");
        assert_eq!(r.退出码, 0, "流水线应成功：{}", r.输出);
        let 请求们 = 最近请求槽.with(|槽| 槽.borrow().clone());
        assert!(!请求们.is_empty(), "应捕获到发往 LLM 的请求");
        for 请求 in &请求们 {
            let 全文本 = 请求
                .消息列表
                .iter()
                .map(|m| m.内容.clone())
                .collect::<Vec<_>>()
                .join("\n");
            assert!(
                全文本.contains("36 格位闭环 API"),
                "请求消息列表应含注入记忆：{}",
                全文本
            );
        }
    }

    #[test]
    fn 测试_默认走_mock() {
        let _g = env_lock();
        清空_env();
        let r = 跑流水线_mock_llm("default-mock");
        assert_eq!(r.退出码, 0);
        assert!(r.输出.contains("后端=Mock") || r.输出.contains("后端=默认"));
        assert!(r.输出.contains("[LLM 道祖]"));
        assert!(r.输出.contains("[LLM 大罗]"));
        assert!(r.输出.contains("[完成]"));
    }

    #[test]
    fn 测试_LLM_BACKEND_mock_显式走_mock() {
        let _g = env_lock();
        清空_env();
        std::env::set_var("LLM_BACKEND", "mock");
        let r = 跑流水线_mock_llm("env-mock");
        assert_eq!(r.退出码, 0);
        assert!(r.输出.contains("后端=Mock"));
        std::env::remove_var("LLM_BACKEND");
    }

    #[test]
    fn 测试_LLM_BACKEND_real_无_key_降级_mock() {
        let _g = env_lock();
        清空_env();
        std::env::set_var("LLM_BACKEND", "real");
        // LLM_API_KEY 未设置 → 从环境变量构造() 返回 None → 降级 mock
        let r = 跑流水线_mock_llm("env-real-no-key");
        assert_eq!(r.退出码, 0);
        assert!(r.输出.contains("后端=真实"));
        assert!(r.输出.contains("[降级]"));
        assert!(r.输出.contains("fallback mock"));
        std::env::remove_var("LLM_BACKEND");
    }

    #[test]
    fn 测试_LLM_BACKEND_real_有_key_走真实() {
        let _g = env_lock();
        清空_env();
        // 先设置 key，再设置 backend，避免 race（key 在 backend 之前可见）
        std::env::set_var("LLM_API_KEY", "sk-test-fake-key");
        std::env::set_var(
            "LLM_BASE_URL",
            "https://api.test.invalid/v1/chat/completions",
        );
        std::env::set_var("LLM_BACKEND", "real");
        // 有 key → 真实模式尝试 HTTP；但 base URL 是 .invalid 会失败
        // 此测试只验证：走到「真实模式」分支（[真实模式] 行），不验证 HTTP 成功
        let r = 跑流水线_mock_llm("env-real-with-key");
        // 故障合约：真实 LLM 网络失败必须 fail loud（退出码 4），不得假装成功
        assert_eq!(r.退出码, 4, "真实 LLM 失败应 fail loud：{}", r.输出);
        // 后端=真实 总是出现
        assert!(r.输出.contains("后端=真实"));
        assert!(
            r.输出.contains("[真实模式]"),
            "有 key 应走真实模式：{}",
            r.输出
        );
        assert!(
            r.输出.contains("[LLM 道祖 错误]"),
            "HTTP 失败应记录错误：{}",
            r.输出
        );
        清空_env();
    }

    #[test]
    fn 测试_跑流水线_真实_llm_显式_无_key_降级() {
        let _g = env_lock();
        清空_env();
        // 显式走真实模式 + 无 key → 应降级
        let r = 跑流水线_真实_llm("explicit-real-no-key");
        assert_eq!(r.退出码, 0);
        assert!(r.输出.contains("后端=真实"));
        assert!(r.输出.contains("[降级]"));
    }

    #[test]
    fn 测试_跑流水线_真实_llm_显式_有_key() {
        let _g = env_lock();
        清空_env();
        std::env::set_var("LLM_API_KEY", "sk-test");
        std::env::set_var(
            "LLM_BASE_URL",
            "https://api.test.invalid/v1/chat/completions",
        );
        let r = 跑流水线_真实_llm("explicit-real-with-key");
        // 故障合约：有 key 但 HTTP 失败 → fail loud（退出码 4）
        assert_eq!(r.退出码, 4, "真实 LLM 失败应 fail loud：{}", r.输出);
        assert!(r.输出.contains("[真实模式]"));
        assert!(r.输出.contains("[LLM 道祖 错误]"));
        清空_env();
    }

    #[test]
    fn 测试_任务标识传递_真实模式() {
        let _g = env_lock();
        清空_env();
        let r = 跑流水线_真实_llm("任务标识真实模式");
        assert!(r.输出.contains("任务标识真实模式"));
    }

    #[test]
    fn 测试_任务标识传递_mock模式() {
        let _g = env_lock();
        清空_env();
        let r = 跑流水线_mock_llm("任务标识mock模式");
        assert!(r.输出.contains("任务标识mock模式"));
    }
}
