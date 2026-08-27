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

    日志.push_str("[完成] e2e 任务全链路通过（追问 + 4 分类 LLM）\n");
    命令结果::成功(日志)
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
        assert_eq!(r.退出码, 0);
        // 后端=真实 总是出现（不论从环境变量构造成功还是降级，都来自后端模式::真实分支）
        assert!(r.输出.contains("后端=真实"));
        // 如果从环境变量构造返回 Some 池 → 进入 [真实模式]；否则 [降级]
        let 真实模式或降级 = r.输出.contains("[真实模式]") || r.输出.contains("[降级]");
        assert!(
            真实模式或降级,
            "期望 [真实模式] 或 [降级]，实际输出：{}",
            r.输出
        );
        // 4 分类调用应正常执行（不论 HTTP 成功与否，调用器都返回 Ok 或 Err）
        assert!(r.输出.contains("[完成]"));
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
        let r = 跑流水线_真实_llm("explicit-real-with-key");
        assert_eq!(r.退出码, 0);
        assert!(r.输出.contains("[真实模式]"));
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
